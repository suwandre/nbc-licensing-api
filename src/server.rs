use api::get_license_base_terms;
use chrono::{DateTime, Utc};
use configs::load_env;
use models::{Licensee, LicenseeRaw};
use salvo::prelude::*;

mod api;
mod configs;
mod models;
mod utils;

/// Checks to see if Salvo is running
#[handler]
async fn run_salvo() -> &'static str {
    "Salvo is running!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    load_env().await;

    let router = Router::new().get(run_salvo);
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}