//! Nexa Gateway CLI entry point
//!
//! This is the main entry point for the Nexa Gateway CLI application.

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use console::Term;
use std::path::PathBuf;

// Make sure the source directory exists
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Nexa Gateway CLI");
    
    // Parse command-line arguments
    let args = CliArgs::parse();
    
    match args.command {
        Some(Commands::Status) => {
            display_status().await?;
        },
        Some(Commands::Dashboard { command }) => {
            match command {
                DashboardCmd::Save { name, path } => {
                    save_dashboard(&name, &path).await?;
                },
                DashboardCmd::Load { name } => {
                    load_dashboard(&name).await?;
                },
                DashboardCmd::List => {
                    list_dashboards().await?;
                },
            }
        },
        None => {
            // No command specified, show interactive menu
            show_interactive_menu().await?;
        },
    }
    
    Ok(())
}

/// Command-line arguments for the Nexa Gateway CLI
#[derive(Parser)]
#[clap(name = "nexa", version = "0.1.0", about = "Nexa Gateway CLI")]
struct CliArgs {
    #[clap(subcommand)]
    command: Option<Commands>,
}

/// Subcommands for Nexa Gateway CLI
#[derive(Subcommand)]
enum Commands {
    /// Show system status
    Status,
    
    /// Manage dashboards
    Dashboard {
        #[clap(subcommand)]
        command: DashboardCmd,
    },
}

/// Dashboard subcommands
#[derive(Subcommand)]
enum DashboardCmd {
    /// Save a dashboard configuration
    Save {
        /// Dashboard name
        #[clap(short, long)]
        name: String,
        
        /// Path to dashboard configuration file
        #[clap(short, long)]
        path: PathBuf,
    },
    
    /// Load a dashboard configuration
    Load {
        /// Dashboard name
        #[clap(short, long)]
        name: String,
    },
    
    /// List saved dashboards
    List,
}

/// Display system status
async fn display_status() -> Result<()> {
    println!("{}", "Nexa Gateway Status".bold().green());
    println!("─────────────────────────────────");
    
    // Show system metrics
    let metrics = core::status::get_system_metrics().await?;
    println!("CPU Usage: {}%", metrics.cpu_usage);
    println!("Memory Usage: {} MB", metrics.memory_usage);
    println!("Uptime: {} seconds", metrics.uptime);
    println!("Active Connections: {}", metrics.active_connections);
    println!("Requests/sec: {:.2}", metrics.requests_per_second);
    
    Ok(())
}

/// Save a dashboard configuration
async fn save_dashboard(name: &str, path: &PathBuf) -> Result<()> {
    println!("Saving dashboard '{}' from {}", name, path.display());
    
    // Read the dashboard configuration
    let content = std::fs::read_to_string(path)?;
    
    // Create dashboards directory if it doesn't exist
    let mut dir = PathBuf::from("dashboards");
    std::fs::create_dir_all(&dir)?;
    
    // Save the dashboard
    dir.push(format!("{}.json", name));
    std::fs::write(dir, content)?;
    
    println!("{}", "Dashboard saved successfully!".green());
    Ok(())
}

/// Load a dashboard configuration
async fn load_dashboard(name: &str) -> Result<()> {
    let path = PathBuf::from("dashboards").join(format!("{}.json", name));
    
    if path.exists() {
        let content = std::fs::read_to_string(path)?;
        println!("Dashboard '{}' content:", name);
        println!("{}", content);
    } else {
        println!("{}", format!("Dashboard '{}' not found", name).red());
    }
    
    Ok(())
}

/// List saved dashboards
async fn list_dashboards() -> Result<()> {
    let dir = PathBuf::from("dashboards");
    
    if !dir.exists() {
        println!("No dashboards directory found.");
        return Ok(());
    }
    
    println!("{}", "Saved Dashboards:".bold());
    let mut found = false;
    
    for entry in std::fs::read_dir(dir)? {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".json") {
                    println!("- {}", filename.trim_end_matches(".json"));
                    found = true;
                }
            }
        }
    }
    
    if !found {
        println!("No dashboards found.");
    }
    
    Ok(())
}

/// Show the interactive CLI menu
async fn show_interactive_menu() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("╔══════════════════════════════════╗");
    println!("║      NEXA AI ORCHESTRATION       ║");
    println!("╚══════════════════════════════════╝");
    println!();
    
    let selections = &[
        "System Status",
        "Manage AI Agents",
        "Agent Orchestration",
        "View Execution Logs",
        "Configure Platform",
        "Dashboard Management",
        "Backup/Restore",
        "Exit",
    ];
    
    let selection = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .items(selections)
        .default(0)
        .interact_on(&term)?;
    
    match selection {
        0 => {
            display_status().await?;
            wait_for_key_press()?;
            Box::pin(show_interactive_menu()).await?;
        },
        7 => {
            println!("Exiting Nexa Gateway...");
        },
        _ => {
            println!("Feature not yet implemented.");
            wait_for_key_press()?;
            Box::pin(show_interactive_menu()).await?;
        },
    }
    
    Ok(())
}

/// Wait for a key press
fn wait_for_key_press() -> Result<()> {
    println!("\nPress any key to continue...");
    let term = Term::stdout();
    term.read_key()?;
    Ok(())
}
