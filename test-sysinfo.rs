use sysinfo::{System, SystemExt, NetworkExt, Networks, NetworksExt};

fn main() {
    // Create a new System instance
    let mut sys = System::new_all();
    
    // Refresh all information
    sys.refresh_all();
    
    // Try to get the boot time (since boot time related to uptime)
    let boot_time = sys.boot_time();
    println!("Boot time (seconds since UNIX epoch): {}", boot_time);
    
    // Calculate uptime from current time and boot time
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let uptime = current_time - boot_time;
    println!("Calculated uptime (seconds): {}", uptime);
    
    // Format uptime nicely
    let hours = uptime / 3600;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;
    println!("Formatted uptime: {}h {}m {}s", hours, minutes, seconds);
    
    // Get CPU usage
    let cpu_usage = sys.global_cpu_usage();
    println!("CPU usage: {}%", cpu_usage);
    
    // Create a Networks instance to get network information
    let mut networks = Networks::new();
    
    // Refresh networks list
    networks.refresh_list();
    
    // Get network interfaces and their traffic
    println!("\nNetwork interfaces traffic:");
    for (interface_name, data) in networks.list() {
        println!(
            "{}: Received: {} bytes, Transmitted: {} bytes",
            interface_name,
            data.received(),
            data.transmitted()
        );
    }
    
    // Also try to access networks through the system object if available
    println!("\nAccessing networks through System object:");
    
    // Different ways the API might expose networks
    if let Some(system_networks) = sys.networks() {
        println!("sys.networks() is available");
        for (name, data) in system_networks {
            println!("{}: {}/{} bytes", name, data.received(), data.transmitted());
        }
    } else {
        println!("sys.networks() is not available");
    }
} 