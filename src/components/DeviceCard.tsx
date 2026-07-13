import { Smartphone, Monitor, Laptop } from 'lucide-react';
import type { Device } from '@/types';

interface DeviceCardProps {
  device: Device;
  onClick: () => void;
}

const deviceIcons = {
  windows: Monitor,
  macos: Laptop,
  android: Smartphone,
  ios: Smartphone,
};

const deviceLabels = {
  windows: 'Windows',
  macos: 'macOS',
  android: 'Android',
  ios: 'iOS',
};

export const DeviceCard = ({ device, onClick }: DeviceCardProps) => {
  const Icon = deviceIcons[device.device_type];
  
  return (
    <div
      onClick={onClick}
      className={`
        p-4 rounded-xl border-2 cursor-pointer transition-all duration-200
        ${device.online 
          ? 'border-blue-200 bg-blue-50 hover:border-blue-400 hover:bg-blue-100' 
          : 'border-gray-200 bg-gray-50 opacity-60 cursor-not-allowed'
        }
      `}
    >
      <div className="flex items-center gap-3">
        <div className={`
          w-12 h-12 rounded-full flex items-center justify-center
          ${device.online ? 'bg-blue-500' : 'bg-gray-400'}
        `}>
          <Icon className="w-6 h-6 text-white" />
        </div>
        <div className="flex-1 min-w-0">
          <h3 className="font-semibold text-gray-800 truncate">
            {device.device_name}
          </h3>
          <p className="text-sm text-gray-500">
            {deviceLabels[device.device_type]}
          </p>
        </div>
        {device.online && (
          <span className="w-2 h-2 rounded-full bg-green-500" />
        )}
      </div>
      <p className="mt-2 text-xs text-gray-400 font-mono">
        {device.ip_address}:{device.tcp_port}
      </p>
    </div>
  );
};
