use crate::error::AsahiResult;

pub type BoxedAsahiExtension = Box<dyn AsahiExtension>;

/// Marker trait for framework's extension points
pub trait AsahiExtension: Send + Sync {
  fn setup(&self) -> AsahiResult<()> { Ok(()) }
}

pub trait AsahiInitializable {
  fn init(&self) -> impl std::future::Future<Output = AsahiResult<()>> + Send;
}

/// Middleware trait for request interception in HTTP or gRPC
pub trait AsahiMiddleware<Request, Response> {
  fn handle(
    &self,
    request: Request
  ) -> AsahiResult<Response>;
}

/// User-declared plugin
pub trait AsahiPlugin: Send + Sync {
  fn name(&self) -> &'static str;
  fn setup(&self) -> crate::AsahiResult<()>;
}
