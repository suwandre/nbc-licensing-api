use ethers::types::U256;

use super::get_license_base_terms;

/// Calculates the license fee for a license application.
/// 
/// NOTE: This is currently a placeholder function with limited parameters.
/// Factors such as the blockchain used, intended use, geographical scope, negotiations, licensee's reputation score and many more will be considered in the future.
pub async fn calculate_license_fee(
    permit: String,
    duration: U256,
) -> Result<U256, String> {
    // check if the permit exists, else throw an error.
    let permit_exists = get_license_base_terms(permit.clone()).await != "".to_string();

    if !permit_exists {
        return Err("Specified permit does not exist".to_string())
    }

    // sample base fee rates and duration multipliers
    // permit, base fee rate in native currency (e.g. BNB/ETH)
    let base_fee_rates = [
        ("Asset Creation", U256::from(15)*U256::exp10(18)),
        ("Existing Asset Usage", U256::from_dec_str("7.5")*U256::exp10(18)),
        ("Asset Modification", U256::from(12)*U256::exp10(18))
    ];

    // seconds, multiplier
    let duration_multipliers = [
        (U256::from(7776000), 1), // 3 months in seconds
        (U256::from(15552000), 3), // 6 months in seconds
        (U256::from(31536000), 6), // 1 year in seconds
        (U256::from(63072000), 9), // 2 years in seconds
    ];

    let base_fee = base_fee_rates.iter().find(|&&x| x.0 == permit).unwrap().1;
    let duration_multiplier = duration_multipliers.iter().find(|&&x| x.0 == duration).unwrap().1;

    Ok(U256::from(base_fee * duration_multiplier))
}