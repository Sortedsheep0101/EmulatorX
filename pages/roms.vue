<template>
  <div class="roms-section">
    <div class="header-actions">
      <h2>Available ROMs</h2>
    </div>

    <div class="filters">
      <div class="search-box">
        <input 
          type="text" 
          v-model="searchQuery"
          placeholder="Search ROMs..."
          class="search-input"
        >
      </div>
    </div>

    <div v-if="state.loading" class="loading-state">
      <div class="loader"></div>
      <p>Loading ROMs...</p>
    </div>

    <div v-else-if="state.error" class="error-state">
      <p>{{ state.error }}</p>
      <button @click="loadRoms" class="primary">
        Retry
      </button>
    </div>

    <div v-else-if="filteredRoms.length === 0" class="empty-state">
      <p>No ROMs found matching your criteria.</p>
    </div>

    <div v-else class="roms-grid">
      <div v-for="rom in filteredRoms" :key="rom.path" class="rom-card">
        <h3>{{ rom.name }}</h3>
        <div class="rom-info">
          <span class="size-badge">{{ formatSize(rom.size) }}</span>
          <span class="date-badge">{{ formatDate(rom.lastModified) }}</span>
        </div>
        <div v-if="state.error" class="error-message">
          {{ state.error }}
        </div>
        <div class="button-group">
          <button 
            @click="() => downloadRom(rom)"
            class="primary" 
            :class="{ 'downloaded': rom.downloaded }"
            :disabled="state.loading"
          >
            <span v-if="state.loading" class="loader"></span>
            <span v-else>{{ rom.downloaded ? 'Downloaded' : 'Download' }}</span>
          </button>
          <button 
            v-if="rom.downloaded"
            @click="() => deleteRom(rom)"
            class="delete"
            :disabled="state.loading"
          >
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRomApi, formatSize, formatDate, type RomMetadata } from '~/composables/useRomApi';

const searchQuery = ref('');
const { state, loadRoms, downloadRom, deleteRom } = useRomApi('http://localhost:1248');

const filteredRoms = computed(() => {
  return state.roms.filter(rom => 
    rom.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

onMounted(() => {
  loadRoms().catch(err => {
    console.error('Failed to load ROMs:', err);
  });
});
</script>

<style scoped>
.roms-section {
  padding: 2rem;
}

.header-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.filters {
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
}

.search-box {
  flex: 1;
}

.roms-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
}

.rom-card {
  background: white;
  border-radius: 16px;
  padding: 1.5rem;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
  border: 1px solid rgba(52, 152, 219, 0.1);
  transition: all 0.2s;
}

.rom-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
}

.rom-info {
  display: flex;
  gap: 0.5rem;
  margin: 1rem 0;
}

.size-badge {
  background: #f1f5f9;
  color: #475569;
  padding: 0.3rem 0.8rem;
  border-radius: 6px;
  font-size: 0.9rem;
}

.date-badge {
  background: #f1f5f9;
  color: #475569;
  padding: 0.3rem 0.8rem;
  border-radius: 6px;
  font-size: 0.9rem;
}

.loading-state, .error-state, .empty-state {
  text-align: center;
  padding: 3rem;
  color: #64748b;
}

.loader {
  display: inline-block;
  width: 1rem;
  height: 1rem;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  border-top-color: white;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

button.primary {
  background: linear-gradient(90deg, #3498db, #2563eb, #3498db);
  background-size: 200% 100%;
  animation: moveGradient 2s linear infinite;
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  border: none;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

button:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

@keyframes moveGradient {
  0% {
    background-position: 0% 50%;
  }
  100% {
    background-position: 200% 50%;
  }
}

.button-group {
  display: flex;
  gap: 0.5rem;
  margin-top: 1rem;
}

button.primary.downloaded {
  background: linear-gradient(90deg, #22c55e, #16a34a, #22c55e);
  background-size: 200% 100%;
  animation: moveGradient 2s linear infinite;
}

button.delete {
  background: #ef4444;
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  border: none;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

button.delete:hover {
  background: #dc2626;
  transform: translateY(-1px);
}

button.delete:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

.error-message {
  color: #dc2626;
  font-size: 0.9rem;
  margin: 0.5rem 0;
  word-break: break-word;
}
</style> 
