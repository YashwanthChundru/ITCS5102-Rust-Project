use axum::{routing::post, Router, Json};
use serde::Deserialize;
use crate::db::Db;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};

#[derive(Deserialize)]
struct RegisterReq { email: String, password: String }

pub fn routes(_pool: Db) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(Json(req): Json<RegisterReq>) -> Json<serde_json::Value> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(req.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    Json(serde_json::json!({ "email": req.email, "password_hash": hash }))
}

async fn login(Json(_): Json<RegisterReq>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "token": "dev_jwt_placeholder" }))
}
