#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error("Config error: {0}")]
    ConfigError(#[source] config::ConfigError),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[source] reqwest::Error),
    #[error("IO error: {0}")]
    IoError(#[source] std::io::Error),
    #[error("Toml error de: {0}")]
    TomlErrorDe(#[source] toml::de::Error),
    #[error("Toml error ser: {0}")]
    TomlErrorSer(#[source] toml::ser::Error),
    #[error("{0}")]
    CustomError(String),
}
