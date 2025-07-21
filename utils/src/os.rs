use {
  std::{
    fs::File,
    io::{
      BufRead,
      BufReader
    },
    path::Path,
    time::{
      Duration,
      SystemTime,
      UNIX_EPOCH
    }
  },
  sysinfo::{
    System,
    get_current_pid
  },
  uptime_lib::get
};

pub struct Memory {
  pub system:  Sysmem,
  pub process: u64
}

pub struct Sysmem {
  pub used:  u64,
  pub total: u64
}

pub struct Uptime {
  pub system:  u64,
  pub process: u64
}

fn system_() -> System {
  let mut system = System::new_all();
  system.refresh_all();
  system
}

/// Fetches the processor's name, e.g 'Cortex-A76'
pub fn get_cpu_info() -> String {
  let sys = system_();
  let cpu = sys.cpus().first().expect("No processor data");
  cpu.brand().to_string()
}

/// Fetches the OS info, e.g 'AlmaLinux 10.0'
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

/// Returns the kernel version, e.g '6.8.0-1020-raspi'
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

/// Fetches the memory usage from both the system and process<br>
/// Process memory returns zero when couldn't be fetched
pub fn get_memory() -> Memory {
  let sys = system_();

  let used = sys.used_memory();
  let total = sys.total_memory();

  let process = match sys.process(get_current_pid().expect("Expected PID to be present!")) {
    Some(p) => p.memory(),
    None => 0u64
  };

  Memory {
    system: Sysmem { used, total },
    process
  }
}

/// Fetches the uptime from both the system and process
pub fn get_uptime() -> Uptime {
  let sys = system_();
  let mut process = 0;
  let now = SystemTime::now();

  let system = get().map_or(0, |u| u.as_secs());
  if let Some(proc) = sys.process(get_current_pid().expect("Expected PID to be present!")) {
    let started = UNIX_EPOCH + Duration::from_secs(proc.start_time());
    process = now.duration_since(started).expect("Time went on an adventure!").as_secs()
  };

  Uptime { system, process }
}
