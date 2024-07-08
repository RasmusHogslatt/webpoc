use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod api;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    db::init_db().expect("Failed to initialize database");

    HttpServer::new(|| {
        App::new()
            .service(api::auth::login)
            .service(api::auth::register)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
