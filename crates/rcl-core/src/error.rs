use thiserror::Error;

#[derive(Error, Debug)]
pub enum RclError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Option not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, RclError>;
