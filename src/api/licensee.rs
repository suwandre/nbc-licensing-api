use ethers::types::H160;

use crate::utils::{LicenseeAccount, contract_base::LICENSE};
use std::{error::Error, str::FromStr};

/// Calls `Licensee - getAccount` on the License contract and returns a LicenseeAccount struct instance.
/// NOTE: This function is only callable by devs and licensors. Licensees should use `get_account` instead.
pub async fn get_account_dev(licensee_address: String) -> Result<LicenseeAccount, Box<dyn Error>> {
    let licensee = H160::from_str(&licensee_address)?;
    let licensee_account = LICENSE.get_account(licensee).await?;

    Ok(licensee_account)
}