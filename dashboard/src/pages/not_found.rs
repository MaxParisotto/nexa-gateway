//! Not Found (404) page for the dashboard
//!
//! This page is shown when a user navigates to an invalid route.

use leptos::*;
use leptos::prelude::*;
use leptos::html::ElementChild;  // Add ElementChild trait
use leptos_router::components::A;  // Import A component specifically
// Don't import components that don't exist yet
// use crate::components::{Header, Sidebar};

/// Not Found page component
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="not-found-page">
            <h1>"404"</h1>
            <h2>"Page Not Found"</h2>
            <p>"The page you are looking for does not exist."</p>
            <A href="/" attr:class="back-link">"Go to Dashboard"</A>
        </div>
    }
}
