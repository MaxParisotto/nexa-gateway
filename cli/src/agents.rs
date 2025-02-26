//! AI Agent management module for Nexa Gateway CLI
//!
//! This module provides functions for listing and managing AI agents.

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm, MultiSelect};
use gateway::agent::{self, AgentInfo, AgentStatus, AgentCapability}; // Assuming this exists in gateway crate

/// Manage AI agents
pub async fn manage_agents() -> Result<()> {
    println!("╔══════════════════════════════════╗");
    println!("║        AI AGENT MANAGEMENT       ║");
    println!("╚══════════════════════════════════╝");
    println!();
    
    // Get all registered agents
    let agents = agent::get_all_agents().await?;
    
    println!("Found {} AI agents:", agents.len());
    println!();
    
    println!("{:<20} {:<15} {:<10} {:<20}", 
        style("Name").bold(), 
        style("Version").bold(), 
        style("Status").bold(), 
        style("Capabilities").bold()
    );
    
    println!("{}", style("─".repeat(70)).dim());
    
    for agent in &agents {
        let status_style = match agent.status {
            AgentStatus::Running => style("Running").green(),
            AgentStatus::Idle => style("Idle").blue(),
            AgentStatus::Stopped => style("Stopped").yellow(),
            AgentStatus::Error => style("Error").red(),
        };
        
        // Format capabilities as a comma-separated list
        let capabilities = agent.capabilities
            .iter()
            .map(|cap| cap.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        
        println!("{:<20} {:<15} {:<10} {:<20}", 
            agent.name, 
            agent.version, 
            status_style, 
            if capabilities.len() > 20 { format!("{}...", &capabilities[..17]) } else { capabilities }
        );
    }
    
    // Show agent management options
    println!("\nAgent Management Options:");
    let options = &[
        "Register New Agent",
        "Configure Agent",
        "Start/Stop Agent",
        "Remove Agent",
        "Deploy Agent",
        "Back to Main Menu"
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(options)
        .default(0)
        .interact()?;
    
    match selection {
        0 => register_new_agent().await?,
        1 => configure_agent(&agents).await?,
        2 => toggle_agent_status(&agents).await?,
        3 => remove_agent(&agents).await?,
        4 => deploy_agent().await?,
        5 => return Ok(()),
        _ => unreachable!(),
    }
    
    Ok(())
}

/// Register a new AI agent
async fn register_new_agent() -> Result<()> {
    println!("\n{}", style("Register New AI Agent").bold());
    
    // Get agent details
    let name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Agent Name")
        .interact_text()?;
        
    let version = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Agent Version")
        .default("1.0.0".into())
        .interact_text()?;
        
    let description = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Agent Description")
        .interact_text()?;
    
    // Select agent type
    let agent_types = &["LLM", "Data Processing", "Reasoning", "Planning", "Custom"];
    let agent_type_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Agent Type")
        .items(agent_types)
        .default(0)
        .interact()?;
    
    // Select agent capabilities
    let all_capabilities = agent::get_available_capabilities().await?;
    let capability_names: Vec<&str> = all_capabilities.iter()
        .map(|c| c.name.as_str())
        .collect();
    
    let selected_indices = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select agent capabilities (Space to select, Enter to confirm)")
        .items(&capability_names)
        .defaults(&[true, false, false, false]) // First capability selected by default
        .interact()?;
    
    let selected_capabilities: Vec<AgentCapability> = selected_indices.iter()
        .map(|&i| all_capabilities[i].clone())
        .collect();
    
    // Create agent configuration
    let endpoint_url = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Agent Endpoint URL")
        .default("http://localhost:8000".into())
        .interact_text()?;
    
    let api_key = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("API Key (leave empty if none)")
        .allow_empty(true)
        .interact_text()?;
    
    // Create new agent info
    let agent = AgentInfo {
        name,
        version,
        description,
        agent_type: agent_types[agent_type_idx].to_string(),
        capabilities: selected_capabilities,
        endpoint_url,
        api_key: if api_key.is_empty() { None } else { Some(api_key) },
        status: AgentStatus::Stopped, // Default to stopped
    };
    
    // Register the agent
    agent::register_agent(&agent).await?;
    
    println!("\n{}", style("Agent registered successfully!").green());
    Ok(())
}

/// Configure an existing agent
async fn configure_agent(agents: &[AgentInfo]) -> Result<()> {
    println!("\n{}", style("Configure AI Agent").bold());
    
    if agents.is_empty() {
        println!("No agents available to configure.");
        return Ok(());
    }
    
    // Create agent name list for selection
    let agent_names: Vec<&str> = agents.iter()
        .map(|a| a.name.as_str())
        .collect();
    
    let selected_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select agent to configure")
        .items(&agent_names)
        .default(0)
        .interact()?;
    
    // Get the selected agent
    let agent = &agents[selected_idx];
    println!("\nConfiguring agent: {}", style(&agent.name).bold());
    
    // Show configuration options
    let config_options = &[
        "Update Endpoint URL",
        "Change API Key",
        "Modify Capabilities",
        "Configure Resources",
        "Advanced Settings",
        "Back"
    ];
    
    let config_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select configuration action")
        .items(config_options)
        .default(0)
        .interact()?;
        
    match config_selection {
        0 => {
            let new_url = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("New endpoint URL")
                .default(agent.endpoint_url.clone())
                .interact_text()?;
                
            agent::update_agent_endpoint(&agent.id, &new_url).await?;
            println!("\n{}", style("Endpoint URL updated successfully!").green());
        },
        1 => {
            let new_api_key = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("New API key (leave empty to remove)")
                .allow_empty(true)
                .default(agent.api_key.clone().unwrap_or_default())
                .interact_text()?;
                
            agent::update_agent_api_key(&agent.id, if new_api_key.is_empty() { None } else { Some(new_api_key) }).await?;
            println!("\n{}", style("API key updated successfully!").green());
        },
        2 => {
            // Modify capabilities
            let all_capabilities = agent::get_available_capabilities().await?;
            let capability_names: Vec<&str> = all_capabilities.iter()
                .map(|c| c.name.as_str())
                .collect();
            
            // Pre-select existing capabilities
            let defaults: Vec<bool> = all_capabilities.iter()
                .map(|cap| agent.capabilities.contains(cap))
                .collect();
            
            let selected_indices = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select agent capabilities (Space to select, Enter to confirm)")
                .items(&capability_names)
                .defaults(&defaults)
                .interact()?;
            
            let updated_capabilities: Vec<AgentCapability> = selected_indices.iter()
                .map(|&i| all_capabilities[i].clone())
                .collect();
                
            agent::update_agent_capabilities(&agent.id, &updated_capabilities).await?;
            println!("\n{}", style("Agent capabilities updated successfully!").green());
        },
        3 => {
            // Configure resources
            let max_memory = Input::<u32>::with_theme(&ColorfulTheme::default())
                .with_prompt("Max memory (MB)")
                .default(1024)
                .interact()?;
                
            let max_cpu = Input::<f32>::with_theme(&ColorfulTheme::default())
                .with_prompt("CPU cores")
                .default(1.0)
                .interact()?;
                
            agent::update_agent_resources(&agent.id, max_memory, max_cpu).await?;
            println!("\n{}", style("Agent resources updated successfully!").green());
        },
        4 => {
            println!("\nAdvanced settings are not available in the CLI.");
            println!("Please use the configuration file or API directly.");
        },
        5 => return Ok(()),
        _ => unreachable!(),
    }
    
    Ok(())
}

/// Start or stop an agent
async fn toggle_agent_status(agents: &[AgentInfo]) -> Result<()> {
    println!("\n{}", style("Start/Stop Agent").bold());
    
    if agents.is_empty() {
        println!("No agents available.");
        return Ok(());
    }
    
    // Create agent name list for selection
    let agent_names: Vec<String> = agents.iter()
        .map(|a| format!("{} ({})", a.name, match a.status {
            AgentStatus::Running => "Running",
            AgentStatus::Idle => "Idle",
            AgentStatus::Stopped => "Stopped", 
            AgentStatus::Error => "Error",
        }))
        .collect();
    
    let selected_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select agent")
        .items(&agent_names)
        .default(0)
        .interact()?;
    
    // Get the selected agent
    let agent = &agents[selected_idx];
    
    let new_status = match agent.status {
        AgentStatus::Running => {
            println!("\nStopping agent: {}", style(&agent.name).bold());
            AgentStatus::Stopped
        },
        AgentStatus::Idle | AgentStatus::Stopped | AgentStatus::Error => {
            println!("\nStarting agent: {}", style(&agent.name).bold());
            AgentStatus::Running
        }
    };
    
    // Update agent status
    agent::update_agent_status(&agent.id, new_status).await?;
    
    println!("\n{}", style("Agent status updated successfully!").green());
    Ok(())
}

/// Remove an agent
async fn remove_agent(agents: &[AgentInfo]) -> Result<()> {
    println!("\n{}", style("Remove Agent").bold());
    
    if agents.is_empty() {
        println!("No agents available to remove.");
        return Ok(());
    }
    
    // Create agent name list for selection
    let agent_names: Vec<&str> = agents.iter()
        .map(|a| a.name.as_str())
        .collect();
    
    let selected_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select agent to remove")
        .items(&agent_names)
        .default(0)
        .interact()?;
    
    // Get the selected agent
    let agent = &agents[selected_idx];
    
    // Confirm removal
    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Are you sure you want to remove agent '{}'?", agent.name))
        .default(false)
        .interact()?;
        
    if confirm {
        agent::remove_agent(&agent.id).await?;
        println!("\n{}", style("Agent removed successfully!").green());
    } else {
        println!("\nAgent removal cancelled.");
    }
    
    Ok(())
}

/// Deploy a new agent from a template or package
async fn deploy_agent() -> Result<()> {
    println!("\n{}", style("Deploy Agent").bold());
    
    // Get available agent templates
    let templates = agent::get_available_templates().await?;
    
    if templates.is_empty() {
        println!("No agent templates available.");
        return Ok(());
    }
    
    // Create template list for selection
    let template_names: Vec<&str> = templates.iter()
        .map(|t| t.name.as_str())
        .collect();
    
    let selected_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select agent template")
        .items(&template_names)
        .default(0)
        .interact()?;
    
    // Get the selected template
    let template = &templates[selected_idx];
    
    println!("\nTemplate: {}", style(&template.name).bold());
    println!("Description: {}", template.description);
    println!("Version: {}", template.version);
    
    // Confirm deployment
    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Deploy this agent template?")
        .default(true)
        .interact()?;
        
    if confirm {
        // Get deployment name
        let name = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Deployment name")
            .default(template.name.clone())
            .interact_text()?;
        
        // Deploy the agent
        let agent_id = agent::deploy_agent_from_template(&template.id, &name).await?;
        
        println!("\n{}", style("Agent deployed successfully!").green());
        println!("Agent ID: {}", agent_id);
    } else {
        println!("\nDeployment cancelled.");
    }
    
    Ok(())
}
