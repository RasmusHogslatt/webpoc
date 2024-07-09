use crate::db;
use actix_web::{post, web, HttpResponse, Responder};
use shared::User;

#[post("/api/login")]
pub async fn login(user: web::Json<User>) -> impl Responder {
    match db::verify_user(&user.username, &user.password) {
        Ok(true) => HttpResponse::Ok().body("Login successful"),
        Ok(false) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
    }
}

#[post("/api/register")]
pub async fn register(user: web::Json<User>) -> impl Responder {
    match db::add_user(&user.username, &user.password) {
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to register user"),
    }
}
