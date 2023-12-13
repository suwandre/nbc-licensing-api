use chrono::{DateTime, Utc};
use mongodb::{bson::oid::ObjectId, Collection};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use crate::{utils::serialization::datetime::{serialize_datetime, deserialize_datetime, serialize_datetime_option, deserialize_datetime_option}, configs::{get_db, get_collection}};

/// `User` struct that represents a user in the database.
/// 
/// All dates and timestamps are stored in UNIX format.
/// NOTE: Temporarily, no passwords are managed here as users are required to log in via their wallet.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// the object ID of the user in the database
    pub _id: Option<ObjectId>,
    /// the user's wallet address tied to this User account instance
    pub wallet_address: String,
    /// when the user instance was created
    pub created_at: i64,
    /// when the user instance was last updated
    pub updated_at: i64,
    
    /* web2 related info */
    /// the user's full name
    pub name: Option<String>,
    /// the user's date of birth
    pub dob: Option<i64>,
    /// the user's email address
    pub email: Option<String>,
    /// the user's phone number
    pub phone: Option<String>,
    /// the user's main home/company address
    pub address: Option<String>,
    /// the user's company name
    pub company: Option<String>,
    /// if the user has completed KYC
    pub kyc_verified: bool,
    /// when the user last completed KYC.
    /// 
    /// if the user has never completed KYC, this will be the same as `created_at`.
    pub last_kyc_verification: i64,
}

impl User {
    /// Creates a new User instance. 
    /// 
    /// NOTE: `new` is expected to be called when a user logs in via their wallet and doesn't have an existing account yet.
    /// Hence, all Web2-related fields will be left blank.
    pub fn new(wallet_address: String) -> Self {
        Self {
            _id: Some(ObjectId::new()),
            wallet_address,
            created_at: Utc::now().timestamp(),
            updated_at: Utc::now().timestamp(),
            name: None,
            dob: None,
            email: None,
            phone: None,
            address: None,
            company: None,
            kyc_verified: false,
            last_kyc_verification: Utc::now().timestamp(),
        }
    }

    /// Stores a User instance in the database.
    /// 
    /// Returns its newly created `ObjectId` if the operation is successful.
    pub async fn store_user(&self) -> Result<ObjectId, mongodb::error::Error> {
        let user_col: Collection<User> = get_collection("MainDatabase", "Users").await;
        let user = user_col.insert_one(self, None).await?;

        match user.inserted_id.as_object_id() {
            Some(id) => Ok(id),
            None => Err(mongodb::error::Error::custom("Failed to get inserted ID."))
        }
    }
}