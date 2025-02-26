//! Metrics page for the dashboard
//!
//! This page displays various metrics for the gateway.

use leptos::*;
use leptos::prelude::*;
use leptos::html::ElementChild;  // Add ElementChild trait
use crate::components::MetricsChart;  // Import directly from components module

/// Metrics page component
#[component]
pub fn MetricsPage() -> impl IntoView {
    view! {
        <div class="page">
            <h1>"Gateway Metrics"</h1>
            
            <div class="metrics-container">
                <MetricsChart 
                    title="CPU Usage (%)"
                    data=vec![23.5, 25.2, 28.7, 22.1, 24.5, 26.8, 29.2]
                />
                
                <MetricsChart
                    title="Memory Usage (MB)"
                    data=vec![512.0, 524.3, 498.7, 532.1, 545.6, 528.9, 510.2]
                />
                
                <MetricsChart
                    title="Requests per Minute" 
                    data=vec![156.0, 142.0, 164.0, 178.0, 153.0, 162.0, 170.0]
                />
                
                <MetricsChart
                    title="Latency (ms)"
                    data=vec![42.0, 38.0, 45.0, 40.0, 37.0, 43.0, 41.0]
                />
            </div>
        </div>
    }
}
