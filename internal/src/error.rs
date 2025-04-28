pub type AsahiResult<T> = Result<T, AsahiError>;

#[derive(Debug, thiserror::Error)]
pub enum AsahiError {
  #[error("(Asahi) Configuration error: {0}")]
  /// User's configuration causes a fatal error
  Config(String),

  #[error("(Asahi) Network error: {0}")]
  /// Network related errors such as reqwest, hyper, etc
  Network(String),

  #[error("(Asahi) Extension error: {0}")]
  /// Extension error caused by user's code
  Extension(String),

  #[error("(Asahi) Worker error: {0}")]
  /// Coordinator's worker encounters an error
  Worker(String),

  #[error("(Asahi) Parsing error: {0}")]
  /// Parsing error, commonly used for failed conversions and etc
  Parse(String),

  #[error("(Asahi) Database error: {0}")]
  /// Database error via sqlx or any related crate
  Database(String),

  #[error("(Asahi) External error: {0}")]
  /// Userland error that can't be mapped to other [AsahiError] variant, or user's custom error
  External(String),

  #[error("(Asahi) Unknown error")]
  /// Unknown error type
  Unknown
}

impl From<sqlx::Error> for AsahiError {
  fn from(error: sqlx::Error) -> Self { AsahiError::Database(error.to_string()) }
}

impl From<serde_json::Error> for AsahiError {
  fn from(error: serde_json::Error) -> Self { AsahiError::Parse(error.to_string()) }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AsahiError {
  fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self { AsahiError::External(error.to_string()) }
}
