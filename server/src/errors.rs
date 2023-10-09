use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(Debug, thiserror::Error)]
pub enum ApiErrors {
    #[error("IO error: {0}")]
    IoError(#[source] std::io::Error),
    #[error("Database error: {0}")]
    Sqlx(#[source] sqlx::Error),
    #[error("Serialization error: {0}")]
    SerdeJson(#[source] serde_json::Error),
    #[error("Hashing error: {0}")]
    ArgonError(#[source] argon2::Error),
    #[error("Config error: {0}")]
    ConfigError(#[source] config::ConfigError),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[source] reqwest::Error),
    #[error("Search error: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Auth error: {0}")]
    AuthError(String),
    #[error("{0}")]
    CustomError(String),
}

impl ResponseError for ApiErrors {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiErrors::SerdeJson(_) => StatusCode::BAD_REQUEST,
            ApiErrors::IoError(_)
            | ApiErrors::Sqlx(_)
            | ApiErrors::CustomError(_)
            | ApiErrors::ConfigError(_)
            | ApiErrors::ArgonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrors::NotFound(_) => StatusCode::NOT_FOUND,
            ApiErrors::ReqwestError(_) => StatusCode::SERVICE_UNAVAILABLE,
            ApiErrors::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiErrors::AuthError(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
