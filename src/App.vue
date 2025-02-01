<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Settings from "./components/Settings.vue";

interface Emulator {
  name: string;
  installed: boolean;
  version?: string;
  lastUsed?: string;
  size?: string;
  description?: string;
  platforms: string[];
}

// Define all emulators with their metadata
const allEmulators: Emulator[] = [
  { 
    name: "Dolphin",
    installed: false,
    version: "5.0.2",
    lastUsed: "2024-03-15",
    size: "45MB",
    description: "GameCube/Wii Emulator",
    platforms: ["nintendo"]
  },
  { 
    name: "RPCS3",
    installed: false,
    version: "0.0.30",
    lastUsed: "2024-03-10",
    size: "120MB",
    description: "PlayStation 3 Emulator",
    platforms: ["playstation"]
  },
  { 
    name: "Xenia",
    installed: false,
    version: "1.0.5",
    description: "Xbox 360 Emulator",
    size: "80MB",
    platforms: ["xbox"]
  },
  { 
    name: "PCSX2",
    installed: false,
    version: "1.7.0",
    description: "PlayStation 2 Emulator",
    size: "65MB",
    platforms: ["playstation"]
  },
  {
    name: "DuckStation",
    installed: false,
    version: "latest",
    description: "PlayStation 1 Emulator",
    size: "30MB",
    platforms: ["playstation"]
  },
  {
    name: "mGBA",
    installed: false,
    version: "0.10.4",
    description: "Game Boy Advance Emulator",
    size: "15MB",
    platforms: ["nintendo"]
  },
  {
    name: "Xemu",
    installed: false,
    version: "latest",
    description: "Original Xbox Emulator",
    size: "40MB",
    platforms: ["xbox"]
  },
  {
    name: "PPSSPP",
    installed: false,
    version: "1.18.1",
    description: "PSP Emulator",
    size: "35MB",
    platforms: ["playstation"]
  },
  {
    name: "Flycast",
    installed: false,
    version: "2.4",
    description: "Dreamcast Emulator",
    size: "28MB",
    platforms: ["sega"]
  },
  {
    name: "ZSNES",
    installed: false,
    version: "1.51",
    description: "Super Nintendo Emulator",
    size: "12MB",
    platforms: ["nintendo"]
  },
  {
    name: "Mesen",
    installed: false,
    version: "2.0.0",
    description: "NES/SNES/GB/GBC Emulator",
    size: "25MB",
    platforms: ["nintendo"]
  }
];

const installedEmulators = ref<Emulator[]>([]);
const availableEmulators = ref<Emulator[]>([]);

// Add currentTab state
const currentTab = ref('installed');

// Add after the currentTab ref
const searchQuery = ref('');
const platformFilter = ref('all');

// Add loading and error states
const isLoading = ref<{ [key: string]: boolean }>({});
const errors = ref<{ [key: string]: string }>({});

// Add platforms list
const platforms = [
  { value: 'all', label: 'All Platforms' },
  { value: 'nintendo', label: 'Nintendo' },
  { value: 'playstation', label: 'PlayStation' },
  { value: 'xbox', label: 'Xbox' },
  { value: 'sega', label: 'Sega' },
];

// Add after the platforms array
const filteredInstalledEmulators = computed(() => {
  return installedEmulators.value.filter(emulator => {
    const matchesSearch = emulator.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
                         emulator.description?.toLowerCase().includes(searchQuery.value.toLowerCase());
    const matchesPlatform = platformFilter.value === 'all' || 
                           emulator.platforms.includes(platformFilter.value);
    return matchesSearch && matchesPlatform;
  });
});

const filteredAvailableEmulators = computed(() => {
  return availableEmulators.value.filter(emulator => {
    const matchesSearch = emulator.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
                         emulator.description?.toLowerCase().includes(searchQuery.value.toLowerCase());
    const matchesPlatform = platformFilter.value === 'all' || 
                           emulator.platforms.includes(platformFilter.value);
    return matchesSearch && matchesPlatform;
  });
});

// Check which emulators are actually installed on mount
onMounted(async () => {
  try {
    const statuses = await invoke<{ name: string, installed: boolean }[]>("check_installations");
    
    // Update installation status for all emulators
    allEmulators.forEach(emulator => {
      const status = statuses.find(s => s.name === emulator.name);
      emulator.installed = status?.installed || false;
    });

    // Split emulators into installed and available lists
    installedEmulators.value = allEmulators.filter(e => e.installed);
    availableEmulators.value = allEmulators.filter(e => !e.installed);
  } catch (error) {
    console.error("Failed to check installations:", error);
    // If check fails, assume nothing is installed
    installedEmulators.value = [];
    availableEmulators.value = [...allEmulators];
  }
});

async function installEmulator(emulator: Emulator) {
  isLoading.value[emulator.name] = true;
  errors.value[emulator.name] = '';
  
  try {
    await invoke("download_emulator", { emulator: emulator.name });
    console.log(`Installing ${emulator.name}`);
    emulator.installed = true;
    installedEmulators.value.push({ ...emulator });
    availableEmulators.value = availableEmulators.value.filter(e => e.name !== emulator.name);
  } catch (error) {
    console.error(`Failed to install ${emulator.name}:`, error);
    errors.value[emulator.name] = `Installation failed: ${error}`;
  } finally {
    isLoading.value[emulator.name] = false;
  }
}

async function uninstallEmulator(emulator: Emulator) {
  isLoading.value[emulator.name] = true;
  errors.value[emulator.name] = '';
  
  try {
    await invoke("uninstall_emulator", { emulator: emulator.name });
    console.log(`Uninstalling ${emulator.name}`);
    emulator.installed = false;
    availableEmulators.value.push({ ...emulator });
    installedEmulators.value = installedEmulators.value.filter(e => e.name !== emulator.name);
  } catch (error) {
    console.error(`Failed to uninstall ${emulator.name}:`, error);
    errors.value[emulator.name] = `Uninstallation failed: ${error}`;
  } finally {
    isLoading.value[emulator.name] = false;
  }
}

async function runEmulator(emulator: Emulator) {
  isLoading.value[emulator.name] = true;
  errors.value[emulator.name] = '';
  
  try {
    await invoke("run_emulator", { emulator: emulator.name });
    console.log(`Running ${emulator.name}`);
    
    // Update last used time
    emulator.lastUsed = new Date().toISOString().split('T')[0];
  } catch (error) {
    console.error(`Failed to run ${emulator.name}:`, error);
    errors.value[emulator.name] = `Failed to launch: ${error}`;
  } finally {
    isLoading.value[emulator.name] = false;
  }
}

function formatDate(date: string | undefined): string {
  if (!date) return 'Never';
  return new Date(date).toLocaleDateString();
}
</script>

<template>
  <div class="app-container">
    <header>
      <h1>EmulatorX</h1>
    </header>

    <nav>
      <button 
        :class="{ active: currentTab === 'installed' }"
        @click="currentTab = 'installed'"
      >
        Installed
      </button>
      <button 
        :class="{ active: currentTab === 'available' }"
        @click="currentTab = 'available'"
      >
        Available
      </button>
      <button 
        :class="{ active: currentTab === 'settings' }"
        @click="currentTab = 'settings'"
      >
        Settings
      </button>
    </nav>

    <!-- Add after the nav section -->
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

    <!-- Show/hide content based on currentTab -->
    <div v-if="currentTab === 'installed'" class="emulators-section">
      <h2>Installed Emulators</h2>
      <div v-if="filteredInstalledEmulators.length === 0" class="empty-state">
        <p>No emulators installed yet.</p>
      </div>
      <div v-else class="emulators-grid">
        <div v-for="emulator in filteredInstalledEmulators" :key="emulator.name" class="emulator-card">
          <div class="emulator-header">
            <h3>{{ emulator.name }}</h3>
          </div>
          <div class="emulator-info">
            <p>Last used: {{ formatDate(emulator.lastUsed) }}</p>
            <p>Size: {{ emulator.size }}</p>
          </div>
          <div class="button-group">
            <button 
              @click="runEmulator(emulator)" 
              class="primary"
              :disabled="isLoading[emulator.name]"
            >
              <span v-if="isLoading[emulator.name]" class="loader"></span>
              <span v-else>Launch</span>
            </button>
            <button 
              @click="uninstallEmulator(emulator)" 
              class="danger"
              :disabled="isLoading[emulator.name]"
            >
              <span v-if="isLoading[emulator.name]" class="loader"></span>
              <span v-else>Uninstall</span>
            </button>
          </div>
          <div v-if="errors[emulator.name]" class="error-message">
            {{ errors[emulator.name] }}
          </div>
        </div>
      </div>
    </div>

    <div v-if="currentTab === 'available'" class="emulators-section">
      <h2>Available Emulators</h2>
      <div v-if="filteredAvailableEmulators.length === 0" class="empty-state">
        <p>No emulators match your current filters.</p>
      </div>
      <div v-else class="emulators-grid">
        <div v-for="emulator in filteredAvailableEmulators" :key="emulator.name" class="emulator-card">
          <div class="emulator-header">
            <h3>{{ emulator.name }}</h3>
            <div class="header-info">
              <span class="version">v{{ emulator.version }}</span>
            </div>
          </div>
          <div class="emulator-info">
            <p>{{ emulator.description }}</p>
            <p>Size: {{ emulator.size }}</p>
          </div>
          <div class="button-group">
            <button 
              @click="installEmulator(emulator)" 
              class="primary"
              :disabled="isLoading[emulator.name]"
            >
              <span v-if="isLoading[emulator.name]" class="loader"></span>
              <span v-else>Install</span>
            </button>
          </div>
          <div v-if="errors[emulator.name]" class="error-message">
            {{ errors[emulator.name] }}
          </div>
        </div>
      </div>
    </div>

    <Settings v-if="currentTab === 'settings'" />
  </div>
