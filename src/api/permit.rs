use ethers::utils::keccak256;

use crate::utils::LICENSE;

/// Fetches a license permit's base terms URL from `Permit.sol - getLicense`.
/// 
/// If the permit does not exist, this function will return an empty string.
pub async fn get_license_base_terms(permit: String) -> String {
    let license_hash = keccak256(permit);

    LICENSE.get_license(license_hash).await.unwrap()
}