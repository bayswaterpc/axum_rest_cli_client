use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

impl Pagination {
    pub fn make_query_uri(&self, mut get_url: String) -> String {
        if self.limit.is_some() || self.offset.is_some() {
            get_url.push('?');
            if let Some(limit) = self.limit {
                get_url = format!("{}limit={}", get_url, limit);
            }
            if !get_url.ends_with('?') {
                get_url.push('&');
            }
            if let Some(offset) = self.offset {
                get_url = format!("{}offset={}", get_url, offset);
            }
        }
        get_url
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTodo {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodo {
    pub text: Option<String>,
    pub completed: Option<bool>,
}

impl UpdateTodo {
    pub fn make_query_uri(&self, mut get_url: String) -> String {
        if self.text.is_some() || self.completed.is_some() {
            get_url.push('?');
            if let Some(text) = self.text.clone() {
                get_url = format!("{}text={}", get_url, text);
            }
            if !get_url.ends_with('?') {
                get_url.push('&');
            }
            if let Some(completed) = self.completed {
                get_url = format!("{}completed={}", get_url, completed);
            }
        }
        get_url
    }
}

pub static ROOT: &str = "http://localhost:3000/todos";
pub static TODOS_PATH: &str = "http://localhost:3000/todos";
