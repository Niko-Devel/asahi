use {
  asahi_internal::{
    AsahiError,
    AsahiResult
  },
  async_trait::async_trait,
  std::{
    sync::Arc,
    time::Duration
  },
  tokio::time::interval
};

#[async_trait]
pub trait AsahiCoordinator<C: Send + Sync>: Send + Sync {
  fn name(&self) -> &'static str;
  fn interval(&self) -> u64;
  async fn main_loop(
    &self,
    ctx: Arc<C>
  ) -> AsahiResult<()>;
}

/// Spawn and run the logic in background on a timer
pub fn spawn<T, C>(
  task: T,
  ctx: Arc<C>
) where
  T: AsahiCoordinator<C> + 'static,
  C: Send + Sync + 'static
{
  let name = task.name().to_string();
  let int = task.interval();

  tokio::spawn(async move {
    let mut interval = interval(Duration::from_secs(int));

    loop {
      interval.tick().await;
      if let Err(e) = task.main_loop(Arc::clone(&ctx)).await {
        let err = AsahiError::Worker(format!("[{name}] {e}"));
        let _ = err;
      }
    }
  });
}
