<script setup>
import { ref, computed } from "vue";
import { useHabitStore } from "../stores/habitStore";

const store = useHabitStore();

const name = ref("");
const category = ref("health");
const activityDate = ref(new Date().toISOString().split("T")[0]); // Format: YYYY-MM-DD
const activityTime = ref(
  new Date().toLocaleTimeString("en-GB", { hour: "2-digit", minute: "2-digit" })
); // Format: HH:mm
const notes = ref("");
const submitting = ref(false);

const categoryLabels = {
  health: "Kesehatan",
  productivity: "Produktivitas",
  hygiene: "Kebersihan",
  exercise: "Olahraga",
  sleep: "Tidur",
  other: "Lainnya",
};

const isValid = computed(() => {
  return name.value.trim().length > 0 && activityDate.value && activityTime.value;
});

const handleSubmit = async () => {
  if (!isValid.value || submitting.value) return;

  submitting.value = true;
  try {
    // Gabungkan tanggal dan jam menjadi ISO string
    const dateTimeString = `${activityDate.value}T${activityTime.value}:00`;
    const timestamp = new Date(dateTimeString).toISOString();

    await store.addHabit({
      name: name.value.trim(),
      category: category.value,
      timestamp: timestamp,
      compliance_level: null,
      notes: notes.value.trim() || null,
    });

    // Reset form
    name.value = "";
    category.value = "health";
    activityDate.value = new Date().toISOString().split("T")[0];
    activityTime.value = new Date().toLocaleTimeString("en-GB", {
      hour: "2-digit",
      minute: "2-digit",
    });
    notes.value = "";
  } catch (error) {
    console.error("Error adding habit:", error);
  } finally {
    submitting.value = false;
  }
};
</script>

<template>
  <div class="card">
    <div class="card-header">
      <h2>Tambah Aktivitas</h2>
    </div>

    <form @submit.prevent="handleSubmit">
      <div class="form-group">
        <label for="name">Nama Aktivitas</label>
        <input
          id="name"
          v-model="name"
          type="text"
          placeholder="Contoh: Makan pagi, Olahraga, dll"
          required
        />
      </div>

      <div class="form-group">
        <label for="category">Kategori</label>
        <select id="category" v-model="category">
          <option
            v-for="cat in store.state.categories"
            :key="cat"
            :value="cat"
          >
            {{ categoryLabels[cat] || cat }}
          </option>
        </select>
      </div>

      <div class="form-row">
        <div class="form-group">
          <label for="date">Tanggal</label>
          <input
            id="date"
            v-model="activityDate"
            type="date"
            required
          />
        </div>

        <div class="form-group">
          <label for="time">Jam</label>
          <input
            id="time"
            v-model="activityTime"
            type="time"
            required
          />
        </div>
      </div>

      <div class="form-group">
        <label for="notes">Catatan (opsional)</label>
        <textarea
          id="notes"
          v-model="notes"
          placeholder="Tambahkan catatan..."
          rows="2"
        ></textarea>
      </div>

      <button
        type="submit"
        class="btn btn-primary btn-block"
        :disabled="!isValid || submitting"
      >
        {{ submitting ? "Menyimpan..." : "Simpan Aktivitas" }}
      </button>
    </form>
  </div>
</template>

<style scoped>
.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}
</style>