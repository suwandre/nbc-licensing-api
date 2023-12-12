use chrono::{DateTime, Utc};
use ethers::{types::H160, utils::hex::{decode, encode_prefixed}};

use crate::{utils::{LicenseeAccount, contract_base::LICENSE}, models::{LicenseeRaw, Licensee}};
use std::str::FromStr;

impl Licensee {
    /// Gets a licensee account's data.
    /// 
    /// Retrieves the raw `LicenseeRaw` struct instance, then decodes the data into a `Licensee` struct instance.
    pub async fn get_account_data(licensee_address: String) -> Result<Self, String> {
        let licensee_raw = LicenseeRaw::get_account_raw(licensee_address).await?;
        let decoded = Licensee::decode_licensee_data(licensee_raw.data, licensee_raw.usable)?;

        Ok(decoded)
    }

    /// Decodes the result obtained from `Licensee.sol - getAccount` into a `Licensee` struct instance.
    /// 
    /// Data should be a `bytes` type string that IS NOT empty and can be decoded using `ethers::utils::hex::decode`.
    pub fn decode_licensee_data<T>(data: T, usable: bool) -> Result<Self, String>
    where
        T: AsRef<[u8]>
    {
        // if the data is empty, return a `Licensee` struct instance with all fields set to their default values.
        if data.as_ref().len() == 0 {
            return Ok(Licensee::default_licensee_data())
        }

        let decoded = decode(&data).map_err(|e| format!("Error decoding data: {}", e.to_string()))?;
        let decoded_str = String::from_utf8(decoded).map_err(|e| format!("Error converting bytes to string: {}", e.to_string()))?;
        let mut split = decoded_str.split('|');

        let wallet_address = split.next().ok_or("")?.to_string();
        let name = split.next().ok_or("")?.to_string();
        let dob = DateTime::parse_from_rfc3339(&split.next().ok_or("")?).unwrap().with_timezone(&Utc);
        let address = split.next().ok_or("")?.to_string();
        let email_address = split.next().ok_or("")?.to_string();
        let phone_number = split.next().ok_or("")?.to_string();
        let company = split.next().filter(|&x| x != "" && x != "None").map(|x| x.to_string());
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

    /// Returns a `Licensee` struct instance with all fields set to their default values.
    /// 
    /// This will be called when trying to get a licensee account that does not exist.
    pub fn default_licensee_data() -> Self {
        Licensee {
            wallet_address: "".to_string(),
            name: "".to_string(),
            dob: Utc::now(),
            address: "".to_string(),
            email_address: "".to_string(),
            phone_number: "".to_string(),
            company: None,
            nationality: "".to_string(),
            country_of_application: "".to_string(),
            usable: false
        }
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
            Err(e) => panic!("dob parameter is NOT RFC3339 compliant: {}", e.to_string())
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

    /// Calls `Licensee - getAccount` on the License contract and returns a LicenseeAccount struct instance.
    pub async fn get_account_raw(licensee_address: String) -> Result<Self, String> {
        let licensee = H160::from_str(&licensee_address).map_err(|e| format!("Error converting licensee address to H160: {}", e.to_string()))?;
        let licensee_account: LicenseeAccount = LICENSE.get_account(licensee).await.map_err(|e| format!("Error getting licensee account: {}", e.to_string()))?;

        Ok(LicenseeRaw {
            data: licensee_account.data.to_string(),
            usable: licensee_account.usable
        })
    }
}