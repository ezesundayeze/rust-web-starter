use mongodb::{options::ClientOptions, Client, Collection};
use dotenv::dotenv;
use crate::api::config::Config;

pub async fn create_client<T>(collection: &str) -> Collection<T> {
    dotenv().ok();

    let config = Config::new();

    let uri = config.database_url;
    let db_name = config.database_name;

    let client_options = ClientOptions::parse(uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&db_name);
    let collection = db.collection::<T>(&collection);
    collection
}
