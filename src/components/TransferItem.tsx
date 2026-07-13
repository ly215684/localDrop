import { File, Upload, Download, Pause, Play, X, CheckCircle, AlertCircle } from 'lucide-react';
import type { Transfer } from '@/types';
import { ProgressBar } from './ProgressBar';
import { tauriApi } from '@/api/tauri';

interface TransferItemProps {
  transfer: Transfer;
}

const statusIcons: Record<Transfer['status'], typeof Upload> = {
  pending: Download,
  sending: Upload,
  receiving: Download,
  paused: Pause,
  completed: CheckCircle,
  failed: AlertCircle,
  cancelled: X,
};

const statusColors: Record<Transfer['status'], string> = {
  pending: 'text-gray-500',
  sending: 'text-blue-500',
  receiving: 'text-green-500',
  paused: 'text-yellow-500',
  completed: 'text-green-600',
  failed: 'text-red-500',
  cancelled: 'text-gray-500',
};

const statusLabels: Record<Transfer['status'], string> = {
  pending: '等待中',
  sending: '发送中',
  receiving: '接收中',
  paused: '已暂停',
  completed: '已完成',
  failed: '失败',
  cancelled: '已取消',
};

export const TransferItem = ({ transfer }: TransferItemProps) => {
  const Icon = statusIcons[transfer.status];
  
  const handlePause = async () => {
    if (transfer.status === 'sending' || transfer.status === 'receiving') {
      await tauriApi.pauseTransfer(transfer.transfer_id);
    }
  };
  
  const handleResume = async () => {
    if (transfer.status === 'paused') {
      await tauriApi.resumeTransfer(transfer.transfer_id);
    }
  };
  
  const handleCancel = async () => {
    await tauriApi.cancelTransfer(transfer.transfer_id);
  };

  const formatSize = (bytes: number) => {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
    return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB';
  };

  return (
    <div className="p-3 rounded-lg bg-gray-50 border border-gray-200">
      <div className="flex items-center gap-3">
        <div className={`p-2 rounded-lg ${statusColors[transfer.status].replace('text-', 'bg-').replace('-500', '-100')}`}>
          <Icon className={`w-5 h-5 ${statusColors[transfer.status]}`} />
        </div>
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2">
            <File className="w-4 h-4 text-gray-400" />
            <span className="font-medium text-gray-800 truncate">
              {transfer.file_name}
            </span>
          </div>
          <p className="text-xs text-gray-500">
            {transfer.direction === 'send' ? '发送给' : '从'} {transfer.peer_device_name}
          </p>
        </div>
        <div className="text-right">
          <p className="text-sm font-medium text-gray-700">
            {formatSize(transfer.file_size)}
          </p>
          <p className="text-xs text-gray-500">
            {statusLabels[transfer.status]}
          </p>
        </div>
      </div>
      
      {(transfer.status === 'sending' || transfer.status === 'receiving' || transfer.status === 'paused') && (
        <ProgressBar 
          progress={transfer.progress}
          speed={transfer.speed_human}
          eta={transfer.eta_human}
          className="mt-3"
        />
      )}
      
      {(transfer.status === 'sending' || transfer.status === 'receiving' || transfer.status === 'paused') && (
        <div className="flex gap-2 mt-3">
          {transfer.status === 'paused' ? (
            <button
              onClick={handleResume}
              className="flex items-center gap-1 px-3 py-1.5 text-sm bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors"
            >
              <Play className="w-4 h-4" />
              继续
            </button>
          ) : (
            <button
              onClick={handlePause}
              className="flex items-center gap-1 px-3 py-1.5 text-sm bg-yellow-500 text-white rounded-lg hover:bg-yellow-600 transition-colors"
            >
              <Pause className="w-4 h-4" />
              暂停
            </button>
          )}
          <button
            onClick={handleCancel}
            className="flex items-center gap-1 px-3 py-1.5 text-sm bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors"
          >
            <X className="w-4 h-4" />
            取消
          </button>
        </div>
      )}
    </div>
  );
};
