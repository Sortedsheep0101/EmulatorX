<template>
  <div class="emulators-section">
    <div class="filters">
      <div class="search-box">
        <input 
          type="text" 
          v-model="searchQuery"
          placeholder="Search emulators..."
          class="search-input"
        >
      </div>
      <div class="platform-filter">
        <select v-model="platformFilter">
          <option v-for="platform in platforms" 
                  :key="platform.value" 
                  :value="platform.value">
            {{ platform.label }}
          </option>
        </select>
      </div>
    </div>

    <h2>Available Emulators</h2>
    
    <div v-if="filteredAvailableEmulators.length === 0" class="empty-state">
      <p>No emulators match your current filters.</p>
    </div>
    
    <div v-else class="emulators-grid">
      <TransitionGroup
        name="emulator-list"
        tag="div"
        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
      >
        <EmulatorCard
          v-for="emulator in filteredAvailableEmulators"
          :key="emulator.name"
          :emulator="emulator"
          :errors="errors"
          class="emulator-card"
        >
          <template #actions>
            <button 
              @click="installEmulator(emulator)"
              class="primary"
              :disabled="isLoading[emulator.name]"
            >
              <span v-if="isLoading[emulator.name]" class="loader"></span>
              <span v-else>Install</span>
            </button>
          </template>
        </EmulatorCard>
      </TransitionGroup>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEmulators } from '~/composables/useEmulators';
import EmulatorCard from '~/components/EmulatorCard.vue';

const {
  searchQuery,
  platformFilter,
  platforms,
  filteredAvailableEmulators,
  isLoading,
  errors,
  installEmulator,
} = useEmulators();
</script>

<style scoped>
.emulators-section {
  padding: 2rem;
}

.filters {
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
}

.search-box {
  flex: 1;
}

.search-box input,
.platform-filter select {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 1rem;
  background: white;
  color: #1e293b;
}

.platform-filter {
  width: 200px;
}

h2 {
  font-size: 1.8rem;
  color: #34495e;
  margin-bottom: 1.5rem;
  border-bottom: 2px solid #eee;
  padding-bottom: 0.5rem;
}

.empty-state {
  text-align: center;
  padding: 3rem;
  background: white;
  border-radius: 16px;
  border: 1px solid rgba(52, 152, 219, 0.1);
  color: #64748b;
  font-size: 1.1rem;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
}

.emulators-grid {
  position: relative;
  min-height: 200px;
}

.emulator-card {
  background: white;
  border-radius: 16px;
  padding: 1.5rem;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
  transition: all 0.2s;
  border: 1px solid rgba(52, 152, 219, 0.1);
  position: relative;
  overflow: hidden;
}

.emulator-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: linear-gradient(45deg, #3498db, #2563eb);
  opacity: 0;
  transition: opacity 0.2s;
}

.emulator-card:hover::before {
  opacity: 1;
}

.emulator-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
}

.emulator-list-move,
.emulator-list-enter-active,
.emulator-list-leave-active {
  transition: all 0.5s ease;
}

.emulator-list-enter-from,
.emulator-list-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

.emulator-list-leave-active {
  position: absolute;
}

button.primary {
  background: linear-gradient(90deg, #3498db, #9b59b6, #3498db);
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

button.primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(155, 89, 182, 0.2);
}

button.primary:active {
  transform: translateY(0);
}

button:disabled {
  opacity: 0.7;
  cursor: not-allowed;
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

@keyframes moveGradient {
  0% {
    background-position: 0% 50%;
  }
  100% {
    background-position: 200% 50%;
  }
}
</style> 