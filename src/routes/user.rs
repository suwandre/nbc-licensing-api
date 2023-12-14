use axum::{response::{IntoResponse, Response}, http::StatusCode, Router, routing::{get, post}, extract::{Json, Request, Path}};
use serde_json::json;
use axum_macros::debug_handler;

use crate::{models::{User, ApiResponse, CreateUser}, api::create_user};

pub fn user_routes() -> Router {
    Router::new()
        .route("/hello-world", get(hello_world))
        .route("/create-user", post(create_user_route))
        .route("/get-user/:wallet_address", get(get_user_route))
}

async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, "Hello, World!")
}

#[debug_handler]
async fn create_user_route(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let object_ids = create_user(
        payload.wallet_address,
        payload.expiration_date,
        payload.chain_id,
        payload.domain,
        payload.nonce,
        payload.signature,
        payload.payload,
        payload.profile_id,
        payload.uri,
        payload.version
    ).await;

    let api_response = match object_ids {
        Ok(ids) => {
            let api_response = ApiResponse {
                status: StatusCode::OK,
                message: "Successfully created user.".to_string(),
                data: Some(json!({
                    "userId": &ids[0],
                    "sessionId": &ids[1]
                })),
                error: None,
                pagination: None,
                version: 1
            };

            api_response
        },

        Err(e) => {
            let api_response= ApiResponse {
                status: StatusCode::BAD_REQUEST,
                message: "Failed to create user.".to_string(),
                data: None,
                error: Some(e.to_string()),
                pagination: None,
                version: 1
            };

            api_response
        }
    };

    serde_json::to_string_pretty(&api_response).unwrap()
}

async fn get_user_route(Path(wallet_address): Path<String>) -> impl IntoResponse {
    let user = User::get_user(wallet_address).await;

    let api_response = match user {
        Ok(user) => {
            let api_response = ApiResponse {
                status: StatusCode::OK,
                message: "Successfully retrieved user.".to_string(),
                data: Some(json!(user)),
                error: None,
                pagination: None,
                version: 1
            };

            api_response
        },

        Err(e) => {
            let api_response= ApiResponse {
                status: StatusCode::BAD_REQUEST,
                message: "Failed to retrieve user.".to_string(),
                data: None,
                error: Some(e.to_string()),
                pagination: None,
                version: 1
            };

            api_response
        }
    };

    serde_json::to_string_pretty(&api_response).unwrap()
}

