use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::{utils::serialization::datetime::{serialize_datetime, deserialize_datetime, serialize_datetime_option, deserialize_datetime_option}, configs::get_db};

/// `User` struct that represents a user in the database.
/// 
/// NOTE: Temporarily, no passwords are managed here as users are required to log in via their wallet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// the object ID of the user in the database
    pub _id: ObjectId,
    /// the user's wallet address tied to this User account instance
    pub wallet_address: String,
    /// when the user instance was created
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    pub created_at: DateTime<Utc>,
    /// when the user instance was last updated
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    pub updated_at: DateTime<Utc>,
    
    /* web2 related info */
    /// the user's full name
    pub name: Option<String>,
    /// the user's date of birth
    #[serde(serialize_with = "serialize_datetime_option", deserialize_with = "deserialize_datetime_option")]
    pub dob: Option<DateTime<Utc>>,
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
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    pub last_kyc_verification: DateTime<Utc>,
}

impl User {
    /// Creates a new User instance. 
    /// 
    /// NOTE: `new` is expected to be called when a user logs in via their wallet and doesn't have an existing account yet.
    /// Hence, all Web2-related fields will be left blank.
    pub fn new(wallet_address: String) -> Self {
        Self {
            _id: ObjectId::new(),
            wallet_address,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name: None,
            dob: None,
            email: None,
            phone: None,
            address: None,
            company: None,
            kyc_verified: false,
            last_kyc_verification: Utc::now(),
        }
    }

    /// Stores a User instance in the database.
    /// 
    /// Returns its newly created `ObjectId` if the operation is successful.
    pub async fn store_user(&self) -> Result<ObjectId, mongodb::error::Error> {
        let db = get_db("MainDatabase").await;
        let user_col = db.collection::<Self>("Users");

        let user = user_col.insert_one(self, None).await?;

        match user.inserted_id.as_object_id() {
            Some(id) => Ok(id),
            None => Err(mongodb::error::Error::custom("Failed to get inserted ID."))
        }
    }
}