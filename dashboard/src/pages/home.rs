//! Home page for the dashboard
//! 
//! This is the main landing page for the dashboard.

use leptos::*;
use crate::components::{Header, Sidebar, Card, StatusIndicator};

/// Home page component
#[component]
pub fn HomePage() -> impl IntoView {
    // Static data for the dashboard
    let gateway_status = "Online".to_string();
    let active_connections = 42;
    let cpu_usage = 23.5;
    let memory_usage = 512.0;
    
    view! {
        <div class="dashboard-container">
            <Header title="Nexa Gateway Dashboard" />
            <div class="dashboard-content">
                <Sidebar active_page="home" />
                <main class="main-content">
                    <h1>"Dashboard Overview"</h1>
                    <div class="status-overview">
                        <Card title="Gateway Status">
                            <StatusIndicator status=gateway_status />
                        </Card>
                        <Card title="Active Connections">
                            <div class="metric-value">{active_connections}</div>
                        </Card>
                        <Card title="CPU Usage">
                            <div class="metric-value">{format!("{:.1}%", cpu_usage)}</div>
                        </Card>
                        <Card title="Memory Usage">
                            <div class="metric-value">{format!("{:.1} MB", memory_usage)}</div>
                        </Card>
                    </div>
                    <div class="quick-actions">
                        <h2>"Quick Actions"</h2>
                        <div class="action-buttons">
                            <button class="action-button">"Restart Gateway"</button>
                            <button class="action-button">"Clear Cache"</button>
                            <button class="action-button">"View Logs"</button>
                            <button class="action-button">"Update Settings"</button>
                        </div>
                    </div>
                </main>
            </div>
        </div>
    }
}
