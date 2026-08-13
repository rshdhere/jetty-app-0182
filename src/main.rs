mod handlers;
mod todo;

use axum::{
    response::Html,
    routing::{get, put},
    Json, Router,
};
use handlers::{create_todo, delete_todo, list_todos, update_todo, TodoStore};
use serde_json::{json, Value};
use std::{net::SocketAddr, sync::{Arc, Mutex}};

#[tokio::main]
async fn main() {
    let store: TodoStore = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/api/todos", get(list_todos).post(create_todo))
        .route(
            "/api/todos/{id}",
            put(update_todo).delete(delete_todo),
        )
        .with_state(store);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<Value> {
    Json(json!({ "ok": true }))
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}
