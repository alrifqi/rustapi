use serde::{Deserialize, Serialize};
pub mod services;

#[derive(Debug)]
struct Validation {
    message: Vec<String>,
    is_valid: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthLogin {
    username: String,
    password: Option<String>,
}

impl AuthLogin {
    pub fn validate(&self) -> Vec<String> {
        let mut message = Vec::new();
        if self.username.is_empty() {
            message.push(String::from("username is mandatory"));
        }
        message
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostSignup {
    email: String,
    password: String,
}
