use tracing_subscriber::{
  EnvFilter,
  FmtSubscriber
};

#[macro_export]
macro_rules! debug {
  ($($arg:tt)*) => {
    $crate::tracing::debug!(target: module_path!(), $($arg)*)
  }
}

#[macro_export]
macro_rules! info {
  ($($arg:tt)*) => {
    $crate::tracing::info!(target: module_path!(), $($arg)*)
  }
}

#[macro_export]
macro_rules! warn {
  ($($arg:tt)*) => {
    $crate::tracing::warn!(target: module_path!(), $($arg)*)
  }
}

#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => {
    $crate::tracing::error!(target: module_path!(), $($arg)*)
  }
}

/// Initialize the tracing subscriber for framework itself<br>
/// Use `RUST_LOG` envvar to set a log level for specific crate(s)
pub fn log_init() {
  let sub = FmtSubscriber::builder()
    .compact()
    .with_ansi(true)
    .with_thread_names(false)
    .with_target(true)
    .with_file(false)
    .with_line_number(true)
    .with_env_filter(
      EnvFilter::from_default_env()
        .add_directive("tokio=warn".parse().unwrap())
        .add_directive("hyper=warn".parse().unwrap())
        .add_directive("hyper_util=warn".parse().unwrap())
        .add_directive("tower=warn".parse().unwrap())
        .add_directive("h2=warn".parse().unwrap())
        .add_directive("sqlx=warn".parse().unwrap())
    )
    .finish();

  tracing::subscriber::set_global_default(sub).expect("setting global subscriber failed");
}
