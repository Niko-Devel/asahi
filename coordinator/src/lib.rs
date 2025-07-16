use {
  asahi_internal::{
    AsahiError,
    AsahiResult
  },
  std::time::Duration,
  tokio::{
    task::JoinHandle,
    time::interval
  }
};

pub use async_trait::async_trait;

#[async_trait]
pub trait AsahiCoordinator: Send + Sync {
  fn name(&self) -> &'static str;
  /// Loop every X seconds
  fn interval(&self) -> u64;
  /// Asynchronous code inside the loop
  async fn main_loop(&self) -> AsahiResult<()>;
}

/// Spawn and run the logic in background on a timer
pub fn spawn<T>(task: T) -> JoinHandle<()>
where
  T: AsahiCoordinator + 'static
{
  tokio::spawn(async move {
    let mut interval = interval(Duration::from_secs(task.interval()));

    loop {
      interval.tick().await;
      if let Err(e) = task.main_loop().await {
        let _ = AsahiError::Worker(format!("[{}] {e}", task.name()).into());
      }
    }
  })
}
