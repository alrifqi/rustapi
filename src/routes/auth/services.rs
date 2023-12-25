use std::u8;

use axum::{extract, Json};
use serde_json::{json, Value};

const DEFAULT_PASSWORD: &str = "defaultpassword";

pub async fn post_signup(extract::Json(payload): extract::Json<super::PostSignup>) -> Json<Value> {
    Json(json!({ "message": "success", "data": payload }))
}

pub async fn post_auth(extract::Json(payload): extract::Json<super::AuthLogin>) -> Json<Value> {
    let validation = payload.validate();
    if !validation.message.is_empty() {
        return Json(json!({ "message": validation.message }));
    }
    Json(json!({ "message": "success", "data": validation.payload }))
}

pub async fn get_me() -> Json<Value> {
    Json(json!({ "message": "success"}))
}
