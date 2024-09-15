mod api;
mod db;

use crate::api::auth;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use db::*;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_db().expect("Failed to initialize database");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(
            "/etc/letsencrypt/live/rasmushogslatt.com/privkey.pem",
            SslFiletype::PEM,
        )
        .unwrap();

    builder
        .set_certificate_chain_file("/etc/letsencrypt/live/rasmushogslatt.com/fullchain.pem")
        .unwrap();

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
    .bind_openssl("[::]:443", builder)?
    .run()
    .await
}

//
