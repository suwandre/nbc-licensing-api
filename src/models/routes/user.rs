use serde_json::Value;
use serde::{Deserialize, Serialize};

/// `CreateUser` struct that represents the request body of the `/user/create` route endpoint.
/// 
/// These fields come from the `User` and `Session` structs that are needed to call the `create_user` function.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub wallet_address: String,
    pub expiration_date: i64,
    pub chain_id: u32,
    pub domain: String,
    pub user_session_id: String,
    pub nonce: String,
    pub signature: String,
    pub payload: Option<Value>,
    pub profile_id: String,
    pub uri: String,
    pub version: u8,
}