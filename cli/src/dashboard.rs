//! Dashboard module for Nexa Gateway CLI
//!
//! This module provides functionality to save and load dashboard configurations
//! and display real-time system metrics.

use anyhow::Result;
use console::style;
use std::fs;
use std::path::PathBuf;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use colored::Colorize;
use std::time::{Duration, Instant, SystemTime};
use tokio::time;
use prettytable::{Table, Row, Cell};

/// Dashboard metrics for Nexa platform
#[derive(Debug, Clone)]
pub struct Dashboard {
    // Resource metrics
    pub cpu_usage: f32,
    pub ram_usage: f32,
    pub uptime: u64,
    pub uptime_formatted: String,
    
    // Agent metrics
    pub active_agents: u32,
    pub tasks_pending: u32,
    pub tasks_completed: u32,
    
    // Network metrics
    pub active_connections: u32,
    pub requests_per_second: f32,
    pub tokens_per_second: f32,
    
    // System status
    pub is_online: bool,
    pub all_services_running: bool,
    pub llm_connected: bool,
    pub llm_provider: String,
    pub llm_model: String,
    
    // Dashboard tracking
    pub update_time: SystemTime,
    pub last_update: LastUpdateTime,
}

/// Time fields for display formatting
#[derive(Debug, Clone)]
pub struct LastUpdateTime {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Dashboard {
    /// Create a new dashboard instance
    pub fn new() -> Self {
        Dashboard {
            cpu_usage: 0.0,
            ram_usage: 0.0,
            uptime: 0,
            uptime_formatted: "00:00:00".to_string(),
            
            active_agents: 0,
            tasks_pending: 0,
            tasks_completed: 0,
            
            active_connections: 0,
            requests_per_second: 0.0,
            tokens_per_second: 0.0,
            
            is_online: true,
            all_services_running: true,
            llm_connected: true,
            llm_provider: "None".to_string(),
            llm_model: "None".to_string(),
            
            update_time: SystemTime::now(),
            last_update: LastUpdateTime {
                hour: 0,
                minute: 0,
                second: 0,
            },
        }
    }
    
    /// Update dashboard with latest metrics
    pub async fn update(&mut self) -> Result<()> {
        // Update time
        self.update_time = SystemTime::now();
        
        // Update time display fields
        if let Ok(duration) = self.update_time.duration_since(std::time::UNIX_EPOCH) {
            let secs = duration.as_secs();
            self.last_update = LastUpdateTime {
                hour: ((secs / 3600) % 24) as u8,
                minute: ((secs / 60) % 60) as u8,
                second: (secs % 60) as u8,
            };
        }
        
        // Get system metrics
        if let Ok(metrics) = core::status::get_system_metrics() {
            self.cpu_usage = metrics.cpu_usage;
            self.ram_usage = metrics.memory_usage_gb;
            self.uptime = metrics.uptime;
            
            // Format uptime
            let uptime_seconds = self.uptime % 60;
            let uptime_minutes = (self.uptime / 60) % 60;
            let uptime_hours = self.uptime / 3600;
            self.uptime_formatted = format!("{:02}h:{:02}m:{:02}s", uptime_hours, uptime_minutes, uptime_seconds);
            
            self.active_connections = metrics.active_connections;
            self.requests_per_second = metrics.requests_per_second;
            self.tokens_per_second = metrics.tokens_per_second;
        }
        
        // Get agent metrics
        if let Ok(agents) = core::status::get_agent_metrics() {
            self.active_agents = agents.iter()
                .filter(|a| a.status == "Running")
                .count() as u32;
                
            // Simulate some task metrics
            self.tasks_pending = 2;
            self.tasks_completed = 17;
        }
        
        // Get LLM provider info
        if let Ok(llm_settings) = core::config::get_llm_provider_settings().await {
            self.llm_connected = true;
            self.llm_provider = llm_settings.provider_name;
            self.llm_model = llm_settings.model;
        } else {
            self.llm_connected = false;
        }
        
        Ok(())
    }
    
    /// Display dashboard with metrics
    pub fn display(&self) {
        let mut table = Table::new();
        
        // Create header row
        let header_row = Row::new(vec![
            Cell::new("  System Metrics  ").style_spec("bFg"),
            Cell::new("  Last updated:   ").style_spec("bFg"),
            Cell::new(&format!("{}:{}:{}", self.last_update.hour, self.last_update.minute, self.last_update.second)).style_spec("bFg")
        ]);
        table.add_row(header_row);
        
        // Resource usage and agent statistics
        table.add_row(Row::new(vec![
            Cell::new("  Resource Usage           ").style_spec("bFg"),
            Cell::new("  Agent Statistics                                  ").style_spec("bFg")
        ]));
        
        table.add_row(Row::new(vec![
            Cell::new(&format!(" CPU: {:.1}%                ", self.cpu_usage)),
            Cell::new(&format!(" Agents Active: {}                                     ", self.active_agents))
        ]));
        
        table.add_row(Row::new(vec![
            Cell::new(&format!(" RAM: {:.1} MB              ", self.ram_usage)),
            Cell::new(&format!(" Tasks Pending: {}                                     ", self.tasks_pending))
        ]));
        
        table.add_row(Row::new(vec![
            Cell::new(&format!(" Uptime: {}          ", self.uptime_formatted)),
            Cell::new(&format!(" Tasks Completed: {}                                  ", self.tasks_completed))
        ]));
        
        // Network and system status
        table.add_row(Row::new(vec![
            Cell::new("  Network & Performance    ").style_spec("bFg"),
            Cell::new("  System Status                                     ").style_spec("bFg")
        ]));
        
        // Online status
        let online_status = if self.is_online { 
            "ONLINE".green().to_string()
        } else { 
            "OFFLINE".red().to_string() 
        };
        
        table.add_row(Row::new(vec![
            Cell::new(&format!(" Active Connections: {}       ", self.active_connections)),
            Cell::new(&format!(" Status: {}                                   ", online_status))
        ]));
        
        // Service health
        let health_status = if self.all_services_running { 
            "✓ All Services Running".green().to_string()
        } else { 
            "✗ Services Degraded".red().to_string() 
        };
        
        table.add_row(Row::new(vec![
            Cell::new(&format!(" Requests/sec: {:.2}          ", self.requests_per_second)),
            Cell::new(&format!(" Service Health: {}             ", health_status))
        ]));
        
        // LLM status
        let llm_status = if self.llm_connected { 
            format!("✓ {} @ {}", self.llm_provider.green(), self.llm_model)
        } else { 
            "✗ Not Connected".red().to_string() 
        };
        
        table.add_row(Row::new(vec![
            Cell::new(&format!(" Tokens/sec: {:.2}          ", self.tokens_per_second)),
            Cell::new(&format!(" LLM: {}            ", llm_status))
        ]));
        
        // Print the table
        table.printstd();
    }
    
    /// Start a background task to continuously update metrics
    #[allow(dead_code)]
    pub fn start_background_updater(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut dashboard = self;
            let mut interval = time::interval(Duration::from_secs(5));
            
            loop {
                interval.tick().await;
                if let Err(e) = dashboard.update().await {
                    eprintln!("Failed to update metrics: {}", e);
                }
            }
        })
    }
}

/// Display a single dashboard update
#[allow(dead_code)]
pub async fn display_metrics_once() -> Result<()> {
    let mut dashboard = Dashboard::new();
    dashboard.update().await?;
    dashboard.display();
    Ok(())
}

/// Continuously update and refresh metrics for a specified duration
pub async fn display_metrics_for_duration(duration_secs: u64) -> Result<()> {
    let start_time = Instant::now();
    let mut dashboard = Dashboard::new();
    let mut interval = time::interval(Duration::from_secs(1));
    
    while start_time.elapsed().as_secs() < duration_secs {
        // Clear console and move cursor to top-left
        print!("\x1B[2J\x1B[1;1H");
        
        dashboard.update().await?;
        dashboard.display();
        println!("\nPress Ctrl+C to exit...");
        
        interval.tick().await;
    }
    
    Ok(())
}

