use bson::doc;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub username: Option<String>,
}