</template>

<style scoped>
.app-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
  font-family: system-ui, -apple-system, sans-serif;
  background: linear-gradient(to bottom, #f8fafc, #f1f5f9);
  min-height: 100vh;
}

header {
  text-align: center;
  margin-bottom: 3rem;
}

h1 {
  font-size: 2.8rem;
  background: linear-gradient(45deg, #3498db, #2563eb);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  margin-bottom: 1rem;
}

.emulators-section {
  margin-bottom: 3rem;
}

h2 {
  font-size: 1.8rem;
  color: #34495e;
  margin-bottom: 1.5rem;
  border-bottom: 2px solid #eee;
  padding-bottom: 0.5rem;
}

.emulators-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
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

.emulator-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.emulator-header h3 {
  margin: 0;
  font-size: 1.4rem;
  color: #1e293b;
  font-weight: 600;
}

.version {
  background: linear-gradient(45deg, #3498db, #2563eb);
  color: white;
  padding: 0.3rem 0.8rem;
  border-radius: 6px;
  font-size: 0.9rem;
  font-weight: 500;
}

.emulator-info {
  margin-bottom: 1.5rem;
}

.emulator-info p {
  margin: 0.5rem 0;
  color: #64748b;
  line-height: 1.6;
}

.button-group {
  display: flex;
  gap: 0.5rem;
}

button {
  padding: 0.6rem 1.2rem;
  border: none;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
  flex: 1;
}

button.primary {
  background: linear-gradient(45deg, #3498db, #2563eb);
  color: white;
  transition: all 0.2s;
}

button.primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(37, 99, 235, 0.2);
}

button.danger {
  background: linear-gradient(45deg, #ef4444, #dc2626);
  color: white;
}

button.danger:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(220, 38, 38, 0.2);
}

button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

nav {
  margin: 1.5rem auto;
  display: flex;
  justify-content: center;
  gap: 1rem;
  background: white;
  padding: 0.75rem;
  border-radius: 12px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
  max-width: fit-content;
  border: 1px solid rgba(52, 152, 219, 0.1);
}

nav button {
  padding: 0.75rem 2rem;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: #64748b;
  cursor: pointer;
  transition: all 0.2s ease;
  font-weight: 500;
  min-width: 120px;
  font-size: 0.95rem;
}

nav button.active {
  background: linear-gradient(45deg, #3498db, #2563eb);
  color: white;
  box-shadow: 0 4px 12px rgba(37, 99, 235, 0.2);
}

nav button:hover:not(.active) {
  background: #f1f5f9;
  color: #334155;
}

/* Remove all the icon-related styles */
nav button[data-tab="installed"]::before,
nav button[data-tab="available"]::before,
nav button[data-tab="settings"]::before {
  display: none;
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

/* Add after existing styles */
.filters {
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
  padding: 0 0.5rem;
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

.search-box input:focus {
  outline: none;
  border-color: #3498db;
  box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.1);
}

.platform-filter select {
  padding: 0.75rem 2rem 0.75rem 1rem;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 0.95rem;
  background: white;
  color: #1e293b;
  cursor: pointer;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%23475569' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 0.75rem center;
  min-width: 200px;
}

.platform-filter select:focus {
  outline: none;
  border-color: #3498db;
  box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.1);
}

/* Add new styles */
.loader {
  width: 16px;
  height: 16px;
  border: 2px solid #ffffff;
  border-bottom-color: transparent;
  border-radius: 50%;
  display: inline-block;
  animation: rotation 1s linear infinite;
}

.error-message {
  color: #ef4444;
  font-size: 0.875rem;
  margin-top: 0.5rem;
  padding: 0.5rem;
  background-color: #fee2e2;
  border-radius: 6px;
}

/* Remove the recently-played class */
.recently-played {
  margin-bottom: 2rem;
}

@keyframes rotation {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

/* Add to your existing styles */
.header-info {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}
</style>