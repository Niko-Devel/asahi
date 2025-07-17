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
      None => return Err(AsahiError::Network("asahi_http: url has no host, problematic".to_string().into()))
    };

    let port = url.port_u16().unwrap_or(80);

    let stream = match time::timeout(time::Duration::from_secs(connect_timeout), TcpStream::connect(format!("{host}:{port}"))).await {
      Ok(Ok(s)) => s,
      Ok(Err(e)) => return Err(AsahiError::Network(format!("asahi_http: connection error: {e}").into())),
      Err(_) => return Err(AsahiError::Network("asahi_http: connection timed out".to_string().into()))
    };

    let io = TokioIo::new(stream);

    let (sender, connection) = http1::handshake(io).await.unwrap();
    tokio::spawn(async move {
      if let Err(e) = connection.await {
        let _ = AsahiError::Network(format!("asahi_http: handshake failed: {e}").into());
      }
    });

    Ok(Self { sender })
  }

  pub async fn send(
    &mut self,
    req: Request<Empty<Bytes>>
  ) -> AsahiResult<Response<Incoming>> {
    self.sender.send_request(req).await.map_err(|e| AsahiError::Network(e.to_string().into()))
  }
}
