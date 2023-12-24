use std::u8;

use axum::{extract, Json};
use serde_json::{json, Value};

const DEFAULT_PASSWORD: &str = "defaultpassword";

pub async fn post_signup(extract::Json(payload): extract::Json<super::PostSignup>) -> Json<Value> {
    Json(json!({ "message": "success", "data": payload }))
}

pub async fn post_auth(extract::Json(payload): extract::Json<super::AuthLogin>) -> Json<Value> {
    let pass = match &payload.password {
        Some(val) => val.to_string(),
        _ => DEFAULT_PASSWORD.to_string(),
    };

    Json(json!({ "message": "success", "username": payload.username, "password": pass}))
}

pub async fn get_me() -> Json<Value> {
    Json(json!({ "message": "success"}))
}

fn ret_valu() -> &'static str {
    "perfect score"
}
