/// Formats timestamp integer to Discord's timestamp format
pub fn format_timestamp(timestamp: i64) -> String { format!("<t:{timestamp}>\n<t:{timestamp}:R>") }

/// Formats large amount of seconds into readable format, e.g "8d, 5h, 32m, 19s"<br>
/// It will remain as seconds if not over 60
pub fn format_duration(secs: u64) -> String {
  let days = secs / 86400;
  let hours = (secs % 86400) / 3600;
  let minutes = (secs % 3600) / 60;
  let seconds = secs % 60;

  let components = [(days, "d"), (hours, "h"), (minutes, "m"), (seconds, "s")];

  let formatted_string: Vec<String> = components
    .iter()
    .filter(|&&(value, _)| value > 0)
    .map(|&(value, suffix)| format!("{value}{suffix}"))
    .collect();

  formatted_string.join(", ")
}

/// Formats the bytes into human-readable string, e.g '7.75 GB'<br>
/// Measured in binary, not decimal
pub fn format_bytes(bytes: u64) -> String {
  let units = ["B", "KB", "MB", "GB"];
  let mut bytes = bytes as f64;
  let mut unit = units[0];

  for &u in &units {
    if bytes < 1024.0 {
      unit = u;
      break;
    }
    bytes /= 1024.0;
  }

  format!("{bytes:.2} {unit}")
}
