//! Settings page for the dashboard
//! 
//! This page allows users to configure gateway settings.

use leptos::*;
use crate::components::{Header, Sidebar, Card};

/// Settings page component
#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="dashboard-container">
            <Header title="Nexa Gateway Dashboard" />
            <div class="dashboard-content">
                <Sidebar active_page="settings" />
                <main class="main-content">
                    <h1>"Gateway Settings"</h1>
                    
                    <div class="settings-grid">
                        <Card title="General Settings">
                            <form class="settings-form">
                                <div class="form-group">
                                    <label for="gateway-name">"Gateway Name"</label>
                                    <input type="text" id="gateway-name" value="Nexa Gateway" />
                                </div>
                                
                                <div class="form-group">
                                    <label for="gateway-description">"Description"</label>
                                    <textarea id="gateway-description">"Main API gateway for Nexa services"</textarea>
                                </div>
                                
                                <div class="form-group">
                                    <label for="gateway-port">"Port"</label>
                                    <input type="number" id="gateway-port" value="8080" />
                                </div>
                            </form>
                        </Card>
                        
                        <Card title="Security Settings">
                            <form class="settings-form">
                                <div class="form-group">
                                    <label for="auth-enabled">"Authentication Enabled"</label>
                                    <input type="checkbox" id="auth-enabled" checked="true" />
                                </div>
                                
                                <div class="form-group">
                                    <label for="jwt-expiry">"JWT Expiry (minutes)"</label>
                                    <input type="number" id="jwt-expiry" value="60" />
                                </div>
                                
                                <div class="form-group">
                                    <label for="rate-limit">"Rate Limit (requests/minute)"</label>
                                    <input type="number" id="rate-limit" value="100" />
                                </div>
                            </form>
                        </Card>
                        
                        <Card title="Logging Settings">
                            <form class="settings-form">
                                <div class="form-group">
                                    <label for="log-level">"Log Level"</label>
                                    <select id="log-level">
                                        <option>"Debug"</option>
                                        <option selected="true">"Info"</option>
                                        <option>"Warning"</option>
                                        <option>"Error"</option>
                                    </select>
                                </div>
                                
                                <div class="form-group">
                                    <label for="log-retention">"Log Retention (days)"</label>
                                    <input type="number" id="log-retention" value="30" />
                                </div>
                            </form>
                        </Card>
                    </div>
                    
                    <div class="settings-actions">
                        <button class="action-button primary">"Save Settings"</button>
                        <button class="action-button">"Reset to Defaults"</button>
                    </div>
                </main>
            </div>
        </div>
    }
}
