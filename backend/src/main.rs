mod api;
mod db;

use crate::api::auth;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use db::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_db().expect("Failed to initialize database");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(auth::login)
            .service(auth::register_user)
            .service(auth::update_user_data)
    })
    .bind("[::]:8080")?
    .run()
    .await
}
