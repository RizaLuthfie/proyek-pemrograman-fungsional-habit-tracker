<script setup>
import { computed } from "vue";

const props = defineProps({
  monthlyStats: {
    type: Object,
    default: null,
  },
  currentStreak: {
    type: Number,
    default: 0,
  },
});

const insights = computed(() => {
  const result = [];

  if (props.currentStreak > 0) {
    if (props.currentStreak >= 7) {
      result.push({
        message: `Streak ${props.currentStreak} hari berturut-turut! Luar biasa!`,
        type: "achievement",
      });
    } else {
      result.push({
        message: `Streak saat ini: ${props.currentStreak} hari. Pertahankan!`,
        type: "pattern",
      });
    }
  } else {
    result.push({
      message: "Belum ada streak. Mulai catat aktivitas hari ini!",
      type: "suggestion",
    });
  }

  if (props.monthlyStats?.insights) {
    props.monthlyStats.insights.forEach((insight) => {
      result.push({
        message: insight,
        type: insight.includes("sangat baik") || insight.includes("Luar biasa")
          ? "achievement"
          : insight.includes("coba") || insight.includes("tingkatkan")
          ? "suggestion"
          : "pattern",
      });
    });
  }

  return result;
});
</script>

<template>
  <div class="card">
    <div class="card-header">
      <h2>Insight</h2>
    </div>

    <div v-if="insights.length === 0" class="empty-state">
      <p>Belum ada insight tersedia.</p>
      <p>Catat lebih banyak aktivitas untuk mendapatkan insight.</p>
    </div>

    <div v-else class="insight-list">
      <div
        v-for="(insight, index) in insights"
        :key="index"
        class="insight-item"
        :class="`insight-${insight.type}`"
      >
        {{ insight.message }}
      </div>
    </div>
  </div>
</template>