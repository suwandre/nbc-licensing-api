use dotenv::dotenv;

/// loads .env via dotenv
pub async fn load_env() {
    dotenv().ok();
}