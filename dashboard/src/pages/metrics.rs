//! Metrics page for the dashboard
//! 
//! This page displays metrics and statistics for the gateway.

use leptos::*;
use crate::components::{Header, Sidebar, Card, MetricsChart};

/// Metrics page component
#[component]
pub fn MetricsPage() -> impl IntoView {
    // Sample metrics data
    let cpu_data = vec![23.5, 25.2, 28.7, 22.1, 24.5, 26.8, 29.2];
    let memory_data = vec![512.0, 524.3, 498.7, 532.1, 545.6, 528.9, 510.2];
    let requests_data = vec![156.0, 142.0, 164.0, 178.0, 153.0, 162.0, 170.0];
    let latency_data = vec![42.0, 38.0, 45.0, 40.0, 37.0, 43.0, 41.0];
    
    view! {
        <div class="dashboard-container">
            <Header title="Nexa Gateway Dashboard" />
            <div class="dashboard-content">
                <Sidebar active_page="metrics" />
                <main class="main-content">
                    <h1>"System Metrics"</h1>
                    
                    <div class="metrics-grid">
                        <Card title="CPU Usage (%)">
                            <MetricsChart title="Last 7 Days" data=cpu_data />
                        </Card>
                        
                        <Card title="Memory Usage (MB)">
                            <MetricsChart title="Last 7 Days" data=memory_data />
                        </Card>
                        
                        <Card title="Requests per Minute">
                            <MetricsChart title="Last 7 Days" data=requests_data />
                        </Card>
                        
                        <Card title="Average Latency (ms)">
                            <MetricsChart title="Last 7 Days" data=latency_data />
                        </Card>
                    </div>
                    
                    <div class="metrics-actions">
                        <button class="action-button">"Export Data"</button>
                        <button class="action-button">"Reset Counters"</button>
                    </div>
                </main>
            </div>
        </div>
    }
}
