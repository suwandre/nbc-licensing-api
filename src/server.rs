use std::str::FromStr;

use api::{get_license_base_terms, calculate_license_fee, pack_data, create_user, check_user_exists};
use axum::http::header::{CONTENT_TYPE, ACCESS_CONTROL_ALLOW_ORIGIN};
use axum::http::{Method, HeaderValue};
use chrono::{DateTime, Utc, TimeZone};
use configs::{load_env, get_db, connect_mongo};
use routes::user_routes;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

mod api;
mod configs;
mod models;
mod utils;
mod routes;

/// Checks to see if Axum is running.
async fn run_axum() -> &'static str {
    "Axum is running!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    env_logger::init();

    load_env();
    connect_mongo().await;

    let port = env::var("PORT").expect("PORT not set in .env");
    let port = port.parse::<u16>().expect("Invalid port given");
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    println!("Listening on {}", addr);

    // temporarily allowing localhost:3000 for testing
    let cors_middleware = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE, ACCESS_CONTROL_ALLOW_ORIGIN]);

    let app = Router::new()
        .route("/", get(run_axum))
        .nest("/user", user_routes())
        .layer(cors_middleware);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}