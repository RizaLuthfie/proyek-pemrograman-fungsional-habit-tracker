use chrono::{Datelike, Duration, Local, NaiveDate, Utc};
use tauri::State;
use uuid::Uuid;

use crate::database::HabitDatabase;
use crate::models::{Category, DailyStats, Habit, HabitInput, MonthlyStats, WeeklyStats};
use crate::statistics::StatisticsCalculator;

pub struct AppState {
    pub db: HabitDatabase,
}

// MODIFIED: Fixed default timestamp menggunakan Local timezone
#[tauri::command]
pub fn add_habit(input: HabitInput, state: State<AppState>) -> Result<Habit, String> {
    let timestamp = input
        .timestamp
        .as_ref()
        .and_then(|ts| chrono::DateTime::parse_from_rfc3339(ts).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|| Local::now().with_timezone(&Utc));

    let habit = Habit {
        id: Uuid::new_v4().to_string(),
        name: input.name,
        category: Category::from_str(&input.category),
        timestamp,
        compliance_level: input.compliance_level,
        notes: input.notes,
    };

    state
        .db
        .insert_habit(&habit)
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(habit)
}

#[tauri::command]
pub fn get_all_habits(state: State<AppState>) -> Result<Vec<Habit>, String> {
    state
        .db
        .get_all_habits()
        .map_err(|e| format!("Database error: {}", e))
}

#[tauri::command]
pub fn get_habits_by_category(category: String, state: State<AppState>) -> Result<Vec<Habit>, String> {
    state
        .db
        .get_habits_by_category(&category)
        .map_err(|e| format!("Database error: {}", e))
}

#[tauri::command]
pub fn get_habits_by_date_range(
    start: String,
    end: String,
    state: State<AppState>,
) -> Result<Vec<Habit>, String> {
    let start_dt = chrono::DateTime::parse_from_rfc3339(&start)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| format!("Invalid start date: {}", e))?;

    let end_dt = chrono::DateTime::parse_from_rfc3339(&end)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| format!("Invalid end date: {}", e))?;

    state
        .db
        .get_habits_by_date_range(start_dt, end_dt)
        .map_err(|e| format!("Database error: {}", e))
}

#[tauri::command]
pub fn delete_habit(id: String, state: State<AppState>) -> Result<bool, String> {
    state
        .db
        .delete_habit(&id)
        .map_err(|e| format!("Database error: {}", e))
}

#[tauri::command]
pub fn get_daily_stats(date: String, state: State<AppState>) -> Result<DailyStats, String> {
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format: {}", e))?;

    let habits = state
        .db
        .get_all_habits()
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(StatisticsCalculator::calculate_daily_stats(&habits, date))
}

#[tauri::command]
pub fn get_weekly_stats(week_start: String, state: State<AppState>) -> Result<WeeklyStats, String> {
    let start_date = NaiveDate::parse_from_str(&week_start, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format: {}", e))?;

    let habits = state
        .db
        .get_all_habits()
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(StatisticsCalculator::calculate_weekly_stats(
        &habits,
        start_date,
    ))
}

#[tauri::command]
pub fn get_monthly_stats(
    year: i32,
    month: u32,
    state: State<AppState>,
) -> Result<MonthlyStats, String> {
    let habits = state
        .db
        .get_all_habits()
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(StatisticsCalculator::calculate_monthly_stats(
        &habits, year, month,
    ))
}

#[tauri::command]
pub fn get_current_streak(state: State<AppState>) -> Result<usize, String> {
    let habits = state
        .db
        .get_all_habits()
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(StatisticsCalculator::get_current_streak(&habits))
}

// MODIFIED: Fixed timezone - gunakan Local untuk today calculation
#[tauri::command]
pub fn get_today_habits(state: State<AppState>) -> Result<Vec<Habit>, String> {
    let today = Local::now();
    let start_of_day = today
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap()
        .with_timezone(&Utc);
    let end_of_day = today
        .date_naive()
        .and_hms_opt(23, 59, 59)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap()
        .with_timezone(&Utc);

    state
        .db
        .get_habits_by_date_range(start_of_day, end_of_day)
        .map_err(|e| format!("Database error: {}", e))
}

#[tauri::command]
pub fn get_this_week_habits(state: State<AppState>) -> Result<Vec<Habit>, String> {
    let today = Local::now();
    let days_since_monday = today.weekday().num_days_from_monday() as i64;
    let monday = today - Duration::days(days_since_monday);

    let start_of_week = monday
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap()
        .with_timezone(&Utc);
    let end_of_week = (monday + Duration::days(6))
        .date_naive()
        .and_hms_opt(23, 59, 59)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap()
        .with_timezone(&Utc);

    state
        .db
        .get_habits_by_date_range(start_of_week, end_of_week)
        .map_err(|e| format!("Database error: {}", e))
}

#[tauri::command]
pub fn get_habit_count(state: State<AppState>) -> Result<usize, String> {
    state
        .db
        .get_habit_count()
        .map_err(|e| format!("Database error: {}", e))
}

#[tauri::command]
pub fn get_categories() -> Vec<String> {
    vec![
        "health".to_string(),
        "productivity".to_string(),
        "hygiene".to_string(),
        "exercise".to_string(),
        "sleep".to_string(),
        "other".to_string(),
    ]
}