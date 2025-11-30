use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habit {
    pub id: String,
    pub name: String,
    pub category: Category,
    pub timestamp: DateTime<Utc>,
    pub compliance_level: Option<u8>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Health,
    Productivity,
    Hygiene,
    Exercise,
    Sleep,
    Other,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Health => "health",
            Category::Productivity => "productivity",
            Category::Hygiene => "hygiene",
            Category::Exercise => "exercise",
            Category::Sleep => "sleep",
            Category::Other => "other",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "health" => Category::Health,
            "productivity" => Category::Productivity,
            "hygiene" => Category::Hygiene,
            "exercise" => Category::Exercise,
            "sleep" => Category::Sleep,
            _ => Category::Other,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitInput {
    pub name: String,
    pub category: String,
    pub timestamp: Option<String>,
    pub compliance_level: Option<u8>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,
    pub total_habits: usize,
    pub by_category: Vec<CategoryCount>,
    pub average_compliance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryCount {
    pub category: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyStats {
    pub week_start: String,
    pub week_end: String,
    pub days: Vec<DailyStats>,
    pub total_habits: usize,
    pub most_active_day: String,
    pub trend: Trend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyStats {
    pub month: String,
    pub year: i32,
    pub weeks: Vec<WeeklyStats>,
    pub total_habits: usize,
    pub consistency_percentage: f64,
    pub most_common_category: String,
    pub insights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Trend {
    Up,
    Down,
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub message: String,
    pub insight_type: InsightType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InsightType {
    Achievement,
    Pattern,
    Suggestion,
    Warning,
}