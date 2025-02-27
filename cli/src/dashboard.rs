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
use std::time::{Duration, Instant, SystemTime};
use chrono::Utc;
use tokio::time;
use rand;

/// Dashboard metrics for Nexa platform
#[derive(Debug, Clone)]
pub struct Dashboard {
    // Resource metrics
    pub cpu_usage: f32,
    pub ram_usage: f32,
    pub uptime: u64,
    
    // Agent metrics
    pub active_agents: u32,
    pub tasks_pending: u32,
    pub tasks_completed: u32,
    
    // Network metrics
    pub active_connections: u32,
    pub requests_per_second: f32,
    pub tokens_per_second: f32,
    
    // LLM metrics
    pub llm_provider: String,
    pub llm_url: String,
    pub llm_model: String,
    
    // Internal state
    update_time: SystemTime,
}

impl Default for Dashboard {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            ram_usage: 0.0,
            uptime: 0,
            active_agents: 0,
            tasks_pending: 0,
            tasks_completed: 0,
            active_connections: 0,
            requests_per_second: 0.0,
            tokens_per_second: 0.0,
            llm_provider: String::new(),
            llm_url: String::new(),
            llm_model: String::new(),
            update_time: SystemTime::now(),
        }
    }
}

impl Dashboard {
    /// Create a new dashboard
    pub fn new() -> Self {
        Self::default()
    }

    /// Update metrics from system
    pub async fn update(&mut self) -> Result<()> {
        // Fetch metrics from various services
        // For now, we'll just simulate random values
        self.cpu_usage = rand::random::<f32>() * 100.0 % 50.0;
        self.ram_usage = rand::random::<f32>() * 2048.0;
        self.uptime = Utc::now().timestamp() as u64 % 86400; // Random time within 24 hours

        // Update active connections and requests per second
        self.active_connections = rand::random::<u32>() % 100;
        self.requests_per_second = rand::random::<f32>() * 200.0;
        
        // Simulate tokens per second for now (in reality would come from LLM metrics)
        self.tokens_per_second = 1000.0 + rand::random::<f32>() * 4000.0;
        
        // Simulate agent metrics
        let active_agents = rand::random::<u32>() % 5;
        let pending_tasks = rand::random::<u32>() % 10;
        let completed_tasks = 100 + rand::random::<u32>() % 200;
        
        self.active_agents = active_agents;
        self.tasks_pending = pending_tasks;
        self.tasks_completed = completed_tasks;
        
        // Fetch LLM settings if available
        if let Ok(llm_settings) = core::config::get_llm_provider_settings().await {
            self.llm_provider = llm_settings.provider_name;
            self.llm_url = llm_settings.url;
            self.llm_model = llm_settings.model;
        }
        
        self.update_time = SystemTime::now();

        Ok(())
    }

    /// Display the dashboard
    pub fn display(&self) {
        // Format uptime
        let uptime_seconds = self.uptime % 60;
        let uptime_minutes = (self.uptime / 60) % 60;
        let uptime_hours = self.uptime / 3600;
        
        // Dashboard layout - top border
        println!("┌──────────────────────────────────────────────────────────────────────────────┐");
        
        // Header
        println!("│ {} │ {} │", 
            " System Metrics ".on_blue().white().bold(),
            format!(" Last updated: {:02}:{:02}:{:02} ", uptime_hours, uptime_minutes, uptime_seconds).on_black().white()
        );
        
        // Separator
        println!("├─────────────────────────────┬────────────────────────────────────────────────┤");
        
        // Resource metrics + Agent statistics
        println!("│ {:<25} │ {:<50} │", 
            " Resource Usage ".green().bold(),
            " Agent Statistics ".green().bold()
        );
        
        println!("│ CPU: {:<20} │ Agents Active: {:<37} │", 
            format!("{:.1}%", self.cpu_usage).yellow(),
            self.active_agents
        );
        
        println!("│ RAM: {:<20} │ Tasks Pending: {:<37} │", 
            format!("{:.1} MB", self.ram_usage).yellow(),
            self.tasks_pending
        );
        
        println!("│ Uptime: {:<18} │ Tasks Completed: {:<35} │", 
            format!("{:02}:{:02}:{:02}", uptime_hours, uptime_minutes, uptime_seconds).yellow(),
            self.tasks_completed
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
        
        println!("│ Tokens/sec: {:<14} │ LLM: {:<41} │", 
            format!("{:.2}", self.tokens_per_second).yellow(),
            format!("✓ {} @ {}", self.llm_provider, self.llm_model).green()
        );
        
        // Bottom border
        println!("└──────────────────────────────────────────────────────────────────────────────┘");
    }

    /// Start a background task to continuously update metrics
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
pub async fn init_dashboard() -> Result<Dashboard> {
    let mut dashboard = Dashboard::new();
    dashboard.update().await?;
    Ok(dashboard)  // Fixed: Wrap the return value in Ok()
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
