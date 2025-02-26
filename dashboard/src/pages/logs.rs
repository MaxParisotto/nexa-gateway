//! Logs page for the dashboard
//! 
//! This page displays system logs and events.

use leptos::*;
use crate::components::{Header, Sidebar, Card};

/// Logs page component
#[component]
pub fn LogsPage() -> impl IntoView {
    // Sample log entries
    let log_entries = vec![
        ("2025-02-26 11:30:12", "INFO", "Gateway started successfully"),
        ("2025-02-26 11:30:15", "INFO", "Connected to database"),
        ("2025-02-26 11:32:45", "INFO", "User admin logged in"),
        ("2025-02-26 11:35:22", "WARN", "High CPU usage detected (78%)"),
        ("2025-02-26 11:38:17", "INFO", "Cache cleared"),
        ("2025-02-26 11:40:03", "ERROR", "Failed to connect to external API: timeout"),
        ("2025-02-26 11:42:51", "INFO", "Scheduled maintenance started"),
        ("2025-02-26 11:43:12", "INFO", "Configuration updated"),
    ];
    
    view! {
        <div class="dashboard-container">
            <Header title="Nexa Gateway Dashboard" />
            <div class="dashboard-content">
                <Sidebar active_page="logs" />
                <main class="main-content">
                    <h1>"System Logs"</h1>
                    
                    <div class="logs-filter">
                        <div class="filter-group">
                            <label for="log-level-filter">"Log Level:"</label>
                            <select id="log-level-filter">
                                <option selected="true">"All"</option>
                                <option>"Info"</option>
                                <option>"Warning"</option>
                                <option>"Error"</option>
                                <option>"Debug"</option>
                            </select>
                        </div>
                        
                        <div class="filter-group">
                            <label for="log-search">"Search:"</label>
                            <input type="text" id="log-search" placeholder="Filter logs..." />
                        </div>
                    </div>
                    
                    <Card title="Recent Logs">
                        <div class="logs-table-container">
                            <table class="logs-table">
                                <thead>
                                    <tr>
                                        <th>"Timestamp"</th>
                                        <th>"Level"</th>
                                        <th>"Message"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {log_entries.into_iter().map(|(timestamp, level, message)| {
                                        let level_class = match level {
                                            "ERROR" => "log-level-error",
                                            "WARN" => "log-level-warn",
                                            "INFO" => "log-level-info",
                                            "DEBUG" => "log-level-debug",
                                            _ => "",
                                        };
                                        
                                        view! {
                                            <tr>
                                                <td class="log-timestamp">{timestamp}</td>
                                                <td class={format!("log-level {}", level_class)}>{level}</td>
                                                <td class="log-message">{message}</td>
                                            </tr>
                                        }
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </table>
                        </div>
                    </Card>
                    
                    <div class="logs-actions">
                        <button class="action-button">"Refresh"</button>
                        <button class="action-button">"Export Logs"</button>
                        <button class="action-button">"Clear Logs"</button>
                    </div>
                </main>
            </div>
        </div>
    }
}
