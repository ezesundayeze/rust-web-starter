use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use bson::doc;
use mongodb::{
    Collection,
};
use serde::{Deserialize, Serialize};
use crate::api::services::user;

use crate::api::database_connection::create_client;


pub async fn db_client(collection: &str) -> Collection<user::UserData> {
    let client: Collection<user::UserData> = create_client(collection).await;
    client
}

#[post("/user/create")]
pub async fn create_user(body: web::Json<user::UserRequestBody>) -> impl Responder {
    let collection = db_client("my_collection").await;

    // Insert a document
    let document = user::UserData {
        id: bson::oid::ObjectId::new(),
        name: Some(body.name.to_string()),
        email: Some(body.email.to_string()),
    };

    match user::add(&collection, document).await {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to insert user: {}", e)),
    }
}

#[delete("/user/{id}")]
pub async fn delete_user(id: web::Path<String>) -> impl Responder {
    let collection = db_client("my_collection").await;

    #[derive(Debug, Serialize, Deserialize)]
    struct Info {
        deleted_count: u64,
    }

    // Delete and check if the delete operation was successful
    match user::delete(&collection, &id).await {
        Ok(result) => HttpResponse::Ok().json(web::Json(Info {
            deleted_count: result.deleted_count,
        })),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to delete: {}", e)),
    }
}

#[patch("/user/{id}")]
pub async fn update_user(
    id: web::Path<String>,
    body: web::Json<user::UserRequestBodyUpdate>,
) -> impl Responder {
    let collection = db_client("my_collection").await;

    #[derive(Debug, Serialize, Deserialize)]
    struct Info {
        modified_count: u64,
    }

    // Delete and check if the delete operation was successful
    match user::update(&collection, &id, &body.name).await {
        Ok(result) => HttpResponse::Ok().json(web::Json(Info {
            modified_count: result.modified_count,
        })),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to update: {}", e)),
    }
}

#[get("/user/list")]
pub async fn get_users() -> impl Responder {
    let collection = db_client("my_collection").await;

    match user::find_all(&collection).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to find: {}", e)),
    }
}
