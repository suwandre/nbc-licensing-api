use std::{env, sync::Arc};

use ethers::prelude::*;
use lazy_static::lazy_static;

// lazy loads the license contract address
static LICENSE_ADDRESS: Lazy<Address> = Lazy::new(|| {
    env::var("LICENSE_ADDRESS")
        .expect("LICENSE_ADDRESS must be set")
        .parse()
        .expect("LICENSE_ADDRESS is not a valid address")
});

// initializes the license contract once
abigen!(
    LicenseContract,
    "src/abi/License.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// initializes a provider, wallet instance, client and the license contract once
lazy_static!{
    // creates a provider instance from the RPC URL.
    // NOTE: we are currently using BNB Testnet to save costs and for non-proper production-ready purposes.
    pub static ref PROVIDER: Provider<Http> = {
        let url = env::var("BNB_TESTNET").expect("BNB_TESTNET must be set");
        Provider::<Http>::try_from(url).expect("failed to create provider")
    };

    // creates a wallet instance from the secondary deployer wallet private key
    pub static ref WALLET: LocalWallet = {
        let private_key = env::var("SECONDARY_DEPLOYER_WALLET_PVT_KEY").expect("SECONDARY_DEPLOYER_WALLET_PVT_KEY must be set");
        private_key.parse::<LocalWallet>().expect("could not parse private key")
    };

    // creates a client instance from the provider and wallet instances
    // NOTE: the current client instance is only for BNB Testnet.
    pub static ref CLIENT: SignerMiddleware<Provider<Http>, LocalWallet> = {
        let client = SignerMiddleware::new(PROVIDER.clone(), WALLET.clone());
        client
    };

    // initializes the license contract using the signer middleware instance
    pub static ref LICENSE: LicenseContract<SignerMiddleware<Provider<Http>, LocalWallet>> = {
        let license = LicenseContract::new(LICENSE_ADDRESS.clone(), Arc::new(CLIENT.clone()));
        license
    };
}

