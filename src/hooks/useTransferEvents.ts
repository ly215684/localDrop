import { useEffect } from 'react';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { TransferProgress, SessionProgress, TransferStatus, DeviceUpdate } from '@/types';
import { useTransferStore } from '@/stores/useTransferStore';
import { useDeviceStore } from '@/stores/useDeviceStore';

export const useTransferEvents = () => {
  const { 
    updateTransferProgress, 
    updateSessionProgress, 
    updateTransferStatus 
  } = useTransferStore();
  
  const { addOrUpdateDevice } = useDeviceStore();

  useEffect(() => {
    let unlisteners: UnlistenFn[] = [];

    const setupListeners = async () => {
      unlisteners = [
        await listen<TransferProgress>('transfer_progress', (event) => {
          updateTransferProgress(event.payload);
        }),
        await listen<SessionProgress>('session_progress', (event) => {
          updateSessionProgress(event.payload);
        }),
        await listen<TransferStatus>('transfer_status', (event) => {
          updateTransferStatus(event.payload);
        }),
        await listen<DeviceUpdate>('device_update', (event) => {
          addOrUpdateDevice(event.payload);
        }),
      ];
    };

    setupListeners();

    return () => {
      unlisteners.forEach((unlisten) => unlisten());
    };
  }, [updateTransferProgress, updateSessionProgress, updateTransferStatus, addOrUpdateDevice]);
};
