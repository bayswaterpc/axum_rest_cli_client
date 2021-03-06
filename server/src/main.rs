//! Provides a RESTful web server managing some Todos.
//!
//! API will be:
//!
//! - `GET /todos`: return a JSON list of Todos.
//! - `POST /todos`: create a new Todo.
//! - `PUT /todos`: create a new Todo with specified id & completed status
//! - `PATCH /todos/:id`: update a specific Todo.
//! - `DELETE /todos/:id`: delete a specific Todo.
//!
//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-todos
//! ```

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};
use common::{Pagination, PostTodo, Todo, UpdateTodo};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::Duration,
};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_todos=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = Db::default();

    // Compose the routes
    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(todos_index).post(todos_create).put(todos_put))
        .route("/todos/:id", patch(todos_update).delete(todos_delete))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(Extension(db))
                .into_inner(),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// #[derive(Debug, Deserialize, Default)]
// pub struct Pagination {
//     pub offset: Option<usize>,
//     pub limit: Option<usize>,
// }

/**
Can access on the browser at
```not_rust
http://localhost:3000/todos
```
with options
```not_rust
http://localhost:3000/todos?offset=1&limit=1
```

or with run with curl
```not_rust
curl -X GET "http://localhost:3000/todos?offset=1&limit=1"
```
**/
async fn todos_index(
    pagination: Option<Query<Pagination>>,
    Extension(db): Extension<Db>,
) -> impl IntoResponse {
    tracing::debug!("getting");
    let todos = db.read().unwrap();

    let Query(pagination) = pagination.unwrap_or_default();

    let todos = todos
        .values()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect::<Vec<_>>();

    Json(todos)
}

async fn root() -> &'static str {
    "Hello, World!"
}

/**
Run with curl
```not_rust
curl -X POST http://localhost:3000/todos\
   -H 'Content-Type: application/json' \
   -d '{"text":"do this"}'
```
**/
async fn todos_create(
    Json(input): Json<PostTodo>,
    Extension(db): Extension<Db>,
) -> impl IntoResponse {
    tracing::debug!("todos_create {:?}", input);
    let todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed: false,
    };

    db.write().unwrap().insert(todo.id, todo.clone());
    (StatusCode::CREATED, Json(todo))
}

/**
Run with curl
```not_rust
curl -X PUT http://localhost:3000/todos\
   -H 'Content-Type: application/json' \
   -d '{"id":"aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa","text":"i put this","completed":true}'
```
**/
async fn todos_put(Json(input): Json<Todo>, Extension(db): Extension<Db>) -> impl IntoResponse {
    tracing::debug!("todos_put {:?}", input);
    db.write().unwrap().insert(input.id, input.clone());
    (StatusCode::CREATED, Json(input))
}

/**
```not_rust
curl -X PATCH "http://localhost:3000/todos/:id" \
   -H 'Content-Type: application/json' \
   -d '{"text":"did it","completed":true}'
```
**/
async fn todos_update(
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTodo>,
    Extension(db): Extension<Db>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::debug!("parsing {:?}", input);
    let mut todo = db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(text) = input.text {
        todo.text = text;
    }

    if let Some(completed) = input.completed {
        todo.completed = completed;
    }

    db.write().unwrap().insert(todo.id, todo.clone());

    Ok(Json(todo))
}

/**
Run with curl
```not_rust
curl -X "DELETE" http://localhost:3000/todos/:id
```
**/
async fn todos_delete(Path(id): Path<Uuid>, Extension(db): Extension<Db>) -> impl IntoResponse {
    if db.write().unwrap().remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;
