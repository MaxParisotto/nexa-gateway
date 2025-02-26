//! Dashboard persistence for Nexa Gateway CLI
//!
//! This module provides functionality to save and load dashboard configurations.

use anyhow::Result;
use console::style;
use std::fs;
use std::path::PathBuf;
use dialoguer::{theme::ColorfulTheme, Input, Select};

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
