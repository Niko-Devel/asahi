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

macro_rules! impl_from_error {
  ($($source_type:ty => $destination_variant:ident),* $(,)?) => {
    $(
      impl From<$source_type> for AsahiError {
        fn from(error: $source_type) -> Self {
          AsahiError::$destination_variant(error.to_string())
        }
      }
    )*
  };
}

impl_from_error!(
  Box<dyn std::error::Error + Send + Sync> => External,
  std::time::SystemTimeError => External,
  serde_json::Error => Parse,
  sqlx::Error => Database,
  bb8_redis::redis::RedisError => Database
);
