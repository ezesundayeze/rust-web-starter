use bson::doc;
use futures::StreamExt;
use mongodb::{
    error::Result as MongoResult,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequestBody {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequestBodyUpdate {
    pub name: String,
}


pub async fn add(
    collection: &Collection<UserData>,
    document: UserData,
) -> MongoResult<InsertOneResult> {
    let result = collection.insert_one(document, None).await?;
    Ok(result)
}

pub async fn delete(collection: &Collection<UserData>, id: &str) -> MongoResult<DeleteResult> {
    let object_id = bson::oid::ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": object_id};
    let result = collection.delete_one(filter, None).await.unwrap();
    Ok(result)
}

pub async fn update(
    collection: &Collection<UserData>,
    id: &str,
    name: &str,
) -> MongoResult<UpdateResult> {
    let object_id = bson::oid::ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": object_id};
    let update = doc! {"$set": {"name": name, "email":"mailstoeze@gmail.com" }};
    let result = collection.update_one(filter, update, None).await.unwrap();
    Ok(result)
}

pub async fn find_all(collection: &Collection<UserData>) -> MongoResult<Vec<UserData>> {
    let filter = doc! {};
    let mut cursor = collection.find(filter, None).await?;
    let mut docs = Vec::new();
    while let Some(doc) = cursor.next().await {
        docs.push(doc?);
    }
    Ok(docs)
}