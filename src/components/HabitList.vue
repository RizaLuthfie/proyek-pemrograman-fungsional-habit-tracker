<script setup>
import { computed } from "vue";
import { useHabitStore } from "../stores/habitStore";

const props = defineProps({
  habits: {
    type: Array,
    required: true,
  },
  title: {
    type: String,
    default: "Riwayat Aktivitas",
  },
  showDelete: {
    type: Boolean,
    default: true,
  },
});

const store = useHabitStore();

const categoryLabels = {
  health: "Kesehatan",
  productivity: "Produktivitas",
  hygiene: "Kebersihan",
  exercise: "Olahraga",
  sleep: "Tidur",
  other: "Lainnya",
};

const sortedHabits = computed(() => {
  return [...props.habits].sort(
    (a, b) => new Date(b.timestamp) - new Date(a.timestamp)
  );
});

const formatTime = (timestamp) => {
  const date = new Date(timestamp);
  return date.toLocaleTimeString("id-ID", {
    hour: "2-digit",
    minute: "2-digit",
  });
};

const formatDate = (timestamp) => {
  const date = new Date(timestamp);
  const today = new Date();
  const yesterday = new Date(today);
  yesterday.setDate(yesterday.getDate() - 1);

  if (date.toDateString() === today.toDateString()) {
    return "Hari ini";
  } else if (date.toDateString() === yesterday.toDateString()) {
    return "Kemarin";
  }

  return date.toLocaleDateString("id-ID", {
    day: "numeric",
    month: "short",
    year: "numeric",
  });
};

const handleDelete = async (id, habitName) => {
  if (confirm(`Apakah Anda yakin ingin menghapus "${habitName}"?`)) {
    try {
      await store.deleteHabit(id);
    } catch (error) {
      console.error("Gagal menghapus:", error);
      alert("Gagal menghapus aktivitas. Silakan coba lagi.");
    }
  }
};
</script>

<template>
  <div class="card">
    <div class="card-header">
      <h2>{{ title }}</h2>
      <span v-if="habits.length > 0" class="badge">
        {{ habits.length }} aktivitas
      </span>
    </div>

    <div v-if="sortedHabits.length === 0" class="empty-state">
      <p>Belum ada aktivitas tercatat.</p>
      <p>Mulai catat aktivitas pertama Anda!</p>
    </div>

    <div v-else class="habit-list">
      <div
        v-for="habit in sortedHabits"
        :key="habit.id"
        class="habit-item"
      >
        <div class="habit-info">
          <div class="habit-name">{{ habit.name }}</div>
          <div class="habit-meta">
            <span
              class="category-badge"
              :class="`category-${habit.category}`"
            >
              {{ categoryLabels[habit.category] || habit.category }}
            </span>
            <span class="habit-date">{{ formatDate(habit.timestamp) }}</span>
            <span class="habit-time">{{ formatTime(habit.timestamp) }}</span>
          </div>
          <div v-if="habit.notes" class="habit-notes">
            {{ habit.notes }}
          </div>
        </div>

        <button
          v-if="showDelete"
          class="btn-delete"
          @click="handleDelete(habit.id, habit.name)"
          title="Hapus aktivitas ini"
        >
          Hapus
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.badge {
  background: var(--color-bg);
  padding: 4px 10px;
  border-radius: 999px;
  font-size: 13px;
  color: var(--color-text-secondary);
}

.habit-list {
  max-height: 400px;
  overflow-y: auto;
}

.habit-item {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 14px 0;
  border-bottom: 1px solid var(--color-border);
  gap: 12px;
}

.habit-item:last-child {
  border-bottom: none;
}

.habit-info {
  flex: 1;
  min-width: 0;
}

.habit-name {
  font-weight: 500;
  color: var(--color-text);
  margin-bottom: 6px;
}

.habit-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  font-size: 13px;
  color: var(--color-text-secondary);
  align-items: center;
}

.habit-date,
.habit-time {
  background: var(--color-bg);
  padding: 2px 8px;
  border-radius: 4px;
}

.habit-notes {
  margin-top: 8px;
  font-size: 13px;
  color: var(--color-text-secondary);
  font-style: italic;
  background: var(--color-bg);
  padding: 8px 12px;
  border-radius: 6px;
}

.btn-delete {
  background: #fee2e2;
  color: #dc2626;
  border: none;
  padding: 6px 12px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
}

.btn-delete:hover {
  background: #dc2626;
  color: white;
}
</style>