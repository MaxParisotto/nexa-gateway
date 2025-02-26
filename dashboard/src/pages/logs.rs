//! Logs page for the dashboard
//!
//! This page displays logs from the gateway.

use leptos::*;
use wasm_bindgen::prelude::*;
use leptos::prelude::*;
use leptos::html::ElementChild;  // Add ElementChild trait

/// Log entry data structure
#[derive(Clone)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

/// Log level enum
#[derive(Clone, PartialEq)]
enum LogLevel {
    All,
    Error,
    Warning,
    Info,
    Debug,
}

// Quick helper function for logging to console
fn log_to_console(msg: &str) {
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }
    log(msg);
}

/// Logs page component
#[component]
pub fn LogsPage() -> impl IntoView {
    // In a real application, we would fetch logs from an API
    // For now, we'll use static data
    let logs = vec![
        LogEntry {
            timestamp: "2025-02-26 13:11:19".to_string(),
            level: "INFO".to_string(),
            message: "Starting Nexa Gateway with UI".to_string(),
        },
        LogEntry {
            timestamp: "2025-02-26 13:11:19".to_string(),
            level: "INFO".to_string(),
            message: "Starting API server on port 3000".to_string(),
        },
        LogEntry {
            timestamp: "2025-02-26 13:11:19".to_string(),
            level: "INFO".to_string(),
            message: "Starting dashboard UI on port 3001".to_string(),
        },
        LogEntry {
            timestamp: "2025-02-26 13:11:19".to_string(),
            level: "INFO".to_string(),
            message: "Creating gateway application".to_string(),
        },
        LogEntry {
            timestamp: "2025-02-26 13:11:19".to_string(),
            level: "INFO".to_string(),
            message: "API server listening on 0.0.0.0:3000".to_string(),
        },
        LogEntry {
            timestamp: "2025-02-26 13:11:19".to_string(),
            level: "INFO".to_string(),
            message: "Dashboard UI listening on 0.0.0.0:3001".to_string(),
        },
    ];
    
    // Reactive signal for selected log level
    let (selected_level, set_selected_level) = signal(LogLevel::All);
    
    // Function to download logs
    let download_logs = move |_| {
        // In a real application, this would trigger a download
        // For now, we'll just log to the console
        log_to_console("Downloading logs...");
    };
    
    // Handler for level selection changes
    let handle_level_change = move |ev| {
        match event_target_value(&ev).as_str() {
            "Error" => set_selected_level.set(LogLevel::Error),
            "Warning" => set_selected_level.set(LogLevel::Warning),
            "Info" => set_selected_level.set(LogLevel::Info),
            "Debug" => set_selected_level.set(LogLevel::Debug),
            _ => set_selected_level.set(LogLevel::All),
        }
    };
    
    view! {
        <div class="page">
            <h1>"Gateway Logs"</h1>
            
            <div class="log-controls">
                <select on:change=handle_level_change>
                    <option selected=move || selected_level.get() == LogLevel::All>"All Levels"</option>
                    <option selected=move || selected_level.get() == LogLevel::Error>"Error"</option>
                    <option selected=move || selected_level.get() == LogLevel::Warning>"Warning"</option>
                    <option selected=move || selected_level.get() == LogLevel::Info>"Info"</option>
                    <option selected=move || selected_level.get() == LogLevel::Debug>"Debug"</option>
                </select>
                
                <button>"Refresh"</button>
                <button on:click=download_logs>"Download Logs"</button>
            </div>
            
            <div class="logs-container">
                {logs.into_iter().map(|log| {
                    view! {
                        <div class=format!("log-entry {}", log.level.clone().to_lowercase())>  // Clone here
                            <span class="log-timestamp">{log.timestamp}</span>
                            <span class=format!("log-level {}", log.level.clone().to_lowercase())>{log.level.clone()}</span>  // Clone here and here
                            <span class="log-message">{log.message}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
