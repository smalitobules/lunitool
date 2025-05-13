use sysinfo::{System, SystemExt, DiskExt};
use std::collections::HashMap;

/// System information 
#[derive(Debug)]
pub struct SystemInfo {
    /// Operating system name and version
    pub os_info: String,
    /// Kernel version
    pub kernel: String,
    /// System architecture
    pub architecture: String,
    /// Available disk space
    pub disk_space: String,
    /// Available RAM
    pub ram: String,
    /// Is this a live environment
    pub is_live: bool,
    /// Package manager
    pub package_manager: String,
    /// Additional system properties
    pub properties: HashMap<String, String>,
}

/// Collect system information
pub fn collect_system_info() -> SystemInfo {
    log::info!("Collecting system information...");
    
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Get OS information
    let os_info = format!(
        "{} {}",
        sys.name().unwrap_or_else(|| "Unknown".to_string()),
        sys.os_version().unwrap_or_else(|| "".to_string())
    );
    
    // Get kernel version
    let kernel = sys.kernel_version()
        .unwrap_or_else(|| "Unknown".to_string());
    
    // Get system architecture
    let architecture = std::env::consts::ARCH.to_string();
    
    // Get disk space
    let mut disk_space = "Unknown".to_string();
    if let Some(disk) = sys.disks().iter().find(|d| d.mount_point() == std::path::Path::new("/")) {
        let available = disk.available_space() as f64 / (1024.0 * 1024.0 * 1024.0); // Convert to GB
        disk_space = format!("{:.2} GB", available);
    }
    
    // Get available RAM
    let total_mem = sys.total_memory() as f64 / (1024.0 * 1024.0); // Convert to MB
    let used_mem = sys.used_memory() as f64 / (1024.0 * 1024.0);
    let available_mem = total_mem - used_mem;
    let ram = format!("{:.2} MB", available_mem);
    
    // Check if we're in a live environment
    let is_live = detect_live_environment();
    
    // Detect package manager
    let package_manager = detect_package_manager();
    
    // Create additional properties map
    let mut properties = HashMap::new();
    properties.insert("CPU Cores".to_string(), sys.cpus().len().to_string());
    properties.insert("Host Name".to_string(), sys.host_name().unwrap_or_else(|| "Unknown".to_string()));
    
    SystemInfo {
        os_info,
        kernel,
        architecture,
        disk_space,
        ram,
        is_live,
        package_manager,
        properties,
    }
}

/// Detect if we're running in a live environment
fn detect_live_environment() -> bool {
    use std::path::Path;
    use std::fs;
    
    // Common paths that indicate a live environment
    let live_paths = [
        Path::new("/run/live"),
        Path::new("/run/initramfs/live"),
    ];
    
    for path in &live_paths {
        if path.exists() {
            return true;
        }
    }
    
    // Check kernel command line for live boot parameters
    if let Ok(cmdline) = fs::read_to_string("/proc/cmdline") {
        if cmdline.contains("boot=live") {
            return true;
        }
    }
    
    false
}

/// Detect available package manager
fn detect_package_manager() -> String {
    use std::process::Command;
    
    // List of common package managers and their check commands
    let package_managers = [
        ("apt", "apt"),
        ("dnf", "dnf"),
        ("pacman", "pacman"),
        ("zypper", "zypper"),
    ];
    
    for (name, cmd) in &package_managers {
        if Command::new(cmd).arg("--version").output().is_ok() {
            return name.to_string();
        }
    }
    
    "unknown".to_string()
}