use axum::{Json, extract };
use serde::{ Deserialize };
use serde_json::{json, Value };
use sqlx::query;
use crate::database;

const DEFAULT_PASSWORD: &str = "defaultpassword";

#[derive(Debug, Deserialize)]
pub struct AuthLogin {
    username: String,
    password: Option<String>,
}

pub async fn post_auth(extract::Json(payload): extract::Json<AuthLogin>) -> Json<Value>{
    let pass = match &payload.password {
        Some(val) => val.to_string(),
        _ => DEFAULT_PASSWORD.to_string(),
    };

    let conn = database::init_connection().await;
    query!(r#"select from user where username=$1"#, pass.username).execute(&conn).await?;

    Json(json!({ "message": "success", "username": payload.username, "password": pass}))
}