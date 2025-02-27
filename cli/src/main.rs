//! Nexa Gateway CLI entry point
//!
//! This is the main entry point for the Nexa Gateway CLI application.

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use console::Term;
use std::path::PathBuf;
use core::status as core_status;
use core::config;

mod dashboard; // Make sure to include the dashboard module
mod configure;
mod status; // Our local status module

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
                DashboardCmd::Monitor { duration } => {
                    // New command to show real-time dashboard
                    dashboard::display_metrics_for_duration(duration).await?;
                },
            }
        },
        None => {
            // No command specified, show interactive menu with metrics
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
    
    /// Monitor system metrics in real-time
    Monitor {
        /// Duration in seconds to monitor (default: run until Ctrl+C)
        #[clap(short, long, default_value = "3600")]
        duration: u64,
    },
}

/// Display system status
async fn display_status() -> Result<()> {
    println!("{}", "Nexa Gateway Status".bold().green());
    println!("─────────────────────────────────");
    
    // Show system metrics
    let metrics = core_status::get_system_metrics()?;
    println!("CPU Usage: {}%", metrics.cpu_usage);
    println!("Memory Usage: {} GB", metrics.memory_usage_gb);
    println!("Uptime: {}", metrics.uptime);
    println!("Active Connections: {}", metrics.active_connections);
    println!("Requests per Second: {:.2}", metrics.requests_per_second);
    println!("Tokens per Second: {:.2}", metrics.tokens_per_second);
    
    // Add LLM settings display
    let llm_settings = config::get_llm_provider_settings().await?;
    println!("\n{}", "LLM Configuration".bold().blue());
    println!("─────────────────────────────────");
    println!("Provider: {}", llm_settings.provider_name);
    println!("Model: {}", llm_settings.model);
    println!("URL: {}", llm_settings.url);
    
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
    
    // Main menu options without numerical prefixes
    let selections = vec![
        "System Status",
        "Manage AI Agents (not implemented)",
        "Agent Orchestration (not implemented)",
        "View Execution Logs (not implemented)",
        "Configure Platform",
        "Select LLM Model",
        "Dashboard Management",
        "Backup/Restore (not implemented)",
        "Exit"
    ];

    loop {
        term.clear_screen()?;
        
        // Display title
        println!("╔══════════════════════════════════╗");
        println!("║      NEXA AI ORCHESTRATION       ║");
        println!("╚══════════════════════════════════╝");
        
        // Create a dashboard and display initial metrics
        let mut dashboard = dashboard::Dashboard::new();
        dashboard.update().await?;
        dashboard.display();
        
        println!("\nSelect an option:");
        
        // Use Select widget for arrow key navigation
        let selection = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Menu Options")
            .items(&selections)
            .default(0)
            .interact_on(&Term::stderr())?;
        
        // Handle selection based on index
        match selection {
            0 => {
                display_status().await?;
                wait_for_key_press()?;
            },
            4 => {
                // Call the previously unused configuration functions
                configure::run_configuration().await?;
                wait_for_key_press()?;
            },
            5 => {
                status::select_llm_model().await?;
                wait_for_key_press()?;
            },
            6 => {
                // Show dashboard management options
                let dashboard_options = vec![
                    "Show Real-time Dashboard",
                    "List Saved Dashboards",
                    "Back to Main Menu"
                ];
                
                println!("\nDashboard Management:");
                
                // Use Select widget for dashboard options
                let dashboard_choice = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
                    .with_prompt("Select option")
                    .items(&dashboard_options)
                    .default(0)
                    .interact_on(&Term::stderr())?;
                
                match dashboard_choice {
                    0 => {
                        // Display metrics for 30 seconds
                        dashboard::display_metrics_for_duration(30).await?;
                    },
                    1 => {
                        list_dashboards().await?;
                        wait_for_key_press()?;
                    },
                    _ => { /* Back to main menu */ }
                }
            },
            8 => {
                println!("Exiting Nexa Gateway...");
                break;
            },
            _ => {
                println!("Feature not yet implemented or invalid option.");
                wait_for_key_press()?;
            }
        }
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
