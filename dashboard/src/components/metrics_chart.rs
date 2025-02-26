//! Metrics chart component for the dashboard
//! 
//! This component displays a chart for metrics data.

use leptos::*;
use leptos::prelude::*;
use leptos::html::ElementChild;  // Add ElementChild trait

/// Metrics chart component for the dashboard
#[component]
pub fn MetricsChart(
    #[prop(into)] title: String,
    #[prop(into)] data: Vec<f64>,
) -> impl IntoView {
    // In a real application, this would use a charting library
    // For now, we'll create a simple bar chart with divs
    
    let max_value = data.iter().fold(0.0, |max, &val| if val > max { val } else { max });
    
    view! {
        <div class="metrics-chart">
            <h3 class="chart-title">{title}</h3>
            <div class="chart-container">
                {data.iter().enumerate().map(|(_i, &value)| {
                    let height = if max_value > 0.0 { (value / max_value * 100.0) as i32 } else { 0 };
                    let bar_style = format!("height: {}%; margin-left: 4px;", height);
                    
                    view! {
                        <div class="chart-bar" style={bar_style}>
                            <div class="bar-value">{format!("{:.1}", value)}</div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
