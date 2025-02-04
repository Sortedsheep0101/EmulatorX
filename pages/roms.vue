<template>
  <div class="roms-section">
    <div class="header-actions">
      <h2>Available ROMs</h2>
    </div>

    <div class="server-config">
      <input 
        type="text" 
        v-model="serverAddress"
        placeholder="Server address (e.g. http://localhost:1248)"
        class="server-input"
      >
      <button @click="handleUpdateServer" class="primary">
        Connect
      </button>
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
        <div class="rom-card-header">
          <h3>{{ rom.name }}</h3>
          <button 
            v-if="rom.downloaded"
            @click="() => openInExplorer(rom)"
            class="explorer-button"
            :disabled="state.loading"
            title="View in Explorer"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="folder-icon">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
            </svg>
          </button>
        </div>
        <div class="rom-info">
          <span class="size-badge">{{ formatSize(rom.size) }}</span>
          <span class="date-badge">{{ formatDate(rom.lastModified) }}</span>
        </div>
        <div v-if="state.error" class="error-message">
          {{ state.error }}
        </div>
        <div class="button-group">
          <button 
            v-if="rom.downloaded"
            @click="() => playRom(rom)"
            class="play"
            :disabled="state.loading"
            title="Play ROM"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="play-icon">
              <polygon points="5 3 19 12 5 21 5 3"></polygon>
            </svg>
            Play
          </button>
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
import { invoke } from '@tauri-apps/api/core';

const searchQuery = ref('');
const { state, serverAddress, loadRoms, updateServer, downloadRom, deleteRom } = useRomApi('http://localhost:1248');

const filteredRoms = computed(() => {
  return state.roms.filter(rom => 
    rom.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

const playingRom = ref<string | null>(null);

function getEmulatorForRom(filename: string): string {
  const ext = filename.split('.').pop()?.toLowerCase();
  switch (ext) {
    case 'iso':
    case 'gcm':
    case 'wbfs':
    case 'rvz':
      return 'Dolphin';
    case 'xex':
    case 'iso':
      return 'Xenia';
    case 'iso':
    case 'bin':
      return 'PCSX2';
    case 'pkg':
    case 'rap':
      return 'RPCS3';
    case 'iso':
    case 'bin':
    case 'img':
      return 'DuckStation';
    case 'gba':
    case 'gbc':
    case 'gb':
      return 'mGBA';
    case 'iso':
      return 'xemu';
    case 'iso':
    case 'cso':
    case 'pbp':
      return 'PPSSPP';
    case 'gdi':
    case 'cdi':
    case 'chd':
      return 'Flycast';
    case 'smc':
    case 'sfc':
      return 'ZSNES';
    case 'nes':
    case 'fds':
      return 'Mesen';
    default:
      throw new Error('Unsupported ROM format');
  }
}

async function handleUpdateServer() {
  await updateServer(serverAddress.value);
}

async function openInExplorer(rom: RomMetadata) {
  try {
    await invoke('open_rom_folder', { filename: rom.name });
  } catch (error) {
    console.error('Failed to open explorer:', error);
  }
}

async function playRom(rom: RomMetadata) {
  if (playingRom.value === rom.name) return;
  playingRom.value = rom.name;
  
  try {
    const emulator = getEmulatorForRom(rom.name);
    const installDir = await invoke('get_install_path');
    const romPath = `${installDir}/roms/${rom.name}`;
    
    await invoke('run_emulator_with_rom', {
      emulator,
      romPath: romPath
    });
  } catch (error) {
    console.error('Failed to launch ROM:', error);
    // You might want to show this error to the user
  } finally {
    playingRom.value = null;
  }
}

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

.server-config {
  display: flex;
  gap: 1rem;
  margin-bottom: 1.5rem;
  align-items: center;
}

.server-input {
  flex: 1;
  padding: 0.75rem 1rem;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 0.95rem;
  transition: border-color 0.2s;
}

.server-input:focus {
  outline: none;
  border-color: #3498db;
  box-shadow: 0 0 0 2px rgba(52, 152, 219, 0.2);
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

.rom-card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1rem;
}

.rom-card-header h3 {
  margin: 0;
  padding-right: 1rem;
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

button.secondary {
  background: linear-gradient(90deg, #64748b, #475569, #64748b);
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

button.secondary:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(71, 85, 105, 0.2);
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

.explorer-button {
  min-width: 32px;
  width: 32px;
  height: 32px;
  padding: 6px;
  border: none;
  border-radius: 6px;
  background: #3498db;
  color: white;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.explorer-button:hover {
  background: #2563eb;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(37, 99, 235, 0.2);
}

.explorer-button:disabled {
  opacity: 0.7;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.folder-icon {
  width: 16px;
  height: 16px;
}

button.play {
  background: linear-gradient(90deg, #10b981, #059669, #10b981);
  background-size: 200% 100%;
  animation: moveGradient 2s linear infinite;
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  border: none;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

button.play:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(16, 185, 129, 0.2);
}

button.play:disabled {
  opacity: 0.7;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.play-icon {
  width: 16px;
  height: 16px;
}
</style> 
