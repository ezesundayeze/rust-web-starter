use crate::api::database_connection::create_client;
use crate::api::model::user::{self, UserData};
use crate::api::services::user::{add, delete, find_all, login, update, LoginResponseData};
use crate::api::{middleware, services};
use bson::doc;
use mongodb::results::InsertOneResult;
use mongodb::Collection;
use rocket::http::Status;
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post};
use serde::{Deserialize, Serialize};

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
pub struct UserRequestBodyUpdate {
    pub name: String,
}
type Result<T, E = Debug<rocket::Error>> = std::result::Result<T, E>;

#[post("/create", format = "json", data = "<body>")]

pub async fn create_user(body: Json<UserRequestBody>) -> Result<Json<InsertOneResult>, Status> {
    let collection = db_client("my_collection").await;

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
pub async fn delete_user(id: &str, auth_guard: middleware::auth::JwtGuard) -> Status {
    let collection = db_client("my_collection").await;

    // Access the user's id in case you need it
    let user_id = &auth_guard.user_id;
    println!("{}", user_id);

    #[derive(Debug, Serialize, Deserialize)]
    struct Info {
        deleted_count: u64,
    }

    // Delete and check if the delete operation was successful
    match delete(&collection, &id).await {
        Ok(result) => {
            if result.deleted_count == 0 {
                Status::NotFound
            } else {
                Status::Found
            }
        }
        Err(_e) => Status::NotAcceptable,
    }
}

#[patch("/<id>", format = "json", data = "<body>")]
pub async fn update_user(
    id: &str,
    body: Json<UserRequestBodyUpdate>,
    _auth_guard: middleware::auth::JwtGuard,
) -> Status {
    let collection = db_client("my_collection").await;

    #[derive(Debug, Serialize, Deserialize)]
    struct Info {
        modified_count: u64,
    }

    // Delete and check if the delete operation was successful
    match update(&collection, &id, &body.name).await {
        Ok(_result) => Status::Accepted,
        Err(_e) => Status::NotAcceptable,
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
