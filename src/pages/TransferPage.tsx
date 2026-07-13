import { ArrowLeft, Upload, Download, Clock } from 'lucide-react';
import { TransferItem } from '@/components/TransferItem';
import { SessionTransferItem } from '@/components/SessionTransferItem';
import { useTransferStore } from '@/stores/useTransferStore';

interface TransferPageProps {
  onNavigate: (page: string) => void;
}

export const TransferPage = ({ onNavigate }: TransferPageProps) => {
  const { transfers, sessions } = useTransferStore();

  const activeTransfers = transfers.filter(t => 
    t.status === 'sending' || t.status === 'receiving' || t.status === 'paused'
  );
  const completedTransfers = transfers.filter(t => 
    t.status === 'completed' || t.status === 'failed' || t.status === 'cancelled'
  );

  const activeSessions = sessions.filter(s => 
    s.status === 'sending' || s.status === 'receiving' || s.status === 'paused'
  );
  const completedSessions = sessions.filter(s => 
    s.status === 'completed' || s.status === 'failed' || s.status === 'cancelled'
  );

  return (
    <div className="h-full flex flex-col">
      <div className="p-4 border-b border-gray-200 bg-white">
        <div className="flex items-center gap-3">
          <button
            onClick={() => onNavigate('/')}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <ArrowLeft className="w-5 h-5 text-gray-600" />
          </button>
          <div>
            <h1 className="text-xl font-bold text-gray-800">传输记录</h1>
            <p className="text-sm text-gray-500">
              {activeTransfers.length + activeSessions.length} 个进行中 · {completedTransfers.length + completedSessions.length} 个已完成
            </p>
          </div>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-4">
        {activeSessions.length > 0 && (
          <div className="mb-6">
            <h2 className="flex items-center gap-2 text-sm font-semibold text-gray-700 mb-3">
              <Clock className="w-4 h-4" />
              会话传输中
            </h2>
            <div className="space-y-3">
              {activeSessions.map((session) => (
                <SessionTransferItem key={session.session_id} session={session} />
              ))}
            </div>
          </div>
        )}

        {activeTransfers.length > 0 && (
          <div className="mb-6">
            <h2 className="flex items-center gap-2 text-sm font-semibold text-gray-700 mb-3">
              <Upload className="w-4 h-4 text-blue-500" />
              <Download className="w-4 h-4 text-green-500" />
              文件传输中
            </h2>
            <div className="space-y-3">
              {activeTransfers.map((transfer) => (
                <TransferItem key={transfer.transfer_id} transfer={transfer} />
              ))}
            </div>
          </div>
        )}

        {completedSessions.length > 0 && (
          <div className="mb-6">
            <h2 className="flex items-center gap-2 text-sm font-semibold text-gray-700 mb-3">
              会话传输历史
            </h2>
            <div className="space-y-3">
              {completedSessions.map((session) => (
                <SessionTransferItem key={session.session_id} session={session} />
              ))}
            </div>
          </div>
        )}

        {completedTransfers.length > 0 && (
          <div>
            <h2 className="flex items-center gap-2 text-sm font-semibold text-gray-700 mb-3">
              文件传输历史
            </h2>
            <div className="space-y-3">
              {completedTransfers.map((transfer) => (
                <TransferItem key={transfer.transfer_id} transfer={transfer} />
              ))}
            </div>
          </div>
        )}

        {activeTransfers.length === 0 && activeSessions.length === 0 && 
         completedTransfers.length === 0 && completedSessions.length === 0 && (
          <div className="flex flex-col items-center justify-center h-64 text-gray-400">
            <div className="w-16 h-16 rounded-full bg-gray-100 flex items-center justify-center mb-4">
              <Upload className="w-8 h-8" />
            </div>
            <p className="text-lg">暂无传输记录</p>
            <p className="text-sm mt-2">从首页选择设备发送文件</p>
          </div>
        )}
      </div>
    </div>
  );
};
