use chrono::{DateTime, Utc};
use ethers::{types::H160, utils::{keccak256, hex::{encode, decode, decode_to_slice, decode_to_array, encode_prefixed}}};

use crate::{utils::{LicenseeAccount, contract_base::LICENSE}, models::LicenseeRaw};
use std::{error::Error, str::FromStr};

/// Gets called when a user wants to register for a licensee account.
/// 
/// Returns a `LicenseeRaw` struct instance, to be submitted to Metamask via the frontend.
pub async fn register_account(
    wallet_address: String,
    name: String,
    dob: DateTime<Utc>,
    address: String,
    email: String,
    phone: String,
    company: Option<String>,
    nationality: String,
    country_of_application: String
) -> LicenseeRaw {
    let concat = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}",
        wallet_address, name, dob, address, email, phone, company.unwrap_or_default(), nationality, country_of_application
    );

    let data = encode_prefixed(concat.as_bytes());

    LicenseeRaw {
        data,
        usable: false
    }
}

/// Calls `Licensee - getAccount` on the License contract and returns a LicenseeAccount struct instance.
pub async fn get_account(licensee_address: String) -> Result<LicenseeAccount, Box<dyn Error>> {
    let licensee = H160::from_str(&licensee_address)?;
    let licensee_account = LICENSE.get_account(licensee).await?;

    Ok(licensee_account)
}