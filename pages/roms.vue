<template>
  <div class="roms-section">
    <div class="header-actions">
      <h2>ROMs Library</h2>
    </div>

    <div class="upload-section">
      <div class="relative">
        <input
          type="file"
          id="rom-upload"
          @change="handleFileUpload"
          :accept="acceptedExtensions"
          multiple
          class="sr-only"
          ref="fileInput"
        >
        <button 
          @click="$refs.fileInput.click()"
          class="upload-button"
          type="button"
        >
          <svg 
            xmlns="http://www.w3.org/2000/svg" 
            class="upload-icon" 
            fill="none" 
            viewBox="0 0 24 24" 
            stroke="currentColor"
          >
            <path 
              stroke-linecap="round" 
              stroke-linejoin="round" 
              stroke-width="2" 
              d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
            />
          </svg>
          Import ROMs
        </button>
      </div>
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
      <p>No ROMs found. Import some ROMs to get started!</p>
    </div>

    <div v-else class="roms-grid">
      <div v-for="rom in filteredRoms" :key="rom.path" class="rom-card">
        <div class="rom-card-header">
          <h3>{{ rom.name }}</h3>
          <button 
            @click="() => openInExplorer(rom)"
            class="explorer-button"
            title="View in Explorer"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="folder-icon">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
            </svg>
          </button>
        </div>
        <div class="rom-info">
          <span class="size-badge">{{ formatSize(rom.size) }}</span>
          <span class="date-badge">{{ formatDate(rom.last_modified * 1000) }}</span>
        </div>
        <div class="button-group">
          <button 
            @click="() => playRom(rom)"
            class="play"
            title="Play ROM"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="play-icon">
              <polygon points="5 3 19 12 5 21 5 3"></polygon>
            </svg>
            Play
          </button>
          <button 
            @click="handleDelete(rom)"
            class="delete-button"
            :disabled="state.loading"
          >
            <svg 
              xmlns="http://www.w3.org/2000/svg" 
              class="delete-icon" 
              fill="none" 
              viewBox="0 0 24 24" 
              stroke="currentColor"
            >
              <path 
                stroke-linecap="round" 
                stroke-linejoin="round" 
                stroke-width="2" 
                d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" 
              />
            </svg>
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { formatSize, formatDate } from '~/composables/useRomApi';

interface RomMetadata {
  name: string;
  size: number;
  last_modified: number;
  path: string;
}

const acceptedExtensions = [
  '.iso', '.bin', '.gba', '.gbc', '.gb', '.nes', 
  '.smc', '.sfc', '.z64', '.n64', '.v64', '.cue', 
  '.chd', '.wbfs', '.gcm', '.dol', '.elf', '.pbp', 
  '.cso', '.xex',
  '.gdi', '.cdi', '.chd',
  '.xbe', '.iso',
  '.zip', '.7z', '.chd',
].join(',');

const searchQuery = ref('');
const state = ref({
  roms: [] as RomMetadata[],
  loading: false,
  error: null as string | null
});

const filteredRoms = computed(() => {
  return state.value.roms.filter(rom => 
    rom.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

const romToDelete = ref<RomMetadata | null>(null);

async function loadRoms() {
  state.value.loading = true;
  state.value.error = null;
  try {
    state.value.roms = await invoke('list_roms');
  } catch (error) {
    state.value.error = error instanceof Error ? error.message : 'Failed to load ROMs';
  } finally {
    state.value.loading = false;
  }
}

async function handleFileUpload(event: Event) {
  const target = event.target as HTMLInputElement;
  if (!target.files?.length) return;

  state.value.loading = true;
  state.value.error = null;

  try {
    for (const file of target.files) {
      // Create a temporary path for the file
      const tempPath = await invoke('create_temp_file', { 
        filename: file.name 
      }) as string;

      // Read the file and write it to the temp location
      const arrayBuffer = await file.arrayBuffer();
      await invoke('write_temp_file', { 
        path: tempPath, 
        data: Array.from(new Uint8Array(arrayBuffer))
      });

      // Import the ROM from the temp location
      await invoke('import_rom', { 
        sourcePath: tempPath 
      });

      // Clean up the temp file
      await invoke('delete_temp_file', { 
        path: tempPath 
      });
    }
    await loadRoms();
  } catch (error) {
    state.value.error = error instanceof Error ? error.message : 'Failed to import ROM';
  } finally {
    state.value.loading = false;
  }
}

const playingRom = ref<string | null>(null);

function getEmulatorForRom(filename: string): string {
  const ext = filename.split('.').pop()?.toLowerCase();
  switch (ext) {
    case 'iso':
    case 'gcm':
    case 'wbfs':
    case 'rvz':
      return 'Dolphin';
    case 'z64':
    case 'n64':
    case 'v64':
      return 'Mupen64Plus';
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

async function handleDelete(rom: RomMetadata) {
  if (window.confirm(`Are you sure you want to delete "${rom.name}"?`)) {
    state.value.loading = true;
    state.value.error = null;

    try {
      await invoke('delete_rom', { 
        filename: rom.name 
      });
      await loadRoms();
    } catch (error) {
      state.value.error = error instanceof Error ? error.message : 'Failed to delete ROM';
    } finally {
      state.value.loading = false;
    }
  }
}

onMounted(() => {
  loadRoms();
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

.upload-section {
  margin-bottom: 2rem;
}

.upload-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background: linear-gradient(90deg, #3498db, #2563eb);
  color: white;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  border: none;
  cursor: pointer;
  transition: all 0.2s;
  font-weight: 500;
}

.upload-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 6px rgba(37, 99, 235, 0.2);
}

.upload-icon {
  width: 20px;
  height: 20px;
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

.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

.delete-button {
  background: linear-gradient(90deg, #ef4444, #dc2626, #ef4444);
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

.delete-button:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(239, 68, 68, 0.2);
}

.delete-button:disabled {
  opacity: 0.7;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.delete-icon {
  width: 16px;
  height: 16px;
}

.cancel-button {
  background: #6b7280;
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  border: none;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.cancel-button:hover {
  background: #4b5563;
  transform: translateY(-1px);
}

.delete-confirm-button {
  background: linear-gradient(90deg, #ef4444, #dc2626, #ef4444);
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

.delete-confirm-button:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(239, 68, 68, 0.2);
}
</style> 
