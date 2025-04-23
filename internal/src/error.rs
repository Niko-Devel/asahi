pub type AsahiResult<T> = Result<T, AsahiError>;

#[derive(Debug, thiserror::Error)]
pub enum AsahiError {
  #[error("(Asahi) Configuration error: {0}")]
  Config(String),

  #[error("(Asahi) Network error: {0}")]
  Network(String),

  #[error("(Asahi) Serialization error: {0}")]
  Serialization(String),

  #[error("(Asahi) Extension error: {0}")]
  Extension(String),

  #[error("(Asahi) Worker error: {0}")]
  Worker(String),

  #[error("(Asahi) Parsing error: {0}")]
  Parse(String),

  #[error("(Asahi) Unknown error")]
  Unknown
}
