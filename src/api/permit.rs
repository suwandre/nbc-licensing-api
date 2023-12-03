use ethers::utils::keccak256;

use crate::utils::LICENSE;

/// Fetches a license permit's base terms URL from `Permit.sol - getLicense`.
pub async fn get_license_base_terms(permit: String) -> Result<String, String> {
    let license_hash = keccak256(permit);

    LICENSE.get_license(license_hash).await.map_err(|e| e.to_string())
}