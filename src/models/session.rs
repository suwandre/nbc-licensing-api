use chrono::{DateTime, Utc};
use mongodb::{bson::oid::ObjectId, Collection};
use crate::configs::get_collection;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `Session` struct that represents a session that gets created when a user logs in via the webapp.
/// 
/// All dates and timestamps are stored in UNIX format.
/// 
/// Since this uses Moralis' NextAuth provider, the majority of these fields will follow SIWE's EIP4361 standard.
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    /// the object ID of the session in the database
    pub _id: ObjectId,
    /// the session's expiration date
    pub expiration_date: i64,
    /// the user's wallet address tied to this session
    pub wallet_address: String,
    /// the id of the chain the user is on when they logged in
    pub chain_id: u32,
    /// the domain that is requesting the login
    pub domain: String,
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

impl Session {
    /// Creates a new `Session` instance.
    pub fn new(
        expiration_date: i64,
        wallet_address: String,
        chain_id: u32,
        domain: String,
        nonce: String,
        signature: String,
        payload: Option<Value>,
        profile_id: String,
        uri: String,
        version: u8
    ) -> Self {
        Self {
            _id: ObjectId::new(),
            expiration_date,
            wallet_address,
            chain_id,
            domain,
            nonce,
            signature,
            payload,
            profile_id,
            uri,
            version
        }
    }

    /// Stores a `Session` instance in the database.
    /// 
    /// Returns its newly created `ObjectId` if the operation is successful.
    pub async fn store_session(&self) -> Result<ObjectId, mongodb::error::Error> {
        let session_col: Collection<Session> = get_collection("MainDatabase", "Sessions").await;
        let session = session_col.insert_one(self, None).await?;

        match session.inserted_id.as_object_id() {
            Some(id) => Ok(id.clone()),
            None => Err(mongodb::error::Error::custom("Failed to get inserted ID."))
        }
    }
}

