use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::env;
use std::env::VarError;

#[derive(Builder, Serialize, Deserialize, Clone)]
pub struct ApiConfig {
    pub db: String,
    pub salt: String,
    pub ip: String,
    pub port: String,
}

impl ApiConfig {
    pub fn load_from_env() -> Result<Self, VarError> {
        Ok(Self {
            db: env::var("DATABASE_URL")?,
            salt: env::var("API_SALT")?,
            ip: env::var("API_IP")?,
            port: env::var("API_PORT")?,
        })
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            db: "postgres://postgres:password@localhost/zero2prod".to_string(),
            salt: "salt".to_string(),
            ip: "127.0.0.1".to_string(),
            port: "8000".to_string(),
        }
    }
}
