//! Configuration module for Nexa Gateway CLI
//!
//! This module provides functions for configuring the Nexa AI Orchestration Platform.

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Select, Confirm, Password};
use common::config::{LlmProviderSettings, AgentCommunicationSettings}; // Add this import
use gateway::config::{self, NetworkSettings, AuthSettings, LogSettings, OrchestratorSettings}; 

/// Run the configuration wizard
pub async fn run_configuration() -> Result<()> {
    println!("╔══════════════════════════════════╗");
    println!("║      PLATFORM CONFIGURATION      ║");
    println!("╚══════════════════════════════════╝");
    println!();
    
    // Ask for configuration options
    let config_options = &[
        "Network Settings",
        "Authentication Settings",
        "Orchestrator Settings",
        "LLM Provider Settings",
        "Agent Communication",
        "Log Settings",
        "User Management",
        "Save and Exit",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a configuration category:")
        .items(config_options)
        .default(0)
        .interact()?;
    
    match selection {
        0 => configure_network().await?,
        1 => configure_authentication().await?,
        2 => configure_orchestrator().await?,
        3 => configure_llm_providers().await?,
        4 => configure_agent_communication().await?,
        5 => configure_logging().await?,
        6 => configure_users().await?,
        7 => save_configuration().await?,
        _ => unreachable!(),
    }
    
    Ok(())
}

/// Configure network settings
async fn configure_network() -> Result<()> {
    println!("\n{}", style("Network Configuration").bold());
    
    // Get current network settings
    let current_settings = config::get_network_settings().await?;
    
    // Update settings
    let hostname = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Hostname")
        .default(current_settings.hostname)
        .interact_text()?;
        
    let port = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("API Port")
        .default(current_settings.port)
        .interact()?;
    
    let cors_origins = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Allowed CORS origins (comma-separated)")
        .default(current_settings.cors_origins.unwrap_or_default())
        .interact_text()?;
        
    let use_dhcp = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Use DHCP for IP configuration?")
        .default(current_settings.use_dhcp)
        .interact()?;
        
    // Static IP settings if DHCP is disabled
    let (ip_address, subnet_mask, gateway) = if !use_dhcp {
        let ip = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("IP Address")
            .default(current_settings.ip_address.unwrap_or_default())
            .interact_text()?;
            
        let subnet = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Subnet Mask")
            .default(current_settings.subnet_mask.unwrap_or_default())
            .interact_text()?;
            
        let gw = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Gateway")
            .default(current_settings.gateway.unwrap_or_default())
            .interact_text()?;
            
        (Some(ip), Some(subnet), Some(gw))
    } else {
        (None, None, None)
    };
    
    // Create updated settings
    let new_settings = NetworkSettings {
        hostname,
        port,
        cors_origins: Some(cors_origins),
        use_dhcp,
        ip_address,
        subnet_mask,
        gateway,
    };
    
    // Save new settings
    config::update_network_settings(&new_settings).await?;
    
    println!("\n{}", style("Network settings updated successfully!").green());
    Ok(())
}

/// Configure authentication settings
async fn configure_authentication() -> Result<()> {
    println!("\n{}", style("Authentication Configuration").bold());
    
    // Get current auth settings
    let current_settings = config::get_auth_settings().await?;
    
    // Choose auth method
    let auth_methods = &["API Key", "JWT Token", "OAuth2"];
    
    let auth_method = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select authentication method")
        .items(auth_methods)
        .default(match current_settings.auth_type.as_str() {
            "api_key" => 0,
            "jwt" => 1,
            "oauth2" => 2,
            _ => 0,
        })
        .interact()?;
        
    let auth_type = match auth_method {
        0 => "api_key".to_string(),
        1 => "jwt".to_string(),
        2 => "oauth2".to_string(),
        _ => unreachable!(),
    };
    
    println!("\nSelected authentication method: {}", auth_methods[auth_method]);
    
    // Additional settings based on selected method
    let (api_key, jwt_secret, oauth_settings) = match auth_method {
        0 => {
            // API Key
            let generate_new = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Generate new API key?")
                .default(false)
                .interact()?;
                
            let key = if generate_new {
                // Generate a new API key
                config::generate_api_key().await?
            } else {
                // Use existing or enter custom
                let custom_key = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter API key (leave empty to keep existing)")
                    .allow_empty(true)
                    .default(current_settings.api_key.unwrap_or_default())
                    .interact_text()?;
                    
                if custom_key.is_empty() {
                    current_settings.api_key.unwrap_or_default()
                } else {
                    custom_key
                }
            };
            
            (Some(key), None, None)
        },
        1 => {
            // JWT Token
            let generate_new = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Generate new JWT secret?")
                .default(false)
                .interact()?;
                
            let secret = if generate_new {
                // Generate a new JWT secret
                config::generate_jwt_secret().await?
            } else {
                // Use existing or enter custom
                let custom_secret = Password::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter JWT secret (leave empty to keep existing)")
                    .allow_empty_password(true)
                    .interact()?;
                    
                if custom_secret.is_empty() {
                    current_settings.jwt_secret.unwrap_or_default()
                } else {
                    custom_secret
                }
            };
            
            (None, Some(secret), None)
        },
        2 => {
            // OAuth2
            println!("OAuth2 configuration requires additional setup.");
            println!("Please use the web interface or configuration file.");
            
            (None, None, current_settings.oauth_settings)
        },
        _ => unreachable!(),
    };
    
    // Create updated settings
    let new_settings = AuthSettings {
        auth_type,
        api_key,
        jwt_secret,
        oauth_settings,
    };
    
    // Save new settings
    config::update_auth_settings(&new_settings).await?;
    
    println!("\n{}", style("Authentication settings updated successfully!").green());
    Ok(())
}

/// Configure orchestrator settings
async fn configure_orchestrator() -> Result<()> {
    println!("\n{}", style("Orchestrator Configuration").bold());
    
    // Get current orchestrator settings
    let current_settings = config::get_orchestrator_settings().await?;
    
    // Update settings
    let orchestrator_url = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Orchestrator URL")
        .default(current_settings.orchestrator_url)
        .interact_text()?;
        
    let orchestrator_token = Password::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Orchestrator Token [current: {}]", 
            if current_settings.orchestrator_token.is_empty() { "<empty>" } 
            else { "********" }))
        .interact()?;
        
    // Create updated settings
    let new_settings = OrchestratorSettings {
        orchestrator_url,
        orchestrator_token,
    };
    
    // Save new settings
    config::update_orchestrator_settings(&new_settings).await?;
    
    println!("\n{}", style("Orchestrator settings updated successfully!").green());
    Ok(())
}

