//! Header component for the dashboard
//! 
//! This component displays the top navigation bar of the dashboard.

use leptos::*;

/// Header component for the dashboard
#[component]
pub fn Header(
    #[prop(into)] title: String,
) -> impl IntoView {
    view! {
        <header class="dashboard-header">
            <div class="logo-container">
                <h1 class="logo">{title}</h1>
            </div>
            <nav class="header-nav">
                <ul class="nav-list">
                    <li class="nav-item"><a href="/">"Home"</a></li>
                    <li class="nav-item"><a href="/metrics">"Metrics"</a></li>
                    <li class="nav-item"><a href="/settings">"Settings"</a></li>
                    <li class="nav-item"><a href="/logs">"Logs"</a></li>
                </ul>
            </nav>
            <div class="user-menu">
                <button class="user-button">
                    <span class="user-name">"Admin"</span>
                    <span class="user-icon">"User"</span>
                </button>
            </div>
        </header>
    }
}
