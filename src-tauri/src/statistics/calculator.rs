use chrono::{Datelike, Duration, NaiveDate, Timelike, Utc, Weekday};
use rayon::prelude::*;
use std::collections::HashMap;

use crate::models::{
    CategoryCount, DailyStats, Habit, Insight, InsightType, MonthlyStats, Trend, WeeklyStats,
};

pub struct StatisticsCalculator;

impl StatisticsCalculator {
    pub fn calculate_daily_stats(habits: &[Habit], date: NaiveDate) -> DailyStats {
        let day_habits: Vec<&Habit> = habits
            .iter()
            .filter(|h| h.timestamp.date_naive() == date)
            .collect();

        let mut category_counts: HashMap<String, usize> = HashMap::new();
        let mut total_compliance: f64 = 0.0;
        let mut compliance_count: usize = 0;

        for habit in &day_habits {
            *category_counts
                .entry(habit.category.as_str().to_string())
                .or_insert(0) += 1;

            if let Some(level) = habit.compliance_level {
                total_compliance += level as f64;
                compliance_count += 1;
            }
        }

        let by_category: Vec<CategoryCount> = category_counts
            .into_iter()
            .map(|(category, count)| CategoryCount { category, count })
            .collect();

        let average_compliance = if compliance_count > 0 {
            total_compliance / compliance_count as f64
        } else {
            0.0
        };

        DailyStats {
            date: date.to_string(),
            total_habits: day_habits.len(),
            by_category,
            average_compliance,
        }
    }

    pub fn calculate_weekly_stats(habits: &[Habit], week_start: NaiveDate) -> WeeklyStats {
        let week_end = week_start + Duration::days(6);

        let days: Vec<DailyStats> = (0..7)
            .into_par_iter()
            .map(|i| {
                let date = week_start + Duration::days(i);
                Self::calculate_daily_stats(habits, date)
            })
            .collect();

        let total_habits: usize = days.iter().map(|d| d.total_habits).sum();

        let most_active_day = days
            .iter()
            .max_by_key(|d| d.total_habits)
            .map(|d| d.date.clone())
            .unwrap_or_default();

        let trend = Self::calculate_trend(&days);

        WeeklyStats {
            week_start: week_start.to_string(),
            week_end: week_end.to_string(),
            days,
            total_habits,
            most_active_day,
            trend,
        }
    }

    pub fn calculate_monthly_stats(habits: &[Habit], year: i32, month: u32) -> MonthlyStats {
        let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let last_day = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap() - Duration::days(1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap() - Duration::days(1)
        };

        let month_habits: Vec<&Habit> = habits
            .iter()
            .filter(|h| {
                let date = h.timestamp.date_naive();
                date >= first_day && date <= last_day
            })
            .collect();

        let mut weeks = Vec::new();
        let mut current_week_start = first_day;
        while current_week_start.weekday() != Weekday::Mon {
            current_week_start = current_week_start - Duration::days(1);
        }

        while current_week_start <= last_day {
            let week_habits: Vec<Habit> = month_habits
                .iter()
                .filter(|h| {
                    let date = h.timestamp.date_naive();
                    date >= current_week_start && date < current_week_start + Duration::days(7)
                })
                .cloned()
                .cloned()
                .collect();

            weeks.push(Self::calculate_weekly_stats(&week_habits, current_week_start));
            current_week_start = current_week_start + Duration::days(7);
        }

        let total_habits = month_habits.len();

        let mut category_counts: HashMap<String, usize> = HashMap::new();
        for habit in &month_habits {
            *category_counts
                .entry(habit.category.as_str().to_string())
                .or_insert(0) += 1;
        }

        let most_common_category = category_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(cat, _)| cat)
            .unwrap_or_else(|| "none".to_string());

        let days_in_month = (last_day - first_day).num_days() + 1;
        let active_days = habits
            .iter()
            .filter(|h| {
                let date = h.timestamp.date_naive();
                date >= first_day && date <= last_day
            })
            .map(|h| h.timestamp.date_naive())
            .collect::<std::collections::HashSet<_>>()
            .len();

        let consistency_percentage = (active_days as f64 / days_in_month as f64) * 100.0;

        let insights = Self::generate_insights(&month_habits, consistency_percentage);

        let month_name = match month {
            1 => "Januari",
            2 => "Februari",
            3 => "Maret",
            4 => "April",
            5 => "Mei",
            6 => "Juni",
            7 => "Juli",
            8 => "Agustus",
            9 => "September",
            10 => "Oktober",
            11 => "November",
            12 => "Desember",
            _ => "Unknown",
        };

        MonthlyStats {
            month: month_name.to_string(),
            year,
            weeks,
            total_habits,
            consistency_percentage,
            most_common_category,
            insights: insights.iter().map(|i| i.message.clone()).collect(),
        }
    }

    fn calculate_trend(days: &[DailyStats]) -> Trend {
        if days.len() < 2 {
            return Trend::Stable;
        }

        let mid = days.len() / 2;
        let first_half: usize = days[..mid].iter().map(|d| d.total_habits).sum();
        let second_half: usize = days[mid..].iter().map(|d| d.total_habits).sum();

        let threshold = (first_half as f64 * 0.1) as usize;

        if second_half > first_half + threshold {
            Trend::Up
        } else if second_half + threshold < first_half {
            Trend::Down
        } else {
            Trend::Stable
        }
    }

    fn generate_insights(habits: &[&Habit], consistency: f64) -> Vec<Insight> {
        let mut insights = Vec::new();

        if consistency >= 80.0 {
            insights.push(Insight {
                message: format!(
                    "Konsistensi sangat baik! Kamu aktif {:.1}% dari hari dalam bulan ini.",
                    consistency
                ),
                insight_type: InsightType::Achievement,
            });
        } else if consistency >= 50.0 {
            insights.push(Insight {
                message: format!(
                    "Konsistensi cukup baik ({:.1}%). Terus tingkatkan!",
                    consistency
                ),
                insight_type: InsightType::Pattern,
            });
        } else {
            insights.push(Insight {
                message: format!(
                    "Konsistensi masih rendah ({:.1}%). Coba buat pengingat harian.",
                    consistency
                ),
                insight_type: InsightType::Suggestion,
            });
        }

        let mut hour_counts: HashMap<u32, usize> = HashMap::new();
        for habit in habits {
            let hour = habit.timestamp.hour();
            *hour_counts.entry(hour).or_insert(0) += 1;
        }

        if let Some((most_active_hour, _)) = hour_counts.into_iter().max_by_key(|(_, count)| *count)
        {
            let time_period = match most_active_hour {
                5..=11 => "pagi",
                12..=17 => "siang/sore",
                18..=21 => "malam",
                _ => "larut malam",
            };
            insights.push(Insight {
                message: format!(
                    "Kamu paling aktif di waktu {} (sekitar jam {}).",
                    time_period, most_active_hour
                ),
                insight_type: InsightType::Pattern,
            });
        }

        let mut weekday_counts: HashMap<Weekday, usize> = HashMap::new();
        for habit in habits {
            let weekday = habit.timestamp.weekday();
            *weekday_counts.entry(weekday).or_insert(0) += 1;
        }

        if let Some((most_active_weekday, _)) =
            weekday_counts.into_iter().max_by_key(|(_, count)| *count)
        {
            let day_name = match most_active_weekday {
                Weekday::Mon => "Senin",
                Weekday::Tue => "Selasa",
                Weekday::Wed => "Rabu",
                Weekday::Thu => "Kamis",
                Weekday::Fri => "Jumat",
                Weekday::Sat => "Sabtu",
                Weekday::Sun => "Minggu",
            };
            insights.push(Insight {
                message: format!("Hari paling aktif adalah {}.", day_name),
                insight_type: InsightType::Pattern,
            });
        }

        insights
    }

    pub fn get_current_streak(habits: &[Habit]) -> usize {
        if habits.is_empty() {
            return 0;
        }

        let mut dates: Vec<NaiveDate> = habits.iter().map(|h| h.timestamp.date_naive()).collect();
        dates.sort();
        dates.dedup();
        dates.reverse();

        let today = Utc::now().date_naive();
        let yesterday = today - Duration::days(1);

        if dates.is_empty() || (dates[0] != today && dates[0] != yesterday) {
            return 0;
        }

        let mut streak = 1;
        for i in 1..dates.len() {
            if dates[i - 1] - dates[i] == Duration::days(1) {
                streak += 1;
            } else {
                break;
            }
        }

        streak
    }
}