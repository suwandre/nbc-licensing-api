use ethers::types::U256;

use crate::utils::bitpos;

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
        ("Existing Asset Usage", U256::from_dec_str("7.5").unwrap()*U256::exp10(18)),
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

/// Packs the application's parameters into `firstPackedData` and `secondPackedData`, as required by `IApplication.sol - ApplicationData`.
pub fn pack_data(
    duration: U256,
    license_fee: U256,
    reporting_frequency: U256,
    reporting_grace_period: U256,
    royalty_grace_period: U256,
    extra_data: U256
) -> [U256; 2] {
    let current_timestamp = U256::from(chrono::Utc::now().timestamp());
    let submission_date = current_timestamp;
    let approval_date = U256::from(0);
    let expiration_date = current_timestamp + duration;

    let untimely_reports = U256::from(0);
    let untimely_royalty_payments = U256::from(0);

    // bitposes:
    // [0 - 39] - the application's submission date.
    // [40 - 79] - the application's approval date (if approved; else 0).
    // [80 - 119] - the license's expiration date.
    // [120 - 255] - the license fee for this license.
    let first_packed_data = submission_date 
        | (approval_date << bitpos("approval_date")) 
        | (expiration_date << bitpos("expiration_date")) 
        | (license_fee << bitpos("license_fee"));

    // bitposes:
    // [0 - 31] - the reporting frequency (frequency that a licensee must submit a revenue report and subsequently pay their royalty share).
    // [32 - 63] - the reporting grace period (amount of time given to report after the reporting frequency has passed before the licensee is penalized).
    // [64 - 95] - the royalty grace period (amount of time given to pay royalty after the revenue report for that period has been approved before the licensee is penalized).
    // [96 - 103] - the amount of untimely reports (i.e. reports that were not submitted within the reporting grace period).
    // [104 - 111] - the amount of untimely royalty payments (i.e. royalty payments that were not submitted within the royalty grace period).
    // [112 - 255] - extra data (if applicable).
    let second_packed_data = reporting_frequency 
        | (reporting_grace_period << bitpos("reporting_grace_period")) 
        | (royalty_grace_period << bitpos("royalty_grace_period")) 
        | (untimely_reports << bitpos("untimely_reports"))
        | (untimely_royalty_payments << bitpos("untimely_royalty_payments")
        | (extra_data << bitpos("extra_data")));

    [first_packed_data, second_packed_data]
}