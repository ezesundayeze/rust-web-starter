use mongodb::{options::ClientOptions, Client, Collection};
use dotenv::dotenv;
use std::env;

pub async fn create_client<T>(collection: &str) -> Collection<T> {
    dotenv().ok();

    let uri = env::var("MONGODB_URI").expect("MONGODB_URI not found in environment");
    let db_name = env::var("DB_NAME").expect("DB_NAME not found in environment");

    let client_options = ClientOptions::parse(uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&db_name);
    let collection = db.collection::<T>(&collection);
    collection
}
