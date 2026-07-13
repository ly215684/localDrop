import { invoke } from '@tauri-apps/api/core';
import type { Settings } from '@/types';

export const tauriApi = {
  startDiscovery: () => invoke<void>('start_discovery'),
  stopDiscovery: () => invoke<void>('stop_discovery'),
  
  sendFiles: (deviceId: string, filePaths: string[]) => 
    invoke<string>('send_files', { deviceId, filePaths }),
  
  acceptTransfer: (transferId: string) => 
    invoke<void>('accept_transfer', { transferId }),
  
  cancelTransfer: (transferId: string) => 
    invoke<void>('cancel_transfer', { transferId }),
  
  pauseTransfer: (transferId: string) => 
    invoke<void>('pause_transfer', { transferId }),
  
  resumeTransfer: (transferId: string) => 
    invoke<void>('resume_transfer', { transferId }),
  
  getSettings: () => invoke<Settings>('get_settings'),
  
  saveSettings: (settings: Settings) => 
    invoke<void>('save_settings', { settings }),
  
  renameDevice: (newName: string) => 
    invoke<void>('rename_device', { newName }),
  
  getMyDeviceInfo: () => 
    invoke<{ device_id: string; device_name: string; device_type: string }>(
      'get_my_device_info'
    ),
  
  openFile: (filePath: string) => invoke<void>('open_file', { filePath }),
  
  openFolder: (folderPath: string) => invoke<void>('open_folder', { folderPath }),
};
