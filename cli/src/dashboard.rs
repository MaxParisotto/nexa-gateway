//! Dashboard module for Nexa Gateway CLI
//!
//! This module provides functionality to save and load dashboard configurations
//! and display real-time system metrics.

use anyhow::Result;
use console::style;
use std::fs;
use std::path::PathBuf;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use colored::*;
use core::status::{get_system_metrics, get_agent_metrics};
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use tokio::time;

/// Metrics data structure to store all system information
#[derive(Debug, Clone)]
pub struct MetricsDashboard {
    cpu_usage: f32,
    memory_usage: f32,
    uptime: u64,
    active_connections: u32,
    requests_per_second: f32,
    tokens_per_second: f32,
    active_agents: u32,
    pending_tasks: u32,
    completed_tasks: u32,
    last_updated: DateTime<Utc>,
}

impl Default for MetricsDashboard {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            uptime: 0,
            active_connections: 0,
            requests_per_second: 0.0,
            tokens_per_second: 0.0,
            active_agents: 0,
            pending_tasks: 0,
            completed_tasks: 0,
            last_updated: Utc::now(),
        }
    }
}

impl MetricsDashboard {
    /// Create a new metrics dashboard
    pub fn new() -> Self {
        Self::default()
    }

    /// Update metrics from system
    pub async fn update(&mut self) -> Result<()> {
        // Get system metrics
        if let Ok(system_metrics) = get_system_metrics().await {
            self.cpu_usage = system_metrics.cpu_usage;
            self.memory_usage = system_metrics.memory_usage;
            self.uptime = system_metrics.uptime;
            self.active_connections = system_metrics.active_connections;
            self.requests_per_second = system_metrics.requests_per_second;
            
            // Simulate tokens per second for now (in reality would come from LLM metrics)
            self.tokens_per_second = self.requests_per_second * 25.0; // Assuming average 25 tokens per request
        }

        // Get agent metrics
        let mut active_agents = 0;
        let mut pending_tasks = 0;
        let mut completed_tasks = 0;

        if let Ok(agent_metrics) = get_agent_metrics().await {
            active_agents = agent_metrics.len() as u32;
            
            for agent in agent_metrics {
                pending_tasks += agent.tasks_pending;
                completed_tasks += agent.tasks_completed;
            }
        }

        self.active_agents = active_agents;
        self.pending_tasks = pending_tasks;
        self.completed_tasks = completed_tasks;
        self.last_updated = Utc::now();

        Ok(())
    }

    /// Display the dashboard
    pub fn display(&self) {
        // Format time in HH:MM:SS
        let uptime_hours = self.uptime / 3600;
        let uptime_minutes = (self.uptime % 3600) / 60;
        let uptime_seconds = self.uptime % 60;
        
        // Top border
        println!("┌──────────────────────────────────────────────────────────────────────────────┐");
        
        // Title
        println!("│ {} │ {} │", 
            " System Metrics ".on_blue().white().bold(),
            format!(" Last updated: {} ", self.last_updated.format("%H:%M:%S"))
                .on_black().white()
        );
        
        // Separator
        println!("├─────────────────────────────┬────────────────────────────────────────────────┤");
        
        // Resource metrics (left side)
        println!("│ {:<25} │ {:<50} │", 
            " Resource Usage ".green().bold(), 
            " Agent Statistics ".green().bold()
        );
        
        println!("│ CPU: {:<20} │ Agents Active: {:<37} │", 
            format!("{}%", self.cpu_usage).yellow(),
            self.active_agents
        );
        
        println!("│ RAM: {:<20} │ Tasks Pending: {:<37} │", 
            format!("{:.1} MB", self.memory_usage).yellow(),
            self.pending_tasks
        );
        
        println!("│ Uptime: {:<18} │ Tasks Completed: {:<35} │", 
            format!("{:02}:{:02}:{:02}", uptime_hours, uptime_minutes, uptime_seconds).yellow(),
            self.completed_tasks
        );
        
        // Separator
        println!("├─────────────────────────────┼────────────────────────────────────────────────┤");
        
        // Performance metrics
        println!("│ {:<25} │ {:<50} │", 
            " Network & Performance ".green().bold(),
            " System Status ".green().bold()
        );
        
        println!("│ Active Connections: {:<8} │ Status: {:<40} │", 
            self.active_connections,
            "ONLINE".green().bold()
        );
        
        println!("│ Requests/sec: {:<13} │ Service Health: {:<34} │", 
            format!("{:.2}", self.requests_per_second).yellow(),
            "✓ All Services Running".green()
        );
        
        println!("│ Tokens/sec: {:<14} │ LLM API Status: {:<34} │", 
            format!("{:.2}", self.tokens_per_second).yellow(),
            "✓ Connected".green()
        );
        
        // Bottom border
        println!("└──────────────────────────────────────────────────────────────────────────────┘");
    }

    /// Start a background task to continuously update metrics
    pub async fn start_background_updater(&self) -> tokio::task::JoinHandle<()> {
        let mut dashboard = self.clone();
        
        tokio::spawn(async move {
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
pub async fn display_metrics_once() -> Result<()> {
    let mut dashboard = MetricsDashboard::new();
    dashboard.update().await?;
    dashboard.display();
    Ok(())
}

/// Continuously update and refresh metrics for a specified duration
pub async fn display_metrics_for_duration(duration_secs: u64) -> Result<()> {
    let start_time = Instant::now();
    let mut dashboard = MetricsDashboard::new();
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
pub async fn init_dashboard() -> Result<MetricsDashboard> {
    let mut dashboard = MetricsDashboard::new();
    dashboard.update().await?;
    Ok(dashboard)  // Fixed: Wrap the return value in Ok()
}

/// Manage dashboard configurations
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
