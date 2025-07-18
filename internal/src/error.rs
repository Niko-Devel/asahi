use std::borrow::Cow;

pub type AsahiResult<T> = Result<T, AsahiError>;

#[derive(Debug, thiserror::Error)]
pub enum AsahiError {
  #[error("(Asahi) Configuration error: {0}")]
  /// Config error that stems from invalid configuration setup and etc
  Config(Cow<'static, str>),

  #[error("(Asahi) Network error: {0}")]
  /// Network related errors such as reqwest, hyper, etc
  Network(Cow<'static, str>),

  #[error("(Asahi) Worker error: {0}")]
  /// Coordinator's worker encounters an error
  Worker(Cow<'static, str>),

  #[error("(Asahi) Parsing error: {0}")]
  /// Parsing error, commonly used for failed conversions and etc
  Parse(Cow<'static, str>),

  #[error("(Asahi) Database error: {0}")]
  /// Database error via sqlx or any related crate
  Database(Cow<'static, str>),

  #[error("(Asahi) External error: {0}")]
  /// Userland error that can't be mapped to other [AsahiError] variant, or user's custom error
  External(Cow<'static, str>),

  #[error("(Asahi) Unknown error")]
  /// Unknown error type
  Unknown
}

macro_rules! impl_from_error {
  ($($source_type:ty => $destination_variant:ident),* $(,)?) => {
    $(
      impl From<$source_type> for AsahiError {
        fn from(error: $source_type) -> Self {
          AsahiError::$destination_variant(Cow::Owned(error.to_string()))
        }
      }
    )*
  };
}

impl_from_error!(
  Box<dyn std::error::Error + Send + Sync> => External,
  std::time::SystemTimeError => External,
  serde_json::Error => Parse,
  serde_xml_rs::Error => Parse,
  sqlx::Error => Database,
  bb8_redis::redis::RedisError => Database,
  hyper::Error => Network,
  reqwest::Error => Network
);
