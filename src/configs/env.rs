use dotenv::dotenv;

/// loads .env via dotenv
pub fn load_env() {
    dotenv().ok();
}