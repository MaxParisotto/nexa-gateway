//! Interactive menu for Nexa Gateway CLI

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select};
use console::Term;

// ...existing code...

/// View execution logs
async fn view_execution_logs() -> Result<()> {
    println!("╔══════════════════════════════════╗");
    println!("║        AGENT EXECUTION LOGS      ║");
    println!("╚══════════════════════════════════╝");
    println!();
    
    // Get log entries from the core (previously gateway)
    let log_entries = core::logs::get_recent_logs(50).await?;
    
    // ...existing code...
}

// ...existing code...
