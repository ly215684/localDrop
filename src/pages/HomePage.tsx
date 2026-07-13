import { useEffect, useState } from 'react';
import { Send, RefreshCw, Wifi, WifiOff, Plus } from 'lucide-react';
import { DeviceCard } from '@/components/DeviceCard';
import { useDeviceStore } from '@/stores/useDeviceStore';
import { tauriApi } from '@/api/tauri';
import { open } from '@tauri-apps/plugin-dialog';
import type { Device } from '@/types';

interface HomePageProps {
  onNavigate: (page: string) => void;
}

export const HomePage = ({ onNavigate }: HomePageProps) => {
  const { devices, myDeviceName, setMyDevice } = useDeviceStore();
  const [isDiscovering, setIsDiscovering] = useState(false);
  const [selectedDevice, setSelectedDevice] = useState<Device | null>(null);
  const [showFilePicker, setShowFilePicker] = useState(false);

  useEffect(() => {
    const init = async () => {
      try {
        const info = await tauriApi.getMyDeviceInfo();
        setMyDevice(info.device_id, info.device_name, info.device_type as 'windows' | 'macos' | 'android' | 'ios');
      } catch (e) {
        console.error('Failed to get device info:', e);
      }
    };
    init();
  }, [setMyDevice]);

  useEffect(() => {
    const start = async () => {
      setIsDiscovering(true);
      await tauriApi.startDiscovery();
    };
    start();

    return () => {
      const stop = async () => {
        setIsDiscovering(false);
        await tauriApi.stopDiscovery();
      };
      stop();
    };
  }, []);

  const handleDeviceClick = (device: Device) => {
    if (device.online) {
      setSelectedDevice(device);
      setShowFilePicker(true);
    }
  };

  const handleFileSelect = async () => {
    if (!selectedDevice) return;
    
    try {
      const result = await open({
        multiple: true,
        directory: true,
      });

      if (result) {
        const filePaths = Array.isArray(result) ? result : [result];
        await tauriApi.sendFiles(selectedDevice.device_id, filePaths);
        onNavigate('/transfers');
      }
    } catch (e) {
      console.error('Failed to select files:', e);
    } finally {
      setShowFilePicker(false);
      setSelectedDevice(null);
    }
  };

  const handleRefresh = async () => {
    await tauriApi.stopDiscovery();
    await tauriApi.startDiscovery();
  };

  const onlineDevices = devices.filter(d => d.online);

  return (
    <div className="h-full flex flex-col">
      <div className="p-4 border-b border-gray-200 bg-white">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-xl font-bold text-gray-800">LocalDrop</h1>
            <p className="text-sm text-gray-500">我的设备: {myDeviceName}</p>
          </div>
          <button
            onClick={handleRefresh}
            className="flex items-center gap-2 px-4 py-2 bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors"
          >
            <RefreshCw className={`w-4 h-4 ${isDiscovering ? 'animate-spin' : ''}`} />
            刷新
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-4">
        <div className="flex items-center gap-2 mb-4">
          {isDiscovering ? (
            <>
              <Wifi className="w-4 h-4 text-green-500" />
              <span className="text-sm text-green-600">正在搜索设备...</span>
            </>
          ) : (
            <>
              <WifiOff className="w-4 h-4 text-gray-400" />
              <span className="text-sm text-gray-500">未搜索设备</span>
            </>
          )}
        </div>

        {onlineDevices.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-64 text-gray-400">
            <div className="w-16 h-16 rounded-full bg-gray-100 flex items-center justify-center mb-4">
              <Wifi className="w-8 h-8" />
            </div>
            <p className="text-lg">未发现设备</p>
            <p className="text-sm mt-2">确保设备在同一局域网内</p>
          </div>
        ) : (
          <div className="grid gap-3">
            {onlineDevices.map((device) => (
              <DeviceCard
                key={device.device_id}
                device={device}
                onClick={() => handleDeviceClick(device)}
              />
            ))}
          </div>
        )}
      </div>

      <div className="p-4 border-t border-gray-200 bg-white">
        <button
          onClick={() => onNavigate('/transfers')}
          className="w-full flex items-center justify-center gap-2 px-6 py-3 bg-blue-500 text-white rounded-xl hover:bg-blue-600 transition-colors font-medium"
        >
          <Send className="w-5 h-5" />
          查看传输记录
        </button>
      </div>

      {showFilePicker && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white rounded-xl p-6 w-96 shadow-xl">
            <h3 className="text-lg font-semibold text-gray-800 mb-4">
              选择要发送的文件
            </h3>
            <p className="text-sm text-gray-500 mb-6">
              发送给: {selectedDevice?.device_name}
            </p>
            <div className="flex gap-3">
              <button
                onClick={() => { setShowFilePicker(false); setSelectedDevice(null); }}
                className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors"
              >
                取消
              </button>
              <button
                onClick={handleFileSelect}
                className="flex-1 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors flex items-center justify-center gap-2"
              >
                <Plus className="w-4 h-4" />
                选择文件
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
