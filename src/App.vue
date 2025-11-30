<script setup>
import { onMounted, computed } from "vue";
import { useHabitStore } from "./stores/habitStore";
import HabitForm from "./components/HabitForm.vue";
import HabitList from "./components/HabitList.vue";
import StatisticsChart from "./components/StatisticsChart.vue";
import InsightPanel from "./components/InsightPanel.vue";

const store = useHabitStore();

const todayCount = computed(() => store.state.todayHabits.length);
const weeklyTotal = computed(() => store.state.weeklyStats?.total_habits || 0);
const consistency = computed(() =>
  store.state.monthlyStats?.consistency_percentage?.toFixed(1) || "0"
);
const streak = computed(() => store.state.currentStreak);

const trendLabel = computed(() => {
  const trend = store.state.weeklyStats?.trend;
  if (trend === "up") return "Meningkat";
  if (trend === "down") return "Menurun";
  return "Stabil";
});

const trendClass = computed(() => {
  return store.state.weeklyStats?.trend || "stable";
});

onMounted(async () => {
  await store.loadAllData();
});
</script>

<template>
  <div class="app-container">
    <header class="app-header">
      <h1>Habit Tracker</h1>
      <p>Catat dan analisis kebiasaan harian Anda secara offline</p>
    </header>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="label">Aktivitas Hari Ini</div>
        <div class="value">{{ todayCount }}</div>
      </div>

      <div class="stat-card">
        <div class="label">Total Minggu Ini</div>
        <div class="value">{{ weeklyTotal }}</div>
        <div class="trend" :class="trendClass">{{ trendLabel }}</div>
      </div>

      <div class="stat-card">
        <div class="label">Konsistensi Bulan Ini</div>
        <div class="value">{{ consistency }}%</div>
      </div>

      <div class="stat-card">
        <div class="label">Streak</div>
        <div class="value">{{ streak }} hari</div>
      </div>
    </div>

    <div v-if="store.state.loading" class="loading">
      Memuat data...
    </div>

    <div v-else class="main-content">
      <div class="sidebar">
        <HabitForm />
        <InsightPanel
          :monthly-stats="store.state.monthlyStats"
          :current-streak="store.state.currentStreak"
        />
      </div>

      <div class="content-area">
        <StatisticsChart
          :weekly-stats="store.state.weeklyStats"
          :monthly-stats="store.state.monthlyStats"
        />

        <HabitList
          :habits="store.state.todayHabits"
          title="Aktivitas Hari Ini"
        />

        <HabitList
          :habits="store.state.habits"
          title="Semua Riwayat"
        />
      </div>
    </div>
  </div>
</template>