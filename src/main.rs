use sysinfo::{System, SystemExt, NetworkExt, Networks};

fn main() {
    // Create a new System instance
    let mut sys = System::new_all();
    
    // Refresh network information
    sys.refresh_networks();
    
    // Try to access networks in different ways
    println!("System methods for network:");
    
    // Try direct network access if available
    if let Some(networks) = sys.networks_mut() {
        println!("networks_mut() method is available");
        for (interface_name, network) in networks {
            println!(
                "{}: {}/{} B",
                interface_name,
                network.received(),
                network.transmitted()
            );
        }
    } else {
        println!("networks_mut() method returned None");
    }
    
    // Try accessing as immutable if available
    if let Some(networks) = sys.networks() {
        println!("\nnetworks() method is available");
        for (interface_name, network) in networks {
            println!(
                "{}: {}/{} B",
                interface_name,
                network.received(),
                network.transmitted()
            );
        }
    } else {
        println!("networks() method returned None");
    }
    
    // This will list all available methods on the System struct
    println!("\nSystem object type information: {:?}", std::any::type_name::<System>());
} 