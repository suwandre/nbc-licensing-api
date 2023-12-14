use std::str::FromStr;

use api::{get_license_base_terms, calculate_license_fee, pack_data, create_user, check_user_exists};
use chrono::{DateTime, Utc, TimeZone};
use configs::{load_env, get_db, connect_mongo};
use routes::user_routes;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use ethers::types::U256;
use models::{Licensee, LicenseeRaw, User};
use axum::{routing::get, Router};

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

    load_env();
    connect_mongo().await;

    let port = env::var("PORT").expect("PORT not set in .env");
    let port = port.parse::<u16>().expect("Invalid port given");
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    println!("Listening on {}", addr);

    let app = Router::new()
        .route("/", get(run_axum))
        .nest("/user", user_routes());

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}