pub mod ansi;
pub mod os;

pub fn format_timestamp(timestamp: i64) -> String { format!("<t:{timestamp}>\n<t:{timestamp}:R>") }

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
