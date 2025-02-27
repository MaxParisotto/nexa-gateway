//! Status module for Nexa Gateway CLI
//!
//! This module provides functionality to display system status information.

use anyhow::Result;
use colored::Colorize;
use prettytable::{table, row};

/// Display system metrics
#[allow(dead_code)]
pub async fn display_system_metrics() -> Result<()> {
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
    
    // Also display LLM settings
    display_llm_settings().await?;
    
    Ok(())
}

/// Display agent metrics
#[allow(dead_code)]
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

/// Display LLM settings
#[allow(dead_code)]
pub async fn display_llm_settings() -> Result<()> {
    println!("\n{}", "LLM Settings".bold().blue().underline());
    
    // In a real implementation, this would fetch from the config service
    // For this demo, we'll use the core config directly
    let llm_settings = core::config::get_llm_provider_settings().await?;
    
    let mut table = table!();
    
    table.add_row(row!["Setting".bold(), "Value".bold()]);
    table.add_row(row!["Provider", llm_settings.provider_name]);
    table.add_row(row!["URL", llm_settings.url]);
    table.add_row(row!["Current Model", llm_settings.model]);
    table.add_row(row!["Default Model", llm_settings.default_model]);
    table.add_row(row!["Temperature", llm_settings.temperature.to_string()]);
    table.add_row(row!["Max Tokens", llm_settings.max_tokens.to_string()]);
    
    table.printstd();
    
    // Show available models
    if !llm_settings.available_models.is_empty() {
        println!("\n{}", "Available Models:".bold());
        for model in &llm_settings.available_models {
            if model == &llm_settings.default_model {
                println!("  • {} (default)", model.green());
            } else if model == &llm_settings.model {
                println!("  • {} (current)", model.yellow());
            } else {
                println!("  • {}", model);
            }
        }
    }
    
    Ok(())
}