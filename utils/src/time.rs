use {
  asahi_internal::AsahiError,
  lazy_static::lazy_static,
  num_bigint::BigInt,
  num_traits::ToPrimitive,
  regex::Regex,
  std::{
    collections::HashMap,
    time::Duration
  }
};

lazy_static! {
  static ref NUMBER_RE: Regex = Regex::new(r"^(\d+)$").unwrap();
  static ref DURATION_RE: Regex = Regex::new(
    r"(?x)
      (?P<int>\d+)
      (?:\.(?P<dec>\d+))?
      (?:e(?P<exp>-?\d+))?
      \s*(?P<unit>[a-zA-Z]+)
    "
  )
  .unwrap();
  static ref UNIT_MAP: HashMap<&'static str, u64> = {
    let mut map = HashMap::new();

    let ns = 1u64;
    let us = 1_000 * ns;
    let ms = 1_000 * us;
    let s = 1_000 * ms;
    let m = 60 * s;
    let h = 60 * m;
    let d = 24 * h;
    let w = 7 * d;
    let mo = 2_629_746_000_000_000u64;
    let y = 31_556_952_000_000_000u64;

    let units = [
      (ns, &["ns", "nanosecond", "nanoseconds"][..]),
      (us, &["us", "microsecond", "microseconds"]),
      (ms, &["ms", "millisecond", "milliseconds"]),
      (s, &["s", "secs", "second", "seconds"]),
      (m, &["m", "mins", "min", "minute", "minutes"]),
      (h, &["h", "hr", "hrs", "hour", "hours"]),
      (d, &["d", "day", "days"]),
      (w, &["w", "wk", "wks", "week", "weeks"]),
      (mo, &["mo", "month", "months"]),
      (y, &["y", "year", "years"])
    ];

    for (value, names) in units {
      for &name in names {
        map.insert(name, value);
      }
    }

    map
  };
}

#[derive(Default)]
struct ProtoDuration {
  nanoseconds: BigInt
}

impl ProtoDuration {
  fn into_duration(self) -> Result<Duration, AsahiError> {
    self
      .nanoseconds
      .to_u64()
      .ok_or_else(|| AsahiError::Parse("duration too large".to_string().into()))
      .map(Duration::from_nanos)
  }
}

pub fn parse_duration(input: &str) -> Result<Duration, AsahiError> {
  if let Some(int) = NUMBER_RE.captures(input) {
    let value =
      BigInt::parse_bytes(int.get(1).unwrap().as_str().as_bytes(), 10).ok_or_else(|| AsahiError::Parse("invalid number".to_string().into()))?;
    return value
      .to_u64()
      .ok_or_else(|| AsahiError::Parse("number too large".to_string().into()))
      .map(Duration::from_secs);
  }

  if !DURATION_RE.is_match(input) {
    return Err(AsahiError::Parse("no value or unit found".to_string().into()));
  }

  let mut duration = ProtoDuration::default();

  for cap in DURATION_RE.captures_iter(input) {
    let unit = cap
      .name("unit")
      .ok_or_else(|| AsahiError::Parse("missing unit".to_string().into()))?
      .as_str();

    let multiplier = *UNIT_MAP
      .get(unit.to_lowercase().as_str())
      .ok_or_else(|| AsahiError::Parse(format!("unknown unit: {unit}").into()))?;

    let base = cap.name("int").ok_or_else(|| AsahiError::Parse("missing number".to_string().into()))?;
    let int = BigInt::parse_bytes(base.as_str().as_bytes(), 10).ok_or_else(|| AsahiError::Parse("invalid integer".to_string().into()))?;

    let mut total = int * BigInt::from(multiplier);

    if let Some(dec) = cap.name("dec") {
      let dec = BigInt::parse_bytes(dec.as_str().as_bytes(), 10).ok_or_else(|| AsahiError::Parse("invalid decimal".to_string().into()))?;
      let scale = BigInt::from(10).pow(dec.to_string().len() as u32);
      total += dec * BigInt::from(multiplier) / scale;
    }

    if let Some(exp) = cap.name("exp") {
      let exp: isize = exp
        .as_str()
        .parse()
        .map_err(|_| AsahiError::Parse("invalid exponent".to_string().into()))?;
      let factor = BigInt::from(10).pow(exp.unsigned_abs() as u32);
      total = if exp < 0 { total / factor } else { total * factor };
    }

    duration.nanoseconds += total;
  }

  duration.into_duration()
}
