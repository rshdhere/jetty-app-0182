use crate::todo::{CreateTodo, Todo, UpdateTodo};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub type TodoStore = Arc<Mutex<Vec<Todo>>>;

pub async fn list_todos(State(store): State<TodoStore>) -> Json<Vec<Todo>> {
    let todos = store.lock().unwrap();
    Json(todos.clone())
}

pub async fn create_todo(
    State(store): State<TodoStore>,
    Json(body): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    let text = body.text.trim();
    if text.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        text: text.to_string(),
        completed: false,
    };

    let mut todos = store.lock().unwrap();
    todos.push(todo.clone());
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn update_todo(
    State(store): State<TodoStore>,
    Path(id): Path<String>,
    Json(body): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let mut todos = store.lock().unwrap();
    let todo = todos
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(text) = body.text {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }
        todo.text = trimmed.to_string();
    }
    if let Some(completed) = body.completed {
        todo.completed = completed;
    }

    Ok(Json(todo.clone()))
}

pub async fn delete_todo(
    State(store): State<TodoStore>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let mut todos = store.lock().unwrap();
    let len_before = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() == len_before {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}
