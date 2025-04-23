#[cfg(feature = "config")]
use asahi_internal::AsahiConfigurable;

use {
  super::tokiort::TokioIo,
  asahi_internal::{
    AsahiError,
    AsahiResult
  },
  http_body_util::Empty,
  hyper::{
    Request,
    Response,
    Uri,
    body::{
      Bytes,
      Incoming
    },
    client::conn::http1
  },
  tokio::{
    net::TcpStream,
    time
  }
};

#[cfg(feature = "config")]
pub trait AsahiHttpConfiguration: AsahiConfigurable {
  fn url(&self) -> Uri;
  /// Connection timeout interval in seconds
  fn connect_timeout(&self) -> u64;
}

pub struct AsahiHttpConnection {
  sender: http1::SendRequest<Empty<Bytes>>
}

impl AsahiHttpConnection {
  pub async fn handshake(
    url: Uri,
    connect_timeout: u64
  ) -> AsahiResult<Self> {
    let host = match url.host() {
      Some(h) => h,
      None => return Err(AsahiError::Network("asahi_http: url has no host, problematic".to_string()))
    };

    let port = url.port_u16().unwrap_or(80);

    let stream = match time::timeout(time::Duration::from_secs(connect_timeout), TcpStream::connect(format!("{host}:{port}"))).await {
      Ok(Ok(s)) => s,
      Ok(Err(e)) => return Err(AsahiError::Network(format!("asahi_http: connection error: {e}"))),
      Err(_) => return Err(AsahiError::Network("asahi_http: connection timed out".to_string()))
    };

    let io = TokioIo::new(stream);

    let (sender, connection) = http1::handshake(io).await.unwrap();
    tokio::spawn(async move {
      if let Err(e) = connection.await {
        AsahiError::Network(format!("asahi_http: handshake failed: {e}"));
      }
    });

    Ok(Self { sender })
  }

  #[cfg(feature = "config")]
  pub async fn handshake_with_config<C>(config: C) -> AsahiResult<Self>
  where
    C: AsahiHttpConfiguration
  {
    let (url, connect_timeout) = { (config.url(), config.connect_timeout()) };
    Self::handshake(url, connect_timeout).await
  }

  pub async fn send(
    &mut self,
    req: Request<Empty<Bytes>>
  ) -> AsahiResult<Response<Incoming>> {
    self.sender.send_request(req).await.map_err(|e| AsahiError::Network(e.to_string()))
  }
}
