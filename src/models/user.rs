use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::utils::serialization::datetime::{serialize_datetime, deserialize_datetime};

/// `User` struct that represents a user in the database.
#[derive(Debug, Serialize, Deserialize)]
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
    pub name: String,
    /// the user's date of birth
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    pub dob: DateTime<Utc>,
    /// the user's email address
    pub email: String,
    /// the user's phone number
    pub phone: String,
    /// the user's main home/company address
    pub address: String,
    /// (optional) the user's company name
    pub company: Option<String>,
    /// if the user has completed KYC
    pub kyc_verified: bool,
    /// when the user last completed KYC.
    /// 
    /// if the user has never completed KYC, this will be the same as `created_at`.
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    pub last_kyc_verification: DateTime<Utc>,
}