import { reactive, readonly } from "vue";
import { invoke } from "@tauri-apps/api/core";

const state = reactive({
  habits: [],
  todayHabits: [],
  weeklyStats: null,
  monthlyStats: null,
  currentStreak: 0,
  loading: false,
  error: null,
  categories: [],
});

const formatDateForDisplay = (timestamp) => {
  const date = new Date(timestamp);
  return date.toLocaleDateString("id-ID", {
    day: "numeric",
    month: "short",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
};

// MODIFIED: Fixed timezone handling untuk week start calculation
const getWeekStart = () => {
  const now = new Date();
  const year = now.getFullYear();
  const month = now.getMonth();
  const date = now.getDate();
  const dayOfWeek = now.getDay();
  
  const daysToMonday = dayOfWeek === 0 ? 6 : dayOfWeek - 1;
  const monday = new Date(year, month, date - daysToMonday);
  
  const mondayYear = monday.getFullYear();
  const mondayMonth = String(monday.getMonth() + 1).padStart(2, '0');
  const mondayDate = String(monday.getDate()).padStart(2, '0');
  
  return `${mondayYear}-${mondayMonth}-${mondayDate}`;
};

const actions = {
  async loadCategories() {
    try {
      state.categories = await invoke("get_categories");
    } catch (error) {
      console.error("Failed to load categories:", error);
      state.categories = [
        "health",
        "productivity",
        "hygiene",
        "exercise",
        "sleep",
        "other",
      ];
    }
  },

  async loadAllHabits() {
    state.loading = true;
    state.error = null;
    try {
      state.habits = await invoke("get_all_habits");
    } catch (error) {
      state.error = error;
      console.error("Failed to load habits:", error);
    } finally {
      state.loading = false;
    }
  },

  async loadTodayHabits() {
    try {
      state.todayHabits = await invoke("get_today_habits");
    } catch (error) {
      console.error("Failed to load today habits:", error);
    }
  },

  async addHabit(habitInput) {
    state.loading = true;
    state.error = null;
    try {
      const newHabit = await invoke("add_habit", { input: habitInput });
      state.habits.unshift(newHabit);
      
      // Refresh semua data
      await this.refreshAllData();
      
      return newHabit;
    } catch (error) {
      state.error = error;
      console.error("Failed to add habit:", error);
      throw error;
    } finally {
      state.loading = false;
    }
  },

  async deleteHabit(id) {
    try {
      const success = await invoke("delete_habit", { id });
      if (success) {
        state.habits = state.habits.filter((h) => h.id !== id);
        state.todayHabits = state.todayHabits.filter((h) => h.id !== id);
        
        // Refresh statistik
        await this.refreshAllData();
      }
      return success;
    } catch (error) {
      console.error("Failed to delete habit:", error);
      throw error;
    }
  },

  async loadWeeklyStats() {
    try {
      const weekStart = getWeekStart();
      state.weeklyStats = await invoke("get_weekly_stats", { weekStart });
    } catch (error) {
      console.error("Failed to load weekly stats:", error);
    }
  },

  async loadMonthlyStats() {
    try {
      const now = new Date();
      state.monthlyStats = await invoke("get_monthly_stats", {
        year: now.getFullYear(),
        month: now.getMonth() + 1,
      });
    } catch (error) {
      console.error("Failed to load monthly stats:", error);
    }
  },

  async loadCurrentStreak() {
    try {
      state.currentStreak = await invoke("get_current_streak");
    } catch (error) {
      console.error("Failed to load streak:", error);
    }
  },

  async refreshAllData() {
    await Promise.all([
      this.loadTodayHabits(),
      this.loadWeeklyStats(),
      this.loadMonthlyStats(),
      this.loadCurrentStreak(),
    ]);
  },

  async loadAllData() {
    await Promise.all([
      this.loadCategories(),
      this.loadAllHabits(),
      this.loadTodayHabits(),
      this.loadWeeklyStats(),
      this.loadMonthlyStats(),
      this.loadCurrentStreak(),
    ]);
  },
};

export const useHabitStore = () => ({
  state: readonly(state),
  ...actions,
  formatDateForDisplay,
});