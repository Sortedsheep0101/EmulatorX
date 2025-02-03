import { ref, reactive } from 'vue';

export interface RomMetadata {
  name: string;
  size: number;
  path: string;
  lastModified?: number;
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
      
      const data = await response.json();
      console.log('Response data:', data);
      
      return {
        success: true,
        data: data.map((rom: RomMetadata) => ({
          ...rom,
          lastModified: rom.lastModified || Date.now()
        }))
      };
    } catch (error) {
      console.error('API Error:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error'
      };
    }
  }

  async downloadRom(filename: string): Promise<Blob> {
    try {
      const response = await fetch(`${this.baseUrl}/api/roms/download/${encodeURIComponent(filename)}`);
      
      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.error || 'Failed to download ROM');
      }
      
      return await response.blob();
    } catch (error) {
      throw error instanceof Error ? error : new Error('Failed to download ROM');
    }
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
        state.roms = response.data;
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
      const blob = await api.downloadRom(rom.name);
      
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = rom.name;
      document.body.appendChild(a);
      a.click();
      window.URL.revokeObjectURL(url);
      document.body.removeChild(a);
    } catch (error) {
      state.error = error instanceof Error ? error.message : 'Failed to download ROM';
    } finally {
      state.loading = false;
    }
  }

  return {
    state,
    loadRoms,
    downloadRom
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