/// Configure LLM provider settings
async fn configure_llm_providers() -> Result<()> {
    println!("\n{}", style("LLM Provider Configuration").bold());
    
    // Get current LLM provider settings
    let current_settings = config::get_llm_provider_settings().await?;
    
    // Update settings
    let provider_name = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("LLM Provider Name")
        .default(current_settings.provider_name)
        .interact_text()?;
        
    let api_key = Password::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("API Key [current: {}]",
            if current_settings.api_key.is_empty() { "<empty>" } 
            else { "********" }))
        .interact()?;
        
    let url = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("LLM Endpoint URL")
        .default(current_settings.url)
        .interact_text()?;
        
    // Test connection and fetch available models
    println!("Testing connection to LLM provider...");
    let connection_ok = core::llm::test_connection(&url).await?;
    
    // Initialize available models
    let mut available_models = current_settings.available_models.clone();
    let mut default_model = current_settings.default_model.clone();
    
    if connection_ok {
        println!("{}", style("✓ Connection successful").green());
        
        // Try to fetch models from the provider
        println!("Fetching available models...");
        match core::llm::fetch_available_models(&url).await {
            Ok(models) if !models.is_empty() => {
                println!("{}", style(format!("✓ Found {} models", models.len())).green());
                available_models = models;
                
                // Suggest a default model
                let suggested_default = core::llm::get_default_model(&available_models);
                
                // Display available models
                println!("\nAvailable models:");
                for (i, model) in available_models.iter().enumerate() {
                    println!("  {}. {}{}", 
                        i + 1, 
                        model,
                        if model == &suggested_default { " (suggested default)" } else { "" }
                    );
                }
                
                // Let user select default model
                let model_choices: Vec<&String> = available_models.iter().collect();
                let default_index = available_models.iter()
                    .position(|m| m == &suggested_default)
                    .unwrap_or(0);
                
                let selected_index = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select default model")
                    .items(&model_choices)
                    .default(default_index)
                    .interact()?;
                
                default_model = available_models[selected_index].clone();
            },
            Ok(_) => {
                println!("{}", style("No models found. Using default models.").yellow());
            },
            Err(e) => {
                println!("{}", style(format!("Failed to fetch models: {}", e)).red());
                println!("{}", style("Using default models.").yellow());
            }
        }
    } else {
        println!("{}", style("✗ Connection failed").red());
        println!("{}", style("Check URL or network connection").yellow());
    }
        
    let model = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Current Model")
        .default(default_model.clone())
        .interact_text()?;
        
    let temperature = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Temperature")
        .default(current_settings.temperature.to_string())
        .interact_text()?
        .parse::<f32>()?;
        
    let max_tokens = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Max Tokens")
        .default(current_settings.max_tokens.to_string())
        .interact_text()?
        .parse::<u32>()?;
        
    // Create updated settings
    let new_settings = LlmProviderSettings {
        provider_name,
        api_key,
        url,
        model,
        temperature,
        max_tokens,
        available_models,
        default_model,
    };
    
    // Save new settings
    config::update_llm_provider_settings(&new_settings).await?;
    
    println!("\n{}", style("LLM provider settings updated successfully!").green());
    Ok(())
}

/// Configure agent communication settings
async fn configure_agent_communication() -> Result<()> {
    println!("\n{}", style("Agent Communication Configuration").bold());
    
    // Get current agent communication settings
    let current_settings = config::get_agent_communication_settings().await?;
    
    // Update settings
    let agent_url = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Agent URL")
        .default(current_settings.agent_url)
        .interact_text()?;
        
    let agent_token = Password::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Agent Token [current: {}]",
            if current_settings.agent_token.is_empty() { "<empty>" } 
            else { "********" }))
        .interact()?;
        
    // Create updated settings
    let new_settings = AgentCommunicationSettings {
        agent_url,
        agent_token,
    };
    
    // Save new settings
    config::update_agent_communication_settings(&new_settings).await?;
    
    println!("\n{}", style("Agent communication settings updated successfully!").green());
    Ok(())
}

/// Configure logging settings
async fn configure_logging() -> Result<()> {
    println!("\n{}", style("Logging Configuration").bold());
    
    // Get current log settings
    let current_settings = config::get_log_settings().await?;
    
    // Log level
    let log_levels = &["Debug", "Info", "Warning", "Error"];
    
    let log_level_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select log level")
        .items(log_levels)
        .default(match current_settings.level.as_str() {
            "debug" => 0,
            "info" => 1,
            "warning" => 2,
            "error" => 3,
            _ => 1,
        })
        .interact()?;
    
    let level = match log_level_idx {
        0 => "debug".to_string(),
        1 => "info".to_string(),
        2 => "warning".to_string(),
        3 => "error".to_string(),
        _ => unreachable!(),
    };
        
    // Log file path
    let file_path = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Log file path")
        .default(current_settings.file_path)
        .interact_text()?;
        
    // Log rotation
    let max_size_mb = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Max log size (MB)")
        .default(current_settings.max_size_mb)
        .interact()?;
        
    let max_files = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Max log files to keep")
        .default(current_settings.max_files)
        .interact()?;
        
    // Create updated settings
    let new_settings = LogSettings {
        level,
        file_path,
        max_size_mb,
        max_files,
    };
    
    // Save new settings
    config::update_log_settings(&new_settings).await?;
    
    println!("\n{}", style("Logging settings updated successfully!").green());
    Ok(())
}

