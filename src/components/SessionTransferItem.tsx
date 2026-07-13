import { useState } from 'react';
import { Folder, ChevronDown, ChevronRight, Upload, Download, Pause, Play, X, CheckCircle, AlertCircle } from 'lucide-react';
import type { TransferSession } from '@/types';
import { ProgressBar } from './ProgressBar';
import { tauriApi } from '@/api/tauri';

interface SessionTransferItemProps {
  session: TransferSession;
}

const statusIcons: Record<TransferSession['status'], typeof Upload> = {
  pending: Download,
  sending: Upload,
  receiving: Download,
  paused: Pause,
  completed: CheckCircle,
  failed: AlertCircle,
  cancelled: X,
};

const statusColors: Record<TransferSession['status'], string> = {
  pending: 'text-gray-500',
  sending: 'text-blue-500',
  receiving: 'text-green-500',
  paused: 'text-yellow-500',
  completed: 'text-green-600',
  failed: 'text-red-500',
  cancelled: 'text-gray-500',
};

const statusLabels: Record<TransferSession['status'], string> = {
  pending: '等待中',
  sending: '发送中',
  receiving: '接收中',
  paused: '已暂停',
  completed: '已完成',
  failed: '失败',
  cancelled: '已取消',
};

export const SessionTransferItem = ({ session }: SessionTransferItemProps) => {
  const [expanded, setExpanded] = useState(false);
  const Icon = statusIcons[session.status];
  
  const handlePause = async () => {
    if (session.status === 'sending' || session.status === 'receiving') {
      await tauriApi.pauseTransfer(session.session_id);
    }
  };
  
  const handleResume = async () => {
    if (session.status === 'paused') {
      await tauriApi.resumeTransfer(session.session_id);
    }
  };
  
  const handleCancel = async () => {
    await tauriApi.cancelTransfer(session.session_id);
  };

  const formatSize = (bytes: number) => {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
    return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB';
  };

  return (
    <div className="rounded-lg bg-gray-50 border border-gray-200 overflow-hidden">
      <div 
        className="p-3 cursor-pointer hover:bg-gray-100 transition-colors"
        onClick={() => setExpanded(!expanded)}
      >
        <div className="flex items-center gap-3">
          <button className="flex-shrink-0">
            {expanded ? (
              <ChevronDown className="w-5 h-5 text-gray-400" />
            ) : (
              <ChevronRight className="w-5 h-5 text-gray-400" />
            )}
          </button>
          <div className={`p-2 rounded-lg ${statusColors[session.status].replace('text-', 'bg-').replace('-500', '-100')}`}>
            <Icon className={`w-5 h-5 ${statusColors[session.status]}`} />
          </div>
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2">
              <Folder className="w-4 h-4 text-gray-400" />
              <span className="font-medium text-gray-800 truncate">
                {session.session_name}
              </span>
            </div>
            <p className="text-xs text-gray-500">
              {session.total_files} 个文件 · {formatSize(session.total_size)}
            </p>
          </div>
          <div className="text-right">
            <p className="text-sm font-medium text-gray-700">
              {statusLabels[session.status]}
            </p>
            <p className="text-xs text-gray-500">
              {session.completed_files}/{session.total_files}
            </p>
          </div>
        </div>
        
        {(session.status === 'sending' || session.status === 'receiving' || session.status === 'paused') && (
          <ProgressBar 
            progress={session.progress}
            speed={session.speed_human}
            eta={session.eta_human}
            className="mt-3"
          />
        )}
        
        {(session.status === 'sending' || session.status === 'receiving' || session.status === 'paused') && (
          <div className="flex gap-2 mt-3">
            {session.status === 'paused' ? (
              <button
                onClick={(e) => { e.stopPropagation(); handleResume(); }}
                className="flex items-center gap-1 px-3 py-1.5 text-sm bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors"
              >
                <Play className="w-4 h-4" />
                继续
              </button>
            ) : (
              <button
                onClick={(e) => { e.stopPropagation(); handlePause(); }}
                className="flex items-center gap-1 px-3 py-1.5 text-sm bg-yellow-500 text-white rounded-lg hover:bg-yellow-600 transition-colors"
              >
                <Pause className="w-4 h-4" />
                暂停
              </button>
            )}
            <button
              onClick={(e) => { e.stopPropagation(); handleCancel(); }}
              className="flex items-center gap-1 px-3 py-1.5 text-sm bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors"
            >
              <X className="w-4 h-4" />
              取消
            </button>
          </div>
        )}
      </div>
      
      {expanded && (
        <div className="border-t border-gray-200 bg-white">
          {session.files.map((file) => (
            <div 
              key={file.file_id}
              className="px-3 py-2 flex items-center gap-3 hover:bg-gray-50 transition-colors"
            >
              <div className="w-16 text-xs text-gray-500">
                {file.progress.toFixed(0)}%
              </div>
              <div className="flex-1 min-w-0">
                <div className="h-1.5 bg-gray-200 rounded-full overflow-hidden">
                  <div
                    className="h-full bg-blue-500 rounded-full transition-all duration-300"
                    style={{ width: `${file.progress}%` }}
                  />
                </div>
              </div>
              <span className="text-sm text-gray-700 truncate max-w-xs">
                {file.file_name}
              </span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
