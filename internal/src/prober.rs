use {
  serde::{
    Deserialize,
    Serialize
  },
  std::sync::Arc,
  tokio::sync::RwLock,
  warp::{
    Filter,
    Rejection,
    http::StatusCode,
    reply::{
      json,
      with_status
    }
  }
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
  pub status:    String,
  pub connected: bool
}

/// Kubernetes-ready health probe
#[derive(Debug, Clone)]
pub struct Probe {
  pub health: Arc<RwLock<Health>>
}

impl Default for Probe {
  fn default() -> Self { Self::new() }
}

impl Probe {
  pub fn new() -> Self {
    Self {
      health: Arc::new(RwLock::new(Health {
        status:    "Starting".to_string(),
        connected: false
      }))
    }
  }

  /// Updates the health status
  pub async fn update_status(
    &self,
    connected: bool
  ) {
    super::debug!("health endpoint updated; connected={connected}");

    let mut health = self.health.write().await;
    health.connected = connected;
    health.status = if connected { "Healthy".to_string() } else { "Unhealthy".to_string() }
  }

  /// Initializes the webserver for Kubernetes to consume for health probing
  pub async fn init(
    &self,
    port: u16
  ) {
    let health_prober = self.clone();
    let readiness_prober = self.clone();

    let health = warp::path("health").and(warp::get()).and_then(move || {
      let prober = health_prober.clone();
      async move {
        let status = prober.health.read().await;
        let status_code = if status.connected {
          StatusCode::NO_CONTENT
        } else {
          StatusCode::SERVICE_UNAVAILABLE
        };

        Ok::<_, Rejection>(with_status(json(&*status), status_code))
      }
    });

    let readiness = warp::path("ready").and(warp::get()).and_then(move || {
      let prober = readiness_prober.clone();
      async move {
        let status = prober.health.read().await;
        if status.connected {
          Ok::<_, Rejection>(with_status("Ready", StatusCode::OK))
        } else {
          Ok::<_, Rejection>(with_status("Not Ready", StatusCode::SERVICE_UNAVAILABLE))
        }
      }
    });

    warp::serve(health.or(readiness)).run(([0, 0, 0, 0], port)).await;
  }
}
