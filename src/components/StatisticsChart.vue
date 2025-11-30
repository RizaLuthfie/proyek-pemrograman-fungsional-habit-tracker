<script setup>
import { ref, computed, watch, onMounted } from "vue";
import { Bar, Line } from "vue-chartjs";
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  BarElement,
  LineElement,
  CategoryScale,
  LinearScale,
  PointElement,
  Filler,
} from "chart.js";

ChartJS.register(
  Title,
  Tooltip,
  Legend,
  BarElement,
  LineElement,
  CategoryScale,
  LinearScale,
  PointElement,
  Filler
);

const props = defineProps({
  weeklyStats: {
    type: Object,
    default: null,
  },
  monthlyStats: {
    type: Object,
    default: null,
  },
});

const activeTab = ref("weekly");

const dayNames = ["Sen", "Sel", "Rab", "Kam", "Jum", "Sab", "Min"];

const weeklyChartData = computed(() => {
  if (!props.weeklyStats?.days) {
    return {
      labels: dayNames,
      datasets: [
        {
          label: "Jumlah Aktivitas",
          backgroundColor: "#4f46e5",
          borderRadius: 6,
          data: [0, 0, 0, 0, 0, 0, 0],
        },
      ],
    };
  }

  const data = props.weeklyStats.days.map((d) => d.total_habits);

  return {
    labels: dayNames,
    datasets: [
      {
        label: "Jumlah Aktivitas",
        backgroundColor: "#4f46e5",
        borderRadius: 6,
        data: data,
      },
    ],
  };
});

const monthlyChartData = computed(() => {
  if (!props.monthlyStats?.weeks) {
    return {
      labels: [],
      datasets: [
        {
          label: "Aktivitas per Minggu",
          borderColor: "#4f46e5",
          backgroundColor: "rgba(79, 70, 229, 0.1)",
          fill: true,
          tension: 0.4,
          data: [],
        },
      ],
    };
  }

  const labels = props.monthlyStats.weeks.map((_, i) => `Minggu ${i + 1}`);
  const data = props.monthlyStats.weeks.map((w) => w.total_habits);

  return {
    labels: labels,
    datasets: [
      {
        label: "Aktivitas per Minggu",
        borderColor: "#4f46e5",
        backgroundColor: "rgba(79, 70, 229, 0.1)",
        fill: true,
        tension: 0.4,
        pointBackgroundColor: "#4f46e5",
        pointRadius: 4,
        data: data,
      },
    ],
  };
});

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      display: false,
    },
    tooltip: {
      backgroundColor: "#1e293b",
      titleColor: "#fff",
      bodyColor: "#fff",
      padding: 12,
      cornerRadius: 8,
    },
  },
  scales: {
    x: {
      grid: {
        display: false,
      },
    },
    y: {
      beginAtZero: true,
      ticks: {
        stepSize: 1,
      },
      grid: {
        color: "#f1f5f9",
      },
    },
  },
};
</script>

<template>
  <div class="card">
    <div class="card-header">
      <h2>Statistik</h2>
    </div>

    <div class="tab-buttons">
      <button
        class="tab-btn"
        :class="{ active: activeTab === 'weekly' }"
        @click="activeTab = 'weekly'"
      >
        Mingguan
      </button>
      <button
        class="tab-btn"
        :class="{ active: activeTab === 'monthly' }"
        @click="activeTab = 'monthly'"
      >
        Bulanan
      </button>
    </div>

    <div class="chart-container">
      <Bar
        v-if="activeTab === 'weekly'"
        :data="weeklyChartData"
        :options="chartOptions"
      />
      <Line
        v-else
        :data="monthlyChartData"
        :options="chartOptions"
      />
    </div>

    <div v-if="activeTab === 'weekly' && weeklyStats" class="stats-summary">
      <p>
        Total minggu ini: <strong>{{ weeklyStats.total_habits }}</strong> aktivitas
      </p>
      <p v-if="weeklyStats.most_active_day">
        Hari paling aktif: <strong>{{ weeklyStats.most_active_day }}</strong>
      </p>
      <p>
        Tren:
        <span :class="`trend ${weeklyStats.trend}`">
          {{ weeklyStats.trend === "up" ? "Meningkat" : weeklyStats.trend === "down" ? "Menurun" : "Stabil" }}
        </span>
      </p>
    </div>

    <div v-else-if="activeTab === 'monthly' && monthlyStats" class="stats-summary">
      <p>
        Total bulan {{ monthlyStats.month }}:
        <strong>{{ monthlyStats.total_habits }}</strong> aktivitas
      </p>
      <p>
        Konsistensi:
        <strong>{{ monthlyStats.consistency_percentage.toFixed(1) }}%</strong>
      </p>
      <p>
        Kategori terbanyak:
        <strong>{{ monthlyStats.most_common_category }}</strong>
      </p>
    </div>
  </div>
</template>

<style scoped>
.stats-summary {
  margin-top: 20px;
  padding-top: 16px;
  border-top: 1px solid var(--color-border);
}

.stats-summary p {
  font-size: 14px;
  color: var(--color-text-secondary);
  margin-bottom: 8px;
}

.stats-summary strong {
  color: var(--color-text);
}
</style>