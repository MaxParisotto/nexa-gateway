//! Sidebar component for the dashboard
//! 
//! This component displays the sidebar navigation of the dashboard.

use leptos::*;

/// Sidebar component for the dashboard
#[component]
pub fn Sidebar(
    #[prop(into)] active_page: String,
) -> impl IntoView {
    let nav_items = vec![
        ("home", "Home", "/", "Home"),
        ("metrics", "Metrics", "/metrics", "Metrics"),
        ("settings", "Settings", "/settings", "Settings"),
        ("logs", "Logs", "/logs", "Logs"),
    ];
    
    view! {
        <aside class="dashboard-sidebar">
            <nav class="sidebar-nav">
                <ul class="sidebar-nav-list">
                    {nav_items.into_iter().map(|(id, label, href, icon)| {
                        let is_active = active_page == id;
                        let class = if is_active { "sidebar-nav-item active" } else { "sidebar-nav-item" };
                        
                        view! {
                            <li class={class}>
                                <a href={href}>
                                    <span class="sidebar-icon">{icon}</span>
                                    <span class="sidebar-label">{label}</span>
                                </a>
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </nav>
            <div class="sidebar-footer">
                <div class="version-info">"Nexa Gateway v0.1.0"</div>
            </div>
        </aside>
    }
}
