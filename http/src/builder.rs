use {
  asahi_internal::{
    AsahiError,
    AsahiResult
  },
  http_body_util::Empty,
  hyper::{
    Method,
    Request,
    Uri,
    body::Bytes,
    header::{
      HeaderMap,
      HeaderValue,
      USER_AGENT
    }
  }
};

pub struct AsahiHttpBuilder {
  method:  Method,
  uri:     Uri,
  headers: HeaderMap
}

impl AsahiHttpBuilder {
  pub fn get(uri: &str) -> AsahiResult<Self> {
    let uri: Uri = uri
      .parse()
      .map_err(|e| AsahiError::Network(format!("asahi_http_builder: invalid uri: {e}").into()))?;
    Ok(Self {
      method: Method::GET,
      uri,
      headers: HeaderMap::new()
    })
  }

  pub fn header(
    mut self,
    k: &'static str,
    v: &str
  ) -> Self {
    if let Ok(value) = HeaderValue::from_str(v) {
      self.headers.insert(k, value);
    }
    self
  }

  pub fn build(self) -> Request<Empty<Bytes>> {
    let mut builder = Request::builder().method(self.method).uri(self.uri);

    let headers = builder.headers_mut().unwrap();
    for (k, v) in self.headers.iter() {
      headers.insert(k, v.clone());
    }

    if !headers.contains_key(USER_AGENT) {
      headers.insert(USER_AGENT, HeaderValue::from_static("hyper/Asahi Framework"));
    }

    builder.body(Empty::new()).expect("asahi_http_builder: failed to build the request")
  }
}
