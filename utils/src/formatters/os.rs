use std::{
  fs::File,
  io::{
    BufRead,
    BufReader
  },
  path::Path
};

pub fn get_os_info() -> String {
  let path = Path::new("/etc/os-release");
  let mut name = "BoringOS".to_string();
  let mut version = "v0.0".to_string();

  if let Ok(file) = File::open(path) {
    let reader = BufReader::new(file);
    let set_value = |s: String| s.split('=').nth(1).unwrap_or_default().trim_matches('"').to_string();
    reader.lines().map_while(Result::ok).for_each(|line| match line {
      l if l.starts_with("NAME=") => name = set_value(l),
      l if l.starts_with("VERSION=") => version = set_value(l),
      l if l.starts_with("VERSION_ID=") => version = set_value(l),
      _ => {}
    });
  }

  format!("{name} {version}")
}

pub fn get_kernel_info() -> String {
  let path = Path::new("/proc/version");
  let mut kern_info = "Unsupported kernel".to_string();

  if let Ok(file) = File::open(path) {
    let mut reader = BufReader::new(file);
    let mut content = String::new();

    if reader.read_line(&mut content).is_ok()
      && let Some(version) = content.split_whitespace().nth(2)
    {
      kern_info = version.to_string();
    }
  }

  kern_info
}

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
