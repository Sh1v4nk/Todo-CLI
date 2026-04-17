use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub priority: Priority,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Priority {
    pub fn from_str(s: &str) -> Priority {
        match s.to_lowercase().as_str() {
            "high" | "h" => Priority::High,
            "low" | "l" => Priority::Low,
            _ => Priority::Medium,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Priority::High => "HIGH",
            Priority::Medium => "MED",
            Priority::Low => "LOW",
        }
    }
}