/// Configure users
async fn configure_users() -> Result<()> {
    println!("\n{}", style("User Management").bold());
    
    // Get current users
    let users = config::get_users().await?;
    
    // Show user management options
    let options = &["List Users", "Add User", "Remove User", "Change Password", "Back"];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action")
        .items(options)
        .default(0)
        .interact()?;
        
    match selection {
        0 => {
            // List users
            println!("\nUsers:");
            println!("{:<20} {:<15} {:<10}", 
                style("Username").bold(),
                style("Role").bold(),
                style("Last Login").bold()
            );
            
            println!("{}", style("─".repeat(45)).dim());
            
            for user in users {
                println!("{:<20} {:<15} {:<10}",
                    user.username,
                    user.role,
                    user.last_login.unwrap_or_default()
                );
            }
        },
        1 => {
            // Add user
            let username = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Username")
                .interact_text()?;
                
            let password = Password::with_theme(&ColorfulTheme::default())
                .with_prompt("Password")
                .with_confirmation("Confirm password", "Passwords don't match")
                .interact()?;
                
            let roles = &["Admin", "User", "ReadOnly"];
            let role_idx = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("User role")
                .items(roles)
                .default(1)
                .interact()?;
                
            let role = roles[role_idx].to_lowercase();
            
            config::add_user(&username, &password, &role).await?;
            println!("\n{}", style("User added successfully!").green());
        },
        2 => {
            // Remove user
            if users.is_empty() {
                println!("No users available to remove.");
                return Ok(());
            }
            
            let user_names: Vec<&str> = users.iter()
                .map(|u| u.username.as_str())
                .collect();
                
            let selected_idx = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select user to remove")
                .items(&user_names)
                .default(0)
                .interact()?;
                
            let username = &users[selected_idx].username;
            
            let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Are you sure you want to remove user '{}'?", username))
                .default(false)
                .interact()?;
                
            if confirm {
                config::remove_user(username).await?;
                println!("\n{}", style("User removed successfully!").green());
            } else {
                println!("\nUser removal cancelled.");
            }
        },
        3 => {
            // Change password
            if users.is_empty() {
                println!("No users available.");
                return Ok(());
            }
            
            let user_names: Vec<&str> = users.iter()
                .map(|u| u.username.as_str())
                .collect();
                
            let selected_idx = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select user")
                .items(&user_names)
                .default(0)
                .interact()?;
                
            let username = &users[selected_idx].username;
            
            let password = Password::with_theme(&ColorfulTheme::default())
                .with_prompt("New password")
                .with_confirmation("Confirm password", "Passwords don't match")
                .interact()?;
                
            config::change_password(username, &password).await?;
            println!("\n{}", style("Password changed successfully!").green());
        },
        4 => return Ok(()),
        _ => unreachable!(),
    }
    
    Ok(())
}

/// Save configuration
async fn save_configuration() -> Result<()> {
    println!("\nSaving configuration...");
    
    // Save all configuration changes
    let result = config::save_all_settings().await;
    
    match result {
        Ok(_) => {
            println!("{}", style("Configuration saved successfully!").green());
            
            // Ask if restart is needed
            let restart = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Some changes may require a restart. Restart gateway now?")
                .default(false)
                .interact()?;
                
            if restart {
                println!("Restarting gateway services...");
                config::restart_gateway().await?;
                println!("{}", style("Gateway restarted successfully!").green());
            }
        },
        Err(e) => {
            println!("{}", style(format!("Error saving configuration: {}", e)).red());
        }
    }
    
    Ok(())
}
