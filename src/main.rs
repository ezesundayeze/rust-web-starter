use actix_web::{App, HttpServer};
mod api;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::controllers::user::create_user)
            .service(api::controllers::user::delete_user)
            .service(api::controllers::user::update_user)
            .service(api::controllers::user::get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
