//! Home page for the dashboard
//! 
//! This is the main landing page for the dashboard.

use leptos::*;
use leptos::prelude::*;
use leptos::html::ElementChild;  // Add ElementChild trait

/// Home page component
#[component]
pub fn HomePage() -> impl IntoView {
    // Note: Renamed from home_page to HomePage to match convention
    view! {
        <div class="page">
            <h1>"Welcome to Nexa Gateway Dashboard"</h1>
            <p>"This dashboard allows you to monitor and control your Nexa Gateway."</p>
            
            <div class="dashboard-tiles">
                <div class="tile">
                    <h2>"Status"</h2>
                    <p class="status-indicator running">"Running"</p>
                </div>
                
                <div class="tile">
                    <h2>"API Requests"</h2>
                    <p class="metric-value">"156 req/min"</p>
                </div>
                
                <div class="tile">
                    <h2>"Latency"</h2>
                    <p class="metric-value">"42ms"</p>
                </div>
                
                <div class="tile">
                    <h2>"Uptime"</h2>
                    <p class="metric-value">"3d 14h 22m"</p>
                </div>
            </div>
            
            <div class="quick-actions">
                <h2>"Quick Actions"</h2>
                <div class="button-group">
                    <button class="primary">"Restart Gateway"</button>
                    <button>"View Logs"</button>
                    <button>"Check Updates"</button>
                </div>
            </div>
        </div>
    }
}
