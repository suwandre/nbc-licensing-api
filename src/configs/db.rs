use mongodb::{Client, options::{ClientOptions, ResolverConfig, ServerApiVersion, ServerApi}, Database, Collection};
use std::env;

/// connects to MongoDB and returns its client instance
pub async fn connect_mongo() -> Client {
    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // using parse_with_resolver_config to resolve DNS issue that causes mongoDB to not load properly/in time
    let mut client_options = ClientOptions::parse_with_resolver_config(&mongo_uri, ResolverConfig::cloudflare()).await.unwrap();
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options).unwrap_or_else(|err| {
        panic!("Failed to initialize MongoDB client: {}", err);
    });

    client
}

/// returns a MongoDB database instance from the given `name`.
pub async fn get_db(name: &str) -> Database {
    let client = connect_mongo().await;
    client.database(name)
}

/// returns a MongoDB collection instance from the given `db_name` and `col_name`.
pub async fn get_collection<T>(db_name: &str, col_name: &str) -> Collection<T> {
    let db = get_db(db_name).await;
    
    db.collection::<T>(col_name)
}