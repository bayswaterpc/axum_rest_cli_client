use common::{Pagination, PostTodo, Todo, UpdateTodo};
use reqwest::header::CONTENT_TYPE;
use reqwest::{self, Result, StatusCode};

pub async fn get_root() -> Result<String> {
    // chaining .await will yield our query result
    let resp = reqwest::get("http://localhost:3000/").await?.text().await?;
    Ok(resp)
}

pub async fn post_todo(todo: PostTodo) -> Result<Todo> {
    let json_str = serde_json::to_string(&todo).unwrap_or_default();
    println!("{}", json_str);

    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:3000/todos")
        .header(CONTENT_TYPE, "application/json")
        .body(json_str)
        .send()
        .await?
        .text()
        .await?;
    let todo: Todo = serde_json::from_str(&resp).expect("Invalid Response");
    Ok(todo)
}

pub async fn get_todo(params: Pagination) -> Result<Vec<Todo>> {
    let get_url = params.make_query_uri("http://localhost:3000/todos".to_string());
    let resp = reqwest::get(get_url).await?.text().await?;
    let todos: Vec<Todo> = serde_json::from_str(&resp).expect("Invalid Response");
    Ok(todos)
}

pub async fn patch_todo(id: &String, params: UpdateTodo) -> Result<Todo> {
    let patch_url = format!("http://localhost:3000/todos/{}", id);

    let json_str = serde_json::to_string(&params).unwrap_or_default();
    println!("{}", json_str);

    let client = reqwest::Client::new();
    let resp = client
        .patch(patch_url)
        .header(CONTENT_TYPE, "application/json")
        .body(json_str)
        .send()
        .await?
        .text()
        .await?;

    let todo: Todo = serde_json::from_str(&resp).expect("Invalid Response");
    Ok(todo)
}

pub async fn delete_todo(id: &String) -> Result<StatusCode> {
    let delete_uri = format!("http://localhost:3000/todos/{}", id);

    let client = reqwest::Client::new();
    let resp = client.delete(delete_uri).send().await?.status();

    println!("delete_todo {:?}", resp);

    Ok(resp)
}
