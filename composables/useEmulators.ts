import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface Emulator {
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
    version: "2412",
    lastUsed: "2024-03-15",
    size: "45MB",
    description: "GameCube/Wii Emulator",
    platforms: ["nintendo"]
  },
  { 
    name: "RPCS3",
    installed: false,
    version: "0.0.32",
    lastUsed: "2024-03-10",
    size: "120MB",
    description: "PlayStation 3 Emulator",
    platforms: ["playstation"]
  },
  { 
    name: "Xenia",
    installed: false,
    version: "1.0.2817",
    description: "Xbox 360 Emulator",
    size: "80MB",
    platforms: ["xbox"]
  },
  { 
    name: "PCSX2",
    installed: false,
    version: "2.2.0",
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
    version: "0.8.15",
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

export const platforms = [
  { value: 'all', label: 'All Platforms' },
  { value: 'nintendo', label: 'Nintendo' },
  { value: 'playstation', label: 'PlayStation' },
  { value: 'xbox', label: 'Xbox' },
  { value: 'sega', label: 'Sega' },
] as const;

export function useEmulators() {
  const installedEmulators = ref<Emulator[]>([]);
  const availableEmulators = ref<Emulator[]>([]);
  const isLoading = ref<{ [key: string]: boolean }>({});
  const errors = ref<{ [key: string]: string }>({});
  const searchQuery = ref('');
  const platformFilter = ref('all');

  const filteredInstalledEmulators = computed(() => {
    return installedEmulators.value.filter(emulator => {
      const matchesSearch = emulator.name.toLowerCase().includes(searchQuery.value.toLowerCase());
      const matchesPlatform = platformFilter.value === 'all' || emulator.platforms.includes(platformFilter.value);
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

  async function initializeEmulators() {
    try {
      // Check which emulators are actually installed
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
  }

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

  async function runEmulator(emulator: Emulator) {
    isLoading.value[emulator.name] = true;
    errors.value[emulator.name] = '';
    
    try {
      await invoke('run_emulator', { emulator: emulator.name });
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

  onMounted(() => {
    initializeEmulators();
  });

  return {
    installedEmulators,
    availableEmulators,
    isLoading,
    errors,
    searchQuery,
    platformFilter,
    platforms,
    filteredInstalledEmulators,
    filteredAvailableEmulators,
    runEmulator,
    uninstallEmulator,
    installEmulator,
  };
} 