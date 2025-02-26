//! Not found page for the dashboard
//! 
//! This page is displayed when a route is not found.

use leptos::*;
use crate::components::{Header, Sidebar};

/// Not found page component
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="dashboard-container">
            <Header title="Nexa Gateway Dashboard" />
            <div class="dashboard-content">
                <Sidebar active_page="" />
                <main class="main-content not-found-page">
                    <div class="not-found-container">
                        <h1 class="not-found-title">"404"</h1>
                        <h2 class="not-found-subtitle">"Page Not Found"</h2>
                        <p class="not-found-message">
                            "The page you are looking for does not exist or has been moved."
                        </p>
                        <a href="/" class="not-found-link">"Return to Dashboard"</a>
                    </div>
                </main>
            </div>
        </div>
    }
}
