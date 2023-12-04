use ethers::types::U256;

/// Returns a specific bitmask, used for `IApplication - ApplicationData - first- and secondPackedData`
pub fn bitmask(var: &str) -> U256 {
    let submission_date_bitmask: U256 = U256::from_dec_str("1099511627775").unwrap();
    let approval_date_bitmask: U256 = U256::from_dec_str("18446744073692774400").unwrap();
    let expiration_date_bitmask: U256 = U256::from_dec_str("311385128491287983866514173009920").unwrap();
    let license_fee_bitmask: U256 = U256::from_dec_str("1329227995784915872903807060280344575").unwrap();
    let reporting_frequency_bitmask: U256 = U256::from_dec_str("4294967295").unwrap();
    let reporting_grace_period_bitmask: U256 = U256::from_dec_str("18446744069414584320").unwrap();
    let royalty_grace_period_bitmask: U256 = U256::from_dec_str("311385128491287983866514173009920").unwrap();
    let untimely_reports_bitmask: U256 = U256::from_dec_str("16711680").unwrap();
    let untimely_royalty_payments_bitmask: U256 = U256::from_dec_str("1099511627775").unwrap();
    let extra_data_bitmask: U256 = U256::from_dec_str("39614081257132168796771975168").unwrap();

    match var {
        "submission_date" => submission_date_bitmask,
        "approval_date" => approval_date_bitmask,
        "expiration_date" => expiration_date_bitmask,
        "license_fee" => license_fee_bitmask,
        "reporting_frequency" => reporting_frequency_bitmask,
        "reporting_grace_period" => reporting_grace_period_bitmask,
        "royalty_grace_period" => royalty_grace_period_bitmask,
        "untimely_reports" => untimely_reports_bitmask,
        "untimely_royalty_payments" => untimely_royalty_payments_bitmask,
        "extra_data" => extra_data_bitmask,
        _ => panic!("Invalid bitmask variable"),
    }
}

/// Returns a specific bitpos, used for `IApplication - ApplicationData - first- and secondPackedData`
pub fn bitpos(var: &str) -> U256 {
    let submission_date_bitpos: U256 = U256::from(0);
    let approval_date_bitpos: U256 = U256::from(40);
    let expiration_date_bitpos: U256 = U256::from(80);
    let license_fee_bitpos = U256::from(120);
    let reporting_frequency_bitpos: U256 = U256::from(0);
    let reporting_grace_period_bitpos = U256::from(32);
    let royalty_grace_period_bitpos = U256::from(64);
    let untimely_reports_bitpos = U256::from(96);
    let untimely_royalty_payments_bitpos = U256::from(104);
    let extra_data_bitpos = U256::from(112);

    match var {
        "submission_date" => submission_date_bitpos,
        "approval_date" => approval_date_bitpos,
        "expiration_date" => expiration_date_bitpos,
        "license_fee" => license_fee_bitpos,
        "reporting_frequency" => reporting_frequency_bitpos,
        "reporting_grace_period" => reporting_grace_period_bitpos,
        "royalty_grace_period" => royalty_grace_period_bitpos,
        "untimely_reports" => untimely_reports_bitpos,
        "untimely_royalty_payments" => untimely_royalty_payments_bitpos,
        "extra_data" => extra_data_bitpos,
        _ => panic!("Invalid bitpos variable"),
    }
}