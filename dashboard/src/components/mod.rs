//! UI components for the dashboard
//! 
//! This module contains reusable UI components for the dashboard.

pub mod metrics_chart;  // Make this public

// We don't have these components yet, but export them when they're ready
// pub mod header;
// pub mod sidebar;
// pub mod card;
// pub mod status_indicator;

// Export the MetricsChart component so it can be used directly
pub use metrics_chart::MetricsChart;
