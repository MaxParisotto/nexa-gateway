//! Status indicator component for the dashboard
//! 
//! This component displays a status indicator with a color and label.

use leptos::*;

/// Status indicator component for the dashboard
#[component]
pub fn StatusIndicator(
    #[prop(into)] status: String,
) -> impl IntoView {
    // Determine the status color based on the status value
    let status_color = match status.as_str() {
        "Online" => "green",
        "Offline" => "red",
        "Warning" => "orange",
        "Maintenance" => "blue",
        _ => "gray",
    };
    
    view! {
        <div class="status-indicator">
            <div class="status-dot" style={format!("background-color: {}", status_color)}></div>
            <div class="status-label">{status}</div>
        </div>
    }
}
