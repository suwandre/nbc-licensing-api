use axum::{response::IntoResponse, http::StatusCode, Router, routing::get};

pub fn user_routes() -> Router {
    Router::new()
        .route("/hello-world", get(hello_world))
}

async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, "Hello, World!")
}