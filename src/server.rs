use std::str::FromStr;

use api::{get_license_base_terms, calculate_license_fee, pack_data};
use chrono::{DateTime, Utc};
use configs::load_env;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use ethers::types::U256;
use models::{Licensee, LicenseeRaw};
use axum::{routing::get, Router};

mod api;
mod configs;
mod models;
mod utils;

/// Checks to see if Axum is running.
async fn run_axum() -> &'static str {
    "Axum is running!"
}

#[tokio::main]
async fn main() {
    load_env();

    let port = env::var("PORT").expect("PORT not set in .env");
    let port = port.parse::<u16>().expect("Invalid port given");
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    let app = Router::new()
        .route("/", get(run_axum));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}