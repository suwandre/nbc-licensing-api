use api::get_account_dev;
use salvo::prelude::*;
use std::env;

mod api;
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

    let router = Router::new().get(run_salvo);
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}