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

/// Select an LLM model from available models
pub async fn select_llm_model() -> Result<()> {
    println!("{}", "LLM Model Selection".bold().blue().underline());
    
    // Get current LLM settings
    let mut llm_settings = core::config::get_llm_provider_settings().await?;
    
    // Display current settings
    println!("\nCurrent LLM Provider: {}", llm_settings.provider_name.green());
    println!("Current Model: {}", llm_settings.model.green());
    println!("Provider URL: {}", llm_settings.url);
    
    println!("\n{}", "Fetching available models...".yellow());
    
    // Fetch models from LLM provider
    let mut available_models = match core::llm::fetch_available_models(&llm_settings.url).await {
        Ok(models) => models,
        Err(e) => {
            println!("{}", format!("Error fetching models: {}", e).red());
            // Fall back to models from settings - clone to avoid partial move
            llm_settings.available_models.clone()
        }
    };
    
    // Safety check: If still empty, add at least one default model to prevent dialogue panic
    if available_models.is_empty() {
        println!("{}", "No models available. Adding default model.".yellow());
        available_models.push("local".to_string());
    }
    
    // Create items for selection with indicators for current and default
    let selection_items: Vec<String> = available_models.iter().map(|model| {
        if model == &llm_settings.model && model == &llm_settings.default_model {
            format!("{} (current, default)", model)
        } else if model == &llm_settings.model {
            format!("{} (current)", model)
        } else if model == &llm_settings.default_model {
            format!("{} (default)", model)
        } else {
            model.clone()
        }
    }).collect();
    
    // Find the current model's position, defaulting to 0 if not found
    let default_position = available_models.iter()
        .position(|m| m == &llm_settings.model)
        .unwrap_or(0);
    
    // Allow user to select a model
    let selection = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Select a model to use")
        .items(&selection_items)
        .default(default_position)
        .interact()?;
    
    let selected_model = &available_models[selection];
    
    // Confirm if not already selected
    if selected_model != &llm_settings.model {
        let confirm = dialoguer::Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt(format!("Switch to model '{}'?", selected_model))
            .default(true)
            .interact()?;
            
        if confirm {
            // Update settings with the new model
            llm_settings.model = selected_model.clone();
            if let Err(e) = core::config::update_llm_provider_settings(&llm_settings).await {
                println!("{}", format!("Error updating model: {}", e).red());
                return Ok(());
            }
            println!("{}", format!("Successfully switched to model '{}'", selected_model).green());
        } else {
            println!("Model change cancelled.");
        }
    } else {
        println!("{}", format!("Already using model '{}'", selected_model).blue());
    }
    
    Ok(())
}