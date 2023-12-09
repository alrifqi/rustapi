use axum::{Json, extract };
use serde::{ Serialize, Deserialize };
use serde_json::{json, Value };

#[derive(Debug, Deserialize)]
pub struct AuthLogin {
    username: String,
    password: Option<String>,
}

pub async fn post_auth(extract::Json(payload): extract::Json<AuthLogin>) -> Json<Value>{
    println!("{}", payload.username);
    Json(json!({ "message": "success"}))
}