use bson::doc;
use mongodb::results::InsertOneResult;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, State};
use serde::{Deserialize, Serialize};

use crate::api::{middleware, services};
use crate::api::database_connection::create_client;
use crate::api::model::user::{self, UserData};
use crate::api::services::user::{add, delete, find_all, login, update, LoginResponseData};


pub async fn db_client(collection: &str) -> Collection<user::UserData> {
    let client: Collection<user::UserData> = create_client(collection).await;
    client
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequestBody {
    pub name: String,
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    modified_count: u64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequestBodyUpdate {
    pub name: String,
}
type Result<T, E = Debug<rocket::Error>> = std::result::Result<T, E>;

#[post("/create", format = "json", data = "<body>")]
pub async fn create_user(
    body: Json<UserRequestBody>,
    db: &State<Database>,
) -> Result<Json<InsertOneResult>, Status> {
    let collection = db.collection("my_collection");

    // Insert a document
    let document = user::UserData {
        id: bson::oid::ObjectId::new(),
        name: Some(body.name.to_string()),
        email: Some(body.email.to_string()),
        username: Some(body.username.to_string()),
        password: Some(body.password.to_string()),
    };

    match add(&collection, document).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(Status::NotAcceptable),
    }
}


#[post("/login", format = "json", data = "<body>")]
pub async fn loguser_in(body: Json<LoginData>) -> Result<Json<LoginResponseData>, Status> {
    let collection = db_client("my_collection").await;

    let data = services::user::LoginData {
        username: body.username.as_str(),
        password: body.password.as_str(),
    };

    let authenticate = login(&collection, data).await;
    match authenticate {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(Status::Forbidden),
    }
}

#[delete("/<id>")]
pub async fn delete_user(id: &str, _auth_guard: middleware::auth::JwtGuard) -> Status {
    const COLLECTION_NAME: &str = "my_collection";
    let collection = db_client(COLLECTION_NAME).await;

    match delete(&collection, &id).await {
        Ok(result) if result.deleted_count > 0 => Status::Found,
        Ok(_) => Status::NotFound,
        Err(_) => Status::NotAcceptable,
    }
}

#[patch("/<id>", format = "json", data = "<body>")]
pub async fn update_user(
    id: &str,
    body: Json<UserRequestBodyUpdate>,
    _auth_guard: middleware::auth::JwtGuard,
) -> Result<Json<Info>, Status> {
    let collection = db_client("my_collection").await;

    let modified_count = update(&collection, &id, &body.name).await.map_err(|_e| Status::NotAcceptable)?.modified_count;

    if modified_count == 0 {
        Err(Status::NotFound)
    } else {
        Ok(Json(Info { modified_count }))
    }
}


#[get("/list", format = "json")]
pub async fn get_users(
    _auth_guard: middleware::auth::JwtGuard,
) -> Result<Json<Vec<UserData>>, Status> {
    let collection = db_client("my_collection").await;

    match find_all(&collection).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(Status::NotAcceptable),
    }
}
