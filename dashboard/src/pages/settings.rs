//! Settings page for the dashboard
//! 
//! This page allows users to configure gateway settings.

use leptos::*;
use leptos::prelude::*;
use leptos::html::ElementChild;  // Add ElementChild trait
use leptos::suspense::Suspense;
// Remove unused import
// use leptos_router::*;
use crate::api::settings::{GatewaySettings, get_settings, update_settings, reset_settings};

/// Settings page component
#[component]
pub fn SettingsPage() -> impl IntoView {
    // Create resource with Resource::new instead of create_resource
    let settings_resource = Resource::new(
        || (), 
        |_| async move { get_settings().await.unwrap_or_else(|_| {
            // Return default settings on error
            GatewaySettings {
                general: crate::api::settings::GeneralSettings {
                    name: "Nexa Gateway".to_string(),
                    description: "Main API gateway for Nexa services".to_string(),
                    port: 8080,
                },
                security: crate::api::settings::SecuritySettings {
                    auth_enabled: true,
                    jwt_expiry: 60,
                    rate_limit: 100,
                },
                logging: crate::api::settings::LoggingSettings {
                    log_level: "Info".to_string(),
                    log_retention: 30,
                },
            }
        })}
    );
    
    // Update to use Action::new instead of create_action
    let _save_settings = Action::new(|settings: &GatewaySettings| {
        let settings = settings.clone();
        async move {
            match update_settings(settings).await {
                Ok(_) => true,
                Err(_) => false,
            }
        }
    });
    
    // Update to use Action::new
    let reset = Action::new(|_: &()| {
        async move {
            match reset_settings().await {
                Ok(settings) => Some(settings),
                Err(_) => None,
            }
        }
    });
    
    view! {
        <div class="page">
            <h1>"Gateway Settings"</h1>
            
            <form class="settings-form">
                <Suspense
                    fallback=move || view! { <p>"Loading settings..."</p> }
                >
                    {move || settings_resource.get().map(|settings| view! {
                        <div class="settings-section">
                            <h2>"General Settings"</h2>
                            
                            <div class="form-group">
                                <label for="gateway-name">"Gateway Name"</label>
                                <input type="text" id="gateway-name" value={settings.general.name} />
                            </div>
                            
                            <div class="form-group">
                                <label for="gateway-description">"Description"</label>
                                <textarea id="gateway-description">{settings.general.description}</textarea>
                            </div>
                            
                            <div class="form-group">
                                <label for="gateway-port">"Port"</label>
                                <input type="number" id="gateway-port" value={settings.general.port} />
                            </div>
                        </div>
                        
                        <div class="settings-section">
                            <h2>"Security Settings"</h2>
                            
                            <div class="form-group checkbox">
                                <input type="checkbox" id="auth-enabled" checked={settings.security.auth_enabled} />
                                <label for="auth-enabled">"Enable Authentication"</label>
                            </div>
                            
                            <div class="form-group">
                                <label for="jwt-expiry">"JWT Expiry (minutes)"</label>
                                <input type="number" id="jwt-expiry" value={settings.security.jwt_expiry} />
                            </div>
                            
                            <div class="form-group">
                                <label for="rate-limit">"Rate Limit (requests/minute)"</label>
                                <input type="number" id="rate-limit" value={settings.security.rate_limit} />
                            </div>
                        </div>
                        
                        <div class="button-group">
                            <button type="submit" class="primary">"Save Settings"</button>
                            <button 
                                type="button"
                                on:click=move |e| {
                                    e.prevent_default();
                                    reset.dispatch(());
                                }
                            >
                                "Reset to Defaults"
                            </button>
                        </div>
                    })}
                </Suspense>
            </form>
        </div>
    }
}
