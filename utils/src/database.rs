#[cfg(all(feature = "sqlx-pg", feature = "sqlx-sqlite"))]
compile_error!("You cannot enable both `sqlx-pg` and `sqlx-sqlite` features at the same time!");

pub trait AsahiDatabaseConfig: Send + Sync {
  /// Returns database connection string
  fn uri(&self) -> &str;

  /// Returns the SQLx-compatible database kind, e.g PostgreSQL, SQLite, etc..
  fn kind(&self) -> AsahiDatabaseKind;

  /// Maximum number of connections the pool should maintain
  fn max_connections(&self) -> u32;
}

/// Supported database types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsahiDatabaseKind {
  Postgres,
  Sqlite
}

macro_rules! impl_connect {
  ($feature:literal, $kind:ident, $pool:ty, $options:ty) => {
    #[cfg(feature = $feature)]
    pub async fn connect<C>(config: &C) -> asahi_internal::AsahiResult<$pool>
    where
      C: AsahiDatabaseConfig
    {
      if config.kind() != AsahiDatabaseKind::$kind {
        return Err(asahi_internal::AsahiError::Config(
          concat!("asahi_utils: this is not a ", stringify!($kind), " config").into()
        ));
      }

      <$options>::new()
        .max_connections(config.max_connections())
        .max_lifetime(Some(std::time::Duration::from_secs(600))) // 10min
        .idle_timeout(Some(std::time::Duration::from_secs(360))) // 6min
        .connect(config.uri())
        .await
        .map_err(|e| asahi_internal::AsahiError::Network(format!("asahi_utils: sqlx connection error: {e}").into()))
    }
  };
}

impl_connect!("sqlx-pg", Postgres, sqlx::PgPool, sqlx::postgres::PgPoolOptions);

impl_connect!("sqlx-sqlite", Sqlite, sqlx::SqlitePool, sqlx::sqlite::SqlitePoolOptions);
