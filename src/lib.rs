pub mod config;
pub mod database;
pub mod logger;
pub mod repository;
pub mod routes;
pub mod usecases;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthLogin {
    username: String,
    password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthLoginValidation {
    payload: AuthLogin,
    message: Vec<String>,
}

impl AuthLogin {
    pub fn validate(&self) -> AuthLoginValidation {
        let mut validation = AuthLoginValidation {
            payload: AuthLogin {
                username: self.username.clone(),
                password: match &self.password {
                    Some(v) => Some(v.clone()),
                    None => Some(String::from("")),
                },
            },
            message: vec![],
        };
        validation.message = vec![];
        if self.username.is_empty() {
            validation
                .message
                .push(String::from("username is mandatory"));
        }

        if self.password.is_none() {
            validation
                .message
                .push(String::from("password is mandatory"));
        }
        validation
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostSignup {
    email: String,
    password: String,
}
