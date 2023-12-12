use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use crate::utils::serialization::datetime::{serialize_datetime, deserialize_datetime};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `Session` struct that represents a session that gets created when a user logs in via the webapp.
/// 
/// Since this uses Moralis' NextAuth provider, the majority of these fields will follow SIWE's EIP4361 standard.
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    /// the object ID of the session in the database
    pub _id: ObjectId,
    /// the session's expiration date
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    pub expiration_date: DateTime<Utc>,
    /// the user's wallet address tied to this session
    pub wallet_address: String,
    /// the id of the chain the user is on when they logged in
    pub chain_id: u32,
    /// the domain that is requesting the login
    pub domain: String,
    /// the unique session-based user ID that gets generated when the user logs in
    /// 
    /// NOTE: not to be confused with the user's object ID in the database.
    pub user_session_id: String,
    /// a random string generated to prevent replay attacks
    pub nonce: String,
    /// a signed signature from the user when they log in
    pub signature: String,
    /// (optional) the payload that gets sent to the webapp when the user logs in
    pub payload: Option<Value>,
    /// the user's unique profile ID that gets generated when the user logs in
    pub profile_id: String,
    /// the uri that the user is logging in from
    pub uri: String,
    /// the current version of SIWE (most likely will remain 1)
    pub version: u8
}

