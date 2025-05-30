use tracing_subscriber::FmtSubscriber;

#[macro_export]
macro_rules! debug {
  ($($arg:tt)*) => {
    $crate::tracing::debug!(target: "asahi", $($arg)*)
  }
}

#[macro_export]
macro_rules! info {
  ($($arg:tt)*) => {
    $crate::tracing::info!(target: "asahi", $($arg)*)
  }
}

#[macro_export]
macro_rules! warn {
  ($($arg:tt)*) => {
    $crate::tracing::warn!(target: "asahi", $($arg)*)
  }
}

#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => {
    $crate::tracing::error!(target: "asahi", $($arg)*)
  }
}

/// Initialize the tracing subscriber for framework itself
pub fn log_init() {
  let sub = FmtSubscriber::builder()
    .compact()
    .with_ansi(true)
    .with_thread_names(true)
    .with_target(true)
    .with_file(false)
    .with_line_number(true)
    .finish();

  tracing::subscriber::set_global_default(sub).expect("setting global subscriber failed");
}
