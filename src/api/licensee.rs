use chrono::{DateTime, Utc};
use ethers::{types::H160, utils::{keccak256, hex::{encode, decode, decode_to_slice, decode_to_array, encode_prefixed}}};

use crate::{utils::{LicenseeAccount, contract_base::LICENSE}, models::{LicenseeRaw, Licensee}};
use std::{error::Error, str::FromStr};

/// Calls `Licensee - getAccount` on the License contract and returns a LicenseeAccount struct instance.
pub async fn get_account_raw(licensee_address: String) -> Result<LicenseeAccount, Box<dyn Error>> {
    let licensee = H160::from_str(&licensee_address)?;
    let licensee_account = LICENSE.get_account(licensee).await?;

    Ok(licensee_account)
}

impl Licensee {
    /// Decodes the result obtained from `Licensee.sol - getAccount` into a `Licensee` struct instance.
    /// 
    /// Data should be a `bytes` type string that can be decoded using `ethers::utils::hex::decode`.
    pub fn decode_licensee_data<T>(data: T, usable: bool) -> Result<Self, String>
    where
        T: AsRef<[u8]>
    {
        let decoded = decode(&data).map_err(|e| format!("Error decoding data: {}", e.to_string()))?;
        let decoded_str = String::from_utf8(decoded).map_err(|e| format!("Error converting bytes to string: {}", e.to_string()))?;
        let mut split = decoded_str.split('|');

        let wallet_address = split.next().ok_or("")?.to_string();
        let name = split.next().ok_or("")?.to_string();
        let dob = DateTime::parse_from_rfc3339(&split.next().ok_or("")?).unwrap().with_timezone(&Utc);
        let address = split.next().ok_or("")?.to_string();
        let email_address = split.next().ok_or("")?.to_string();
        let phone_number = split.next().ok_or("")?.to_string();
        let company = split.next().filter(|&x| x != "").map(|x| x.to_string());
        let nationality = split.next().ok_or("")?.to_string();
        let country_of_application = split.next().ok_or("")?.to_string();

        Ok(Licensee {
            wallet_address,
            name,
            dob,
            address,
            email_address,
            phone_number,
            company,
            nationality,
            country_of_application,
            usable
        })
    }
}

impl LicenseeRaw {
    /// Encodes the parameters into a `bytes` type string and returns a `LicenseeRaw` struct instance, containing the encoded data and a `usable` field.
    /// 
    /// Gets called when a user registers for a licensee account; the returned `LicenseeRaw` instance is to be submitted to Metamask via the frontend, calling `Licensee.sol - registerAccount`.
    /// 
    /// NOTE: `dob` (i.e. date of birth) MUST be RFC3339 compliant, otherwise the function will panic.
    pub fn register_account_params(
        wallet_address: String,
        name: String,
        dob: String,
        address: String,
        email: String,
        phone: String,
        company: Option<String>,
        nationality: String,
        country_of_application: String
    ) -> Self {
        // if the `dob` parameter is NOT RFC3339 compliant, panic.
        match DateTime::parse_from_rfc3339(&dob) {
            Ok(_) => (),
            Err(e) => panic!("dob parameter is NOT in the right format: {}", e.to_string())
        }

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
}