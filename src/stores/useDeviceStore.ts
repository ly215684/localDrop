import { create } from 'zustand';
import type { Device, DeviceUpdate } from '@/types';

interface DeviceStore {
  devices: Device[];
  myDeviceId: string;
  myDeviceName: string;
  myDeviceType: 'windows' | 'macos' | 'android' | 'ios';
  
  setMyDevice: (id: string, name: string, type: 'windows' | 'macos' | 'android' | 'ios') => void;
  addOrUpdateDevice: (device: DeviceUpdate) => void;
  removeDevice: (deviceId: string) => void;
  clearDevices: () => void;
  updateDeviceName: (deviceId: string, name: string) => void;
}

export const useDeviceStore = create<DeviceStore>((set) => ({
  devices: [],
  myDeviceId: '',
  myDeviceName: '',
  myDeviceType: 'windows',
  
  setMyDevice: (id, name, type) => set({ myDeviceId: id, myDeviceName: name, myDeviceType: type }),
  
  addOrUpdateDevice: (update) => set((state) => {
    const index = state.devices.findIndex(d => d.device_id === update.device_id);
    if (index >= 0) {
      const devices = [...state.devices];
      devices[index] = {
        ...devices[index],
        ...update,
        last_seen: Date.now(),
      };
      return { devices };
    }
    return {
      devices: [...state.devices, { ...update, last_seen: Date.now() }],
    };
  }),
  
  removeDevice: (deviceId) => set((state) => ({
    devices: state.devices.filter(d => d.device_id !== deviceId),
  })),
  
  clearDevices: () => set({ devices: [] }),
  
  updateDeviceName: (deviceId, name) => set((state) => ({
    devices: state.devices.map(d => 
      d.device_id === deviceId ? { ...d, device_name: name } : d
    ),
  })),
}));
