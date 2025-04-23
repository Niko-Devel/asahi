pub trait AsahiConfigurable: Sized {
  /// Load variables from OS-level envvars or external sources
  fn from_env() -> crate::error::AsahiResult<Self>;

  /// Override the framework's values
  fn apply_overrides(
    &mut self,
    overrides: Self
  );
}
