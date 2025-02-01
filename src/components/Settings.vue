<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";

const installPath = ref("");

interface Settings {
  auto_start: boolean;
  check_updates: boolean;
  close_to_tray: boolean;
}

const settings = ref<Settings>({
  auto_start: false,
  check_updates: true,
  close_to_tray: false,
});

// Load settings and installation path when component mounts
onMounted(async () => {
  try {
    const savedSettings = await invoke<typeof settings.value>("load_settings");
    settings.value = savedSettings;
    
    const path = await invoke<string>("get_install_path");
    installPath.value = path;
  } catch (error) {
    console.error("Failed to load settings:", error);
  }
});

async function changeInstallPath() {
  // TODO: Implement directory selection dialog
  console.log("Changing install path...");
}

async function saveSettings() {
  try {
    await invoke("save_settings", { settings: settings.value });
    console.log("Settings saved successfully");
  } catch (error) {
    console.error("Failed to save settings:", error);
  }
}

async function updateAutoStart(value: boolean) {
  try {
    await invoke("toggle_auto_launch", { enable: value });
    settings.value.auto_start = value;
  } catch (error) {
    console.error("Failed to update auto-launch setting:", error);
    // Revert the checkbox if it failed
    settings.value.auto_start = !value;
  }
}
</script>

<template>
  <div class="settings-container">
    <h2>Settings</h2>
    
    <div class="settings-section">
      <h3>Installation Directory</h3>
      <div class="setting-item">
        <div class="path-display">
          <input type="text" :value="installPath" readonly />
          <button @click="changeInstallPath" class="primary">
            Change Path
          </button>
        </div>
      </div>
    </div>

    <div class="settings-section">
      <h3>Application</h3>
      <div class="setting-item checkbox-item">
        <label>
          <input type="checkbox" v-model="settings.auto_start" @change="updateAutoStart(settings.auto_start)" />
          <span class="checkbox-label">
            <strong>Start with Windows</strong>
            <span class="description">Launch EmulatorX when your system starts</span>
          </span>
        </label>
      </div>
      <div class="setting-item checkbox-item">
        <label>
          <input type="checkbox" v-model="settings.check_updates" />
          <span class="checkbox-label">
            <strong>Check for updates automatically</strong>
            <span class="description">Keep EmulatorX up to date with the latest features</span>
          </span>
        </label>
      </div>
      <div class="setting-item checkbox-item">
        <label>
          <input type="checkbox" v-model="settings.close_to_tray" />
          <span class="checkbox-label">
            <strong>Minimize to tray when closed</strong>
            <span class="description">Keep EmulatorX running in the background</span>
          </span>
        </label>
      </div>
    </div>

    <div class="settings-actions">
      <button @click="saveSettings" class="primary">
        <span class="icon">💾</span>
        Save Settings
      </button>
    </div>
  </div>
</template>

<style scoped>
.settings-container {
  padding: 2rem;
  max-width: 800px;
  margin: 0 auto;
  color: #2c3e50;
}

.settings-section {
  background: white;
  border-radius: 12px;
  padding: 1.5rem;
  margin-bottom: 1.5rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  border: 1px solid #edf2f7;
}

h2 {
  font-size: 2rem;
  color: #2c3e50;
  margin-bottom: 2rem;
  font-weight: 600;
}

h3 {
  font-size: 1.4rem;
  color: #2d3748;
  margin-bottom: 1.25rem;
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.setting-item {
  margin-bottom: 1.25rem;
  display: flex;
  align-items: flex-start;
  gap: 1rem;
}

.checkbox-item {
  padding: 0.75rem;
  border-radius: 8px;
  transition: background-color 0.2s;
}

.checkbox-item:hover {
  background-color: #f8fafc;
}

.path-display {
  display: flex;
  gap: 0.5rem;
  width: 100%;
}

input[type="text"] {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-family: monospace;
  background: #f8fafc;
  color: #4a5568;
  font-size: 0.9rem;
}

input[type="checkbox"] {
  margin-right: 1rem;
  width: 1.2rem;
  height: 1.2rem;
  border-radius: 4px;
  border: 2px solid #cbd5e0;
  cursor: pointer;
}

.checkbox-label {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.checkbox-label strong {
  font-size: 1rem;
  color: #2d3748;
}

.description {
  font-size: 0.9rem;
  color: #718096;
}

label {
  display: flex;
  align-items: center;
  color: #2d3748;
  width: 100%;
  cursor: pointer;
}

.settings-actions {
  margin-top: 2.5rem;
  display: flex;
  justify-content: flex-end;
  padding: 0 0.5rem;
}

button {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 8px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.95rem;
}

.icon {
  font-size: 1.1rem;
}

button.primary {
  background-color: #3498db;
  color: white;
}

button.primary:hover {
  background-color: #2980b9;
  transform: translateY(-1px);
}

button.secondary {
  background-color: #f1f5f9;
  color: #475569;
}

button.secondary:hover {
  background-color: #e2e8f0;
  transform: translateY(-1px);
}

button:active {
  transform: translateY(0);
}
</style> 