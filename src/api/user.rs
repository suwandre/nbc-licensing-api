use chrono::{DateTime, Utc};
use mongodb::{Collection, bson::{oid::ObjectId, doc}};
use serde_json::Value;

use crate::{models::{ApiResponse, User, Session}, utils::CustomError, configs::get_collection};

/// Creates a user and simultaneously a new session instance, storing both to the database.
/// 
/// Returns the Object ID of both the newly created user and the session instance if no errors occur.
/// 
/// NOTE: We assume that a new user will obviously not have an existing session instance, so we don't check for that here.
pub async fn create_user(
    wallet_address: String,
    expiration_date: i64,
    chain_id: u32,
    domain: String,
    user_session_id: String,
    nonce: String,
    signature: String,
    payload: Option<Value>,
    profile_id: String,
    uri: String,
    version: u8
) -> Result<Vec<ObjectId>, CustomError> {
    // checks if the user exists. if they do, return an error.
    if check_user_exists(wallet_address.clone()).await? {
        return Err(CustomError::DatabaseError("To be created user already exists.".to_string()));
    }

    let user = User::new(wallet_address.clone());
    let user_id = user.store_user().await?;

    let session = Session::new(
        expiration_date,
        wallet_address,
        chain_id,
        domain,
        user_session_id,
        nonce,
        signature,
        payload,
        profile_id,
        uri,
        version
    );

    let session_id = session.store_session().await?;

    Ok(Vec::from([user_id, session_id]))
}

/// Checks whether a user with the specified wallet address exists.
pub async fn check_user_exists(wallet_address: String) -> Result<bool, CustomError> {
    let user_col: Collection<User> = get_collection("MainDatabase", "Users").await;
    let user = user_col.find_one(doc! { "wallet_address": wallet_address }, None).await?;

    Ok(user.is_some())
}