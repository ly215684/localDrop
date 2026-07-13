import { useState, useEffect } from 'react';
import { ArrowLeft, Save, User, HardDrive, Settings as SettingsIcon, Wifi } from 'lucide-react';
import { tauriApi } from '@/api/tauri';
import type { Settings } from '@/types';
import { DEFAULT_CHUNK_SIZE, DEFAULT_MAX_CONNECTIONS, DEFAULT_BROADCAST_PORT, DEFAULT_TCP_PORT } from '@/types';

interface SettingsPageProps {
  onNavigate: (page: string) => void;
}

export const SettingsPage = ({ onNavigate }: SettingsPageProps) => {
  const [settings, setSettings] = useState<Settings>({
    device_name: '',
    chunk_size: DEFAULT_CHUNK_SIZE,
    max_connections: DEFAULT_MAX_CONNECTIONS,
    broadcast_port: DEFAULT_BROADCAST_PORT,
    tcp_port: DEFAULT_TCP_PORT,
    save_path: '',
    auto_accept: false,
  });
  const [isSaving, setIsSaving] = useState(false);
  const [saveMessage, setSaveMessage] = useState('');

  useEffect(() => {
    const loadSettings = async () => {
      try {
        const savedSettings = await tauriApi.getSettings();
        setSettings(savedSettings);
      } catch (e) {
        console.error('Failed to load settings:', e);
      }
    };
    loadSettings();
  }, []);

  const handleSave = async () => {
    setIsSaving(true);
    try {
      await tauriApi.saveSettings(settings);
      await tauriApi.renameDevice(settings.device_name);
      setSaveMessage('设置已保存');
      setTimeout(() => setSaveMessage(''), 3000);
    } catch (e) {
      setSaveMessage('保存失败');
      setTimeout(() => setSaveMessage(''), 3000);
    } finally {
      setIsSaving(false);
    }
  };

  const chunkSizeOptions = [
    { value: 16 * 1024, label: '16 KB' },
    { value: 32 * 1024, label: '32 KB' },
    { value: 64 * 1024, label: '64 KB (推荐)' },
    { value: 128 * 1024, label: '128 KB' },
    { value: 256 * 1024, label: '256 KB' },
  ];

  return (
    <div className="h-full flex flex-col">
      <div className="p-4 border-b border-gray-200 bg-white">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <button
              onClick={() => onNavigate('/')}
              className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
            >
              <ArrowLeft className="w-5 h-5 text-gray-600" />
            </button>
            <div>
              <h1 className="text-xl font-bold text-gray-800">设置</h1>
              <p className="text-sm text-gray-500">配置应用参数</p>
            </div>
          </div>
          <button
            onClick={handleSave}
            disabled={isSaving}
            className={`flex items-center gap-2 px-4 py-2 rounded-lg transition-colors ${
              isSaving 
                ? 'bg-gray-300 text-gray-500 cursor-not-allowed' 
                : 'bg-blue-500 text-white hover:bg-blue-600'
            }`}
          >
            <Save className="w-4 h-4" />
            {isSaving ? '保存中...' : '保存'}
          </button>
        </div>
        {saveMessage && (
          <p className={`mt-2 text-sm ${saveMessage === '设置已保存' ? 'text-green-600' : 'text-red-600'}`}>
            {saveMessage}
          </p>
        )}
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-6">
        <div className="bg-white rounded-xl p-4 border border-gray-200">
          <div className="flex items-center gap-2 mb-4">
            <User className="w-5 h-5 text-blue-500" />
            <h2 className="font-semibold text-gray-800">设备信息</h2>
          </div>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                设备名称
              </label>
              <input
                type="text"
                value={settings.device_name}
                onChange={(e) => setSettings({ ...settings, device_name: e.target.value })}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                placeholder="输入设备名称"
              />
            </div>
          </div>
        </div>

        <div className="bg-white rounded-xl p-4 border border-gray-200">
          <div className="flex items-center gap-2 mb-4">
            <SettingsIcon className="w-5 h-5 text-blue-500" />
            <h2 className="font-semibold text-gray-800">传输设置</h2>
          </div>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                块大小
              </label>
              <select
                value={settings.chunk_size}
                onChange={(e) => setSettings({ ...settings, chunk_size: parseInt(e.target.value) })}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              >
                {chunkSizeOptions.map((option) => (
                  <option key={option.value} value={option.value}>
                    {option.label}
                  </option>
                ))}
              </select>
              <p className="text-xs text-gray-500 mt-1">
                较大的块可以提高传输速度，但会增加内存占用
              </p>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                最大并发连接数
              </label>
              <input
                type="number"
                value={settings.max_connections}
                onChange={(e) => setSettings({ ...settings, max_connections: parseInt(e.target.value) || 1 })}
                min={1}
                max={16}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              />
              <p className="text-xs text-gray-500 mt-1">
                同时传输的文件块数量，建议值为4-8
              </p>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-xl p-4 border border-gray-200">
          <div className="flex items-center gap-2 mb-4">
            <Wifi className="w-5 h-5 text-blue-500" />
            <h2 className="font-semibold text-gray-800">网络设置</h2>
          </div>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                广播端口
              </label>
              <input
                type="number"
                value={settings.broadcast_port}
                onChange={(e) => setSettings({ ...settings, broadcast_port: parseInt(e.target.value) || 50000 })}
                min={1}
                max={65535}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                TCP传输端口
              </label>
              <input
                type="number"
                value={settings.tcp_port}
                onChange={(e) => setSettings({ ...settings, tcp_port: parseInt(e.target.value) || 50001 })}
                min={1}
                max={65535}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              />
            </div>
          </div>
        </div>

        <div className="bg-white rounded-xl p-4 border border-gray-200">
          <div className="flex items-center gap-2 mb-4">
            <HardDrive className="w-5 h-5 text-blue-500" />
            <h2 className="font-semibold text-gray-800">文件设置</h2>
          </div>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                默认保存路径
              </label>
              <input
                type="text"
                value={settings.save_path}
                onChange={(e) => setSettings({ ...settings, save_path: e.target.value })}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                placeholder="接收文件的默认保存路径"
              />
            </div>
            <div className="flex items-center justify-between">
              <div>
                <label className="block text-sm font-medium text-gray-700">
                  自动接收
                </label>
                <p className="text-xs text-gray-500">
                  自动接受所有传输请求
                </p>
              </div>
              <button
                onClick={() => setSettings({ ...settings, auto_accept: !settings.auto_accept })}
                className={`relative w-12 h-6 rounded-full transition-colors ${
                  settings.auto_accept ? 'bg-blue-500' : 'bg-gray-300'
                }`}
              >
                <span
                  className={`absolute top-1 w-4 h-4 bg-white rounded-full transition-transform ${
                    settings.auto_accept ? 'translate-x-7' : 'translate-x-1'
                  }`}
                />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
