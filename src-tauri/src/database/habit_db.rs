use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result as SqliteResult};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::models::{Category, Habit};

pub struct HabitDatabase {
    conn: Mutex<Connection>,
}

impl HabitDatabase {
    pub fn new(app_data_dir: PathBuf) -> SqliteResult<Self> {
        std::fs::create_dir_all(&app_data_dir).ok();
        let db_path = app_data_dir.join("habits.db");
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS habits (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                category TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                compliance_level INTEGER,
                notes TEXT,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_habits_timestamp ON habits(timestamp)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_habits_category ON habits(category)",
            [],
        )?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn insert_habit(&self, habit: &Habit) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO habits (id, name, category, timestamp, compliance_level, notes, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                habit.id,
                habit.name,
                habit.category.as_str(),
                habit.timestamp.to_rfc3339(),
                habit.compliance_level,
                habit.notes,
                Utc::now().to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn get_all_habits(&self) -> SqliteResult<Vec<Habit>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, category, timestamp, compliance_level, notes 
             FROM habits ORDER BY timestamp DESC",
        )?;

        let habits = stmt
            .query_map([], |row| {
                let timestamp_str: String = row.get(3)?;
                let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                let category_str: String = row.get(2)?;

                Ok(Habit {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    category: Category::from_str(&category_str),
                    timestamp,
                    compliance_level: row.get(4)?,
                    notes: row.get(5)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(habits)
    }

    pub fn get_habits_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> SqliteResult<Vec<Habit>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, category, timestamp, compliance_level, notes 
             FROM habits 
             WHERE timestamp >= ?1 AND timestamp <= ?2
             ORDER BY timestamp DESC",
        )?;

        let habits = stmt
            .query_map(params![start.to_rfc3339(), end.to_rfc3339()], |row| {
                let timestamp_str: String = row.get(3)?;
                let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                let category_str: String = row.get(2)?;

                Ok(Habit {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    category: Category::from_str(&category_str),
                    timestamp,
                    compliance_level: row.get(4)?,
                    notes: row.get(5)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(habits)
    }

    pub fn get_habits_by_category(&self, category: &str) -> SqliteResult<Vec<Habit>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, category, timestamp, compliance_level, notes 
             FROM habits 
             WHERE category = ?1
             ORDER BY timestamp DESC",
        )?;

        let habits = stmt
            .query_map(params![category], |row| {
                let timestamp_str: String = row.get(3)?;
                let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                let category_str: String = row.get(2)?;

                Ok(Habit {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    category: Category::from_str(&category_str),
                    timestamp,
                    compliance_level: row.get(4)?,
                    notes: row.get(5)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(habits)
    }

    pub fn delete_habit(&self, id: &str) -> SqliteResult<bool> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM habits WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    pub fn get_habit_count(&self) -> SqliteResult<usize> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM habits", [], |row| row.get(0))?;
        Ok(count as usize)
    }
}