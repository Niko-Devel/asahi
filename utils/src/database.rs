pub trait AsahiDatabaseConfig: Send + Sync {
  /// Returns database connection string
  fn uri(&self) -> &str;

  /// Returns the SQLx-compatible database kind, e.g PostgreSQL, SQLite, etc..
  fn kind(&self) -> AsahiDatabaseKind;
}

/// Supported database types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsahiDatabaseKind {
  Postgres,
  Sqlite
}

#[cfg(feature = "sqlx-pg")]
pub async fn connect_pg<C>(config: &C) -> asahi::AsahiResult<sqlx::PgPool>
where
  C: AsahiDatabaseConfig
{
  if config.kind() != AsahiDatabaseKind::Postgres {
    return Err(asahi::AsahiError::Config("asahi_utils: this is not a postgresql config".to_string()));
  }

  sqlx::PgPool::connect(config.uri())
    .await
    .map_err(|e| asahi::AsahiError::Network(format!("asahi_utils: sqlx connection error: {e}")))
}

#[cfg(feature = "sqlx-sqlite")]
pub async fn connect_sqlite<C>(config: &C) -> asahi::AsahiResult<sqlx::SqlitePool>
where
  C: AsahiDatabaseConfig
{
  if config.kind() != AsahiDatabaseKind::Sqlite {
    return Err(asahi::AsahiError::Config("asahi_utils: this is not a sqlite config".to_string()));
  }

  sqlx::SqlitePool::connect(config.uri())
    .await
    .map_err(|e| asahi::AsahiError::Network(format!("asahi_utils: sqlx connection error: {e}")))
}
