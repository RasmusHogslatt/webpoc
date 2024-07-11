use crate::db;
use actix_web::{post, web, HttpResponse, Responder};
use serde_json;
use shared::{User, UserData};

#[post("/api/login")]
pub async fn login(user: web::Json<User>) -> impl Responder {
    match db::verify_user(&user.username, &user.password) {
        Ok(Some(user_data)) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "user_data": user_data
        })),
        Ok(None) => HttpResponse::Unauthorized().json(serde_json::json!({
            "status": "error",
            "message": "Invalid credentials"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Database error"
        })),
    }
}

#[post("/api/register")]
pub async fn register_user(user: web::Json<User>) -> impl Responder {
    match db::add_user(&user.username, &user.password, user.email.as_deref()) {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": "User registered successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": format!("Failed to register user: {:?}", e)
        })),
    }
}

#[post("/api/update_user_data")]
pub async fn update_user_data(user_data: web::Json<UserData>) -> impl Responder {
    match db::update_user_data(&user_data.username, &user_data) {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": "User data updated successfully"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Failed to update user data"
        })),
    }
}
