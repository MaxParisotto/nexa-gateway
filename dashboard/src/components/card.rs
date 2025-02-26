//! Card component for the dashboard
//! 
//! This component displays a card with a title and content.

use leptos::*;

/// Card component for the dashboard
#[component]
pub fn Card(
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="dashboard-card">
            <div class="card-header">
                <h3 class="card-title">{title}</h3>
            </div>
            <div class="card-content">
                {children()}
            </div>
        </div>
    }
}
