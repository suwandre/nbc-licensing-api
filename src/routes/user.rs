use axum::{response::IntoResponse, http::StatusCode, Router, routing::{get, post}, extract::Json};

use crate::models::User;

pub fn user_routes() -> Router {
    Router::new()
        .route("/hello-world", get(hello_world))
        .route("/create-user", post(create_user))

}

async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, "Hello, World!")
}

async fn create_user(Json(payload): Json<User>) {
    println!("User: {:?}", payload);
}