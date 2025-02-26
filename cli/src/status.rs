//! Status module for Nexa Gateway CLI
//!
//! This module provides functionality to display system status information.

use anyhow::Result;
use colored::Colorize;
use prettytable::{table, row};

/// Display system metrics
pub fn display_system_metrics() -> Result<()> {
    println!("{}", "System Metrics".bold().green().underline());
    
    // For now, let's just create mock metrics
    let cpu_usage = 25.5;
    let memory_usage = 1024.0;
    let uptime = 3600;
    let connections = 42;
    
    let mut table = table!();
    
    table.add_row(row!["Metric".bold(), "Value".bold()]);
    table.add_row(row!["CPU Usage", format!("{:.1}%", cpu_usage)]);
    table.add_row(row!["Memory Usage", format!("{:.1} MB", memory_usage)]);
    table.add_row(row!["Uptime", format!("{} seconds", uptime)]);
    table.add_row(row!["Active Connections", connections.to_string()]);
    
    table.printstd();
    
    Ok(())
}

/// Display agent metrics
pub fn display_agent_metrics() -> Result<()> {
    println!("{}", "Agent Metrics".bold().green().underline());
    
    // Mock agent metrics
    let active_agents = 3;
    let messages_processed = 120;
    
    let mut table = table!();
    
    table.add_row(row!["Metric".bold(), "Value".bold()]);
    table.add_row(row!["Active Agents", active_agents.to_string()]);
    table.add_row(row!["Messages Processed", messages_processed.to_string()]);
    
    table.printstd();
    
    Ok(())
}