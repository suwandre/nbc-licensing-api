use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////
////////// Structs, Impl Blocks and Traits related to Licensee.sol ////////////
///////////////////////////////////////////////////////////////////////////////

/// `Licensee` is used to represent a licensee's account information.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Licensee {
    /// the licensee's wallet address
    pub wallet_address: String,
    /// the licensee's full name
    pub name: String,
    /// the licensee's date of birth
    pub dob: DateTime<Utc>,
    /// the licensee's home/company address
    pub address: String,
    /// the licensee's email address
    pub email_address: String,
    /// the licensee's phone number
    pub phone_number: String,
    /// (optional) the licensee's company name
    pub company: Option<String>,
    /// the licensee's nationality
    pub nationality: String,
    /// the licensee's country of application
    pub country_of_application: String,
    /// whether the licensee account is usable or not (i.e. applicable for license applications)
    pub usable: bool,
}

/// An alternative to `Licensee` that returns the raw data from the `LicenseeAccount` struct in the smart contract.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LicenseeRaw {
    /// contains the licensee's account information in the form of a `bytes` string
    pub data: String,
    /// whether the licensee account is usable or not (i.e. applicable for license applications)
    pub usable: bool,
}