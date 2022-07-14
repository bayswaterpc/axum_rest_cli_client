use common::{Pagination, PostTodo, Todo, UpdateTodo, ROOT};
use reqwest::header::CONTENT_TYPE;
use reqwest::{self, Result, StatusCode};
use uuid::Uuid;

pub async fn get_root() -> Result<String> {
    // chaining .await will yield our query result
    let resp = reqwest::get(ROOT).await?.text().await?;
    Ok(resp)
}

pub async fn post_todo(uri: &str, todo: PostTodo) -> Result<Todo> {
    let json_str = serde_json::to_string(&todo).unwrap_or_default();
    println!("{}", json_str);

    let client = reqwest::Client::new();
    let resp = client
        .post(uri)
        .header(CONTENT_TYPE, "application/json")
        .body(json_str)
        .send()
        .await?
        .text()
        .await?;
    let todo: Todo = serde_json::from_str(&resp).expect("Invalid Response");
    Ok(todo)
}

pub async fn get_todo(uri: &str, params: Pagination) -> Result<Vec<Todo>> {
    let get_uri = params.make_query_uri(uri.to_string());
    let resp = reqwest::get(get_uri).await?.text().await?;
    let todos: Vec<Todo> = serde_json::from_str(&resp).expect("Invalid Response");
    Ok(todos)
}

pub async fn put_todo(uri: &str, params: Todo) -> Result<Todo> {
    let json_str = serde_json::to_string(&params).unwrap_or_default();
    println!("{}", json_str);

    let client = reqwest::Client::new();
    let resp = client
        .put(uri)
        .header(CONTENT_TYPE, "application/json")
        .body(json_str)
        .send()
        .await?
        .text()
        .await?;

    let todo: Todo = serde_json::from_str(&resp).expect("Invalid Response");
    Ok(todo)
}

pub async fn patch_todo(uri: &str, id: &Uuid, params: UpdateTodo) -> Result<Todo> {
    let patch_uri = format!("{}/{}", uri, id);

    let json_str = serde_json::to_string(&params).unwrap_or_default();
    println!("{}", json_str);

    let client = reqwest::Client::new();
    let resp = client
        .patch(patch_uri)
        .header(CONTENT_TYPE, "application/json")
        .body(json_str)
        .send()
        .await?
        .text()
        .await?;

    let todo: Todo = serde_json::from_str(&resp).expect("Invalid Response");
    Ok(todo)
}

pub async fn delete_todo(uri: &str, id: &Uuid) -> Result<StatusCode> {
    let delete_uri = format!("{}/{}", uri, id);

    let client = reqwest::Client::new();
    let resp = client.delete(delete_uri).send().await?.status();
    Ok(resp)
}
