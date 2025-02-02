<template>
  <div class="emulators-section">
    <div class="filters">
      <div class="search-box">
        <input 
          type="text" 
          v-model="searchQuery"
          placeholder="Search emulators..."
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

    <h2>Installed Emulators</h2>
    
    <div v-if="filteredInstalledEmulators.length === 0" class="empty-state">
      <p>No emulators installed yet.</p>
    </div>
    
    <div v-else class="emulators-grid">
      <TransitionGroup
        name="emulator-list"
        tag="div"
        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
      >
        <EmulatorCard
          v-for="emulator in filteredInstalledEmulators"
          :key="emulator.name"
          :emulator="emulator"
          :errors="errors"
        >
          <template #actions>
            <button 
              @click="runEmulator(emulator)" 
              class="primary mr-2"
              :disabled="isLoading[emulator.name]"
            >
              <span v-if="isLoading[emulator.name]" class="loader"></span>
              <span v-else>Run</span>
            </button>
            <button 
              @click="uninstallEmulator(emulator)" 
              class="danger"
              :disabled="isLoading[emulator.name]"
            >
              Uninstall
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
  filteredInstalledEmulators,
  isLoading,
  errors,
  runEmulator,
  uninstallEmulator,
} = useEmulators();
</script>

<style scoped>
.emulators-section {
  padding: 2rem 0;
}

.filters {
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
}

.search-box {
  flex: 1;
}

.search-box input {
  width: 100%;
  padding: 0.75rem 1rem;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 0.95rem;
  background: white;
  color: #1e293b;
  transition: all 0.2s;
}

.platform-filter select {
  padding: 0.75rem 1rem;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 0.95rem;
  background: white;
  color: #1e293b;
  min-width: 180px;
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

button.danger {
  background: linear-gradient(90deg, #e74c3c, #c0392b, #e74c3c);
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
</style> 