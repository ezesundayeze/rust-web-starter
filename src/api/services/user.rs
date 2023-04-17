use bson::{doc};
use futures::StreamExt;
use mongodb::{
    error::Result as MongoResult,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

use jsonwebtoken::{Algorithm, encode, Header, EncodingKey};
use serde::Serialize;
use crate::api::{model::user, config};

#[derive(Debug, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, PartialEq)]
pub enum JwtGuardError {
    InvalidToken,
}

fn generate_token(user_id: String, jwt_secret: &str) -> Result<String, JwtGuardError> {

    let now = chrono::Utc::now().timestamp();
    let exp = now + 60 * 60 * 24 * 7; // 1 week expiration
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: exp as usize,
    };
    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());
    match encode(&header, &claims, &encoding_key){
        Ok(result)=> Ok(result),
        Err(_)=> Err(JwtGuardError::InvalidToken)
    }
}


pub struct LoginData <'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize)]
pub struct LoginResponseData {
    username: String,
    token: String
}

pub async fn add(
    collection: &Collection<user::UserData>,
    document: user::UserData,
) -> MongoResult<InsertOneResult> {
    let result = collection.insert_one(document, None).await?;
    Ok(result)
}


pub enum Error {
    NotFound,
    // other variants
}


pub async fn login(
    collection: &Collection<user::UserData>,
    data: LoginData<'_>
)-> Result<LoginResponseData, Error> {
    let filter = doc! { "username": data.username};
    let result = collection.find_one(filter, None).await.expect("User not found");
    let config = config::Config::new();
    let user = result.ok_or_else(|| Error::NotFound)?;

    let token = generate_token(user.id.to_string(), &config.jwt_secret).unwrap();
    let result_response = LoginResponseData {
        username: user.username.unwrap(),
        token
    };
    Ok(result_response)
}

pub async fn delete(collection: &Collection<user::UserData>, id: &str) -> MongoResult<DeleteResult> {
    let object_id = bson::oid::ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": object_id};
    let result = collection.delete_one(filter, None).await.unwrap();
    Ok(result)
}

pub async fn update(
    collection: &Collection<user::UserData>,
    id: &str,
    name: &str,
) -> MongoResult<UpdateResult> {
    let object_id = bson::oid::ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": object_id};
    let update = doc! {"$set": {"name": name }};
    let result = collection.update_one(filter, update, None).await.unwrap();
    Ok(result)
}

pub async fn find_all(collection: &Collection<user::UserData>) -> MongoResult<Vec<user::UserData>> {
    
    let filter = doc! {};
    let mut cursor = collection.find(filter, None).await?;
    let mut docs = Vec::new();

    while let Some(doc) = cursor.next().await {
        docs.push(doc?);
    }
    Ok(docs)
}
