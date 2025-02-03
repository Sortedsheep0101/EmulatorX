import { ref, reactive } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface RomMetadata {
  name: string;
  size: number;
  path: string;
  lastModified?: number;
  downloaded?: boolean;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  count?: number;
  error?: string;
  details?: string;
}

class RomApiClient {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  async listRoms(): Promise<ApiResponse<RomMetadata[]>> {
    try {
      console.log('Fetching from:', `${this.baseUrl}/api/roms`);
      const response = await fetch(`${this.baseUrl}/api/roms`, {
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json'
        }
      });
      console.log('Response status:', response.status);
      
      const responseData = await response.json();
      console.log('Response data:', responseData);
      
      if (!responseData.success || !Array.isArray(responseData.data)) {
        throw new Error('Invalid response format from server');
      }
      
      return {
        success: true,
        data: responseData.data.map((rom: RomMetadata) => ({
          ...rom,
          lastModified: rom.lastModified || Date.now()
        })),
        count: responseData.count
      };
    } catch (error) {
      console.error('API Error:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error'
      };
    }
  }

  async downloadRom(filename: string): Promise<string> {
    const downloadUrl = `${this.baseUrl}/api/roms/download/${encodeURIComponent(filename)}`;
    console.log('Download URL:', downloadUrl);
    return await invoke('download_rom', { 
      url: downloadUrl, 
      filename 
    });
  }

  async isRomDownloaded(filename: string): Promise<boolean> {
    return await invoke('is_rom_downloaded', { filename });
  }

  async deleteRom(filename: string): Promise<void> {
    return await invoke('delete_rom', { filename });
  }
}

export function useRomApi(baseUrl: string) {
  const api = new RomApiClient(baseUrl);
  
  const state = reactive({
    roms: [] as RomMetadata[],
    loading: false,
    error: null as string | null
  });

  async function loadRoms() {
    state.loading = true;
    state.error = null;
    
    try {
      const response = await api.listRoms();
      
      if (response.success && response.data) {
        // Check downloaded status for each ROM
        const romsWithStatus = await Promise.all(
          response.data.map(async (rom) => ({
            ...rom,
            downloaded: await api.isRomDownloaded(rom.name)
          }))
        );
        state.roms = romsWithStatus;
      } else {
        throw new Error(response.error || 'Failed to load ROMs');
      }
    } catch (error) {
      state.error = error instanceof Error ? error.message : 'Unknown error';
      throw error;
    } finally {
      state.loading = false;
    }
  }

  async function downloadRom(rom: RomMetadata) {
    try {
      state.loading = true;
      state.error = null;
      console.log(`Starting download for ROM: ${rom.name}`);
      
      const filepath = await api.downloadRom(rom.name);
      console.log('ROM downloaded to:', filepath);
      
      // Verify the download was successful
      const isDownloaded = await api.isRomDownloaded(rom.name);
      if (!isDownloaded) {
        throw new Error('ROM file not found after download');
      }
      
      // Update the downloaded status in the state
      const romIndex = state.roms.findIndex(r => r.name === rom.name);
      if (romIndex !== -1) {
        state.roms[romIndex].downloaded = true;
      }
      
      console.log(`Download completed successfully for ${rom.name}`);
    } catch (error) {
      console.error('Download error:', error);
      state.error = error instanceof Error ? error.message : 'Failed to download ROM';
      throw error;
    } finally {
      state.loading = false;
    }
  }

  async function deleteRom(rom: RomMetadata) {
    try {
      state.loading = true;
      await api.deleteRom(rom.name);
      
      // Update the downloaded status in the state
      const romIndex = state.roms.findIndex(r => r.name === rom.name);
      if (romIndex !== -1) {
        state.roms[romIndex].downloaded = false;
      }
    } catch (error) {
      state.error = error instanceof Error ? error.message : 'Failed to delete ROM';
    } finally {
      state.loading = false;
    }
  }

  return {
    state,
    loadRoms,
    downloadRom,
    deleteRom
  };
}

export function formatSize(bytes: number): string {
  const units = ['B', 'KB', 'MB', 'GB'];
  let size = bytes;
  let unitIndex = 0;
  
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex++;
  }
  
  return `${size.toFixed(2)} ${units[unitIndex]}`;
}

export function formatDate(timestamp: number | undefined): string {
  if (!timestamp) {
    return 'Unknown date';
  }
  return new Date(timestamp).toLocaleDateString();
} 