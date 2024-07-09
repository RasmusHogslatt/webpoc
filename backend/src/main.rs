mod api;
use actix_cors::Cors;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the database
    api::db::init_db().expect("Failed to initialize database");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(api::auth::login)
            .service(api::auth::register)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