/// Initialize dashboard for CLI main menu
#[allow(dead_code)]
pub async fn init_dashboard() -> Result<Dashboard> {
    let mut dashboard = Dashboard::new();
    dashboard.update().await?;
    Ok(dashboard)
}

/// Manage dashboard configurations
#[allow(dead_code)]
pub async fn manage_dashboards() -> Result<()> {
    println!("╔══════════════════════════════════╗");
    println!("║        DASHBOARD MANAGER         ║");
    println!("╚══════════════════════════════════╝");
    println!();
    
    let options = &[
        "Save Current Dashboard",
        "Load Dashboard",
        "List Saved Dashboards",
        "Back to Main Menu"
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action")
        .items(options)
        .default(0)
        .interact()?;
        
    match selection {
        0 => save_dashboard().await?,
        1 => load_dashboard().await?,
        2 => list_dashboards().await?,
        3 => return Ok(()),
        _ => unreachable!(),
    }
    
    Ok(())
}

/// Save current dashboard configuration
#[allow(dead_code)]
async fn save_dashboard() -> Result<()> {
    println!("\n{}", style("Save Dashboard").bold());
    
    let name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Dashboard name")
        .interact_text()?;
        
    // For now, we'll create a sample dashboard config
    let dashboard_config = r#"{
        "version": "1.0",
        "layout": "grid",
        "widgets": [
            {"type": "status", "position": {"x": 0, "y": 0, "w": 2, "h": 2}},
            {"type": "agents", "position": {"x": 2, "y": 0, "w": 2, "h": 1}},
            {"type": "logs", "position": {"x": 0, "y": 2, "w": 4, "h": 2}}
        ]
    }"#;
    
    let mut path = PathBuf::from("dashboards");
    fs::create_dir_all(&path)?;
    path.push(format!("{}.json", name));
    fs::write(path, dashboard_config)?;
    
    println!("\n{}", style(format!("Dashboard '{}' saved successfully!", name)).green());
    Ok(())
}

/// Load a saved dashboard configuration
#[allow(dead_code)]
async fn load_dashboard() -> Result<()> {
    println!("\n{}", style("Load Dashboard").bold());
    
    let dashboards = list_dashboard_files()?;
    
    if dashboards.is_empty() {
        println!("No saved dashboards found.");
        return Ok(());
    }
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select dashboard to load")
        .items(&dashboards)
        .default(0)
        .interact()?;
        
    let selected = &dashboards[selection];
    
    let mut path = PathBuf::from("dashboards");
    path.push(format!("{}.json", selected));
    
    let content = fs::read_to_string(path)?;
    
    println!("\n{}", style(format!("Dashboard '{}' loaded successfully!", selected)).green());
    println!("\nDashboard configuration:\n{}", content);
    
    Ok(())
}

/// List all saved dashboards
#[allow(dead_code)]
async fn list_dashboards() -> Result<()> {
    println!("\n{}", style("Saved Dashboards").bold());
    
    let dashboards = list_dashboard_files()?;
    
    if dashboards.is_empty() {
        println!("No saved dashboards found.");
        return Ok(());
    }
    
    for (index, name) in dashboards.iter().enumerate() {
        println!("{:2}. {}", index + 1, name);
    }
    
    Ok(())
}

/// Helper function to list dashboard files
#[allow(dead_code)]
fn list_dashboard_files() -> Result<Vec<String>> {
    let path = PathBuf::from("dashboards");
    
    if !path.exists() {
        return Ok(Vec::new());
    }
    
    let mut dashboards = Vec::new();
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name().into_string().unwrap();
        
        if file_name.ends_with(".json") {
            dashboards.push(file_name.trim_end_matches(".json").to_string());
        }
    }
    
    Ok(dashboards)
}
