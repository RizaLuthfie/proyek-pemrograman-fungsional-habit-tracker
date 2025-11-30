pub mod commands;
pub mod database;
pub mod models;
pub mod statistics;

use tauri::Manager;

use commands::{
    add_habit, delete_habit, get_all_habits, get_categories, get_current_streak, get_daily_stats,
    get_habit_count, get_habits_by_category, get_habits_by_date_range, get_monthly_stats,
    get_this_week_habits, get_today_habits, get_weekly_stats, AppState,
};
use database::HabitDatabase;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            let db = HabitDatabase::new(app_data_dir).expect("Failed to initialize database");

            app.manage(AppState { db });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_habit,
            get_all_habits,
            get_habits_by_category,
            get_habits_by_date_range,
            delete_habit,
            get_daily_stats,
            get_weekly_stats,
            get_monthly_stats,
            get_current_streak,
            get_today_habits,
            get_this_week_habits,
            get_habit_count,
            get_categories,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}