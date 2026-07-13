import { create } from 'zustand';
import type { Transfer, TransferSession, TransferProgress, SessionProgress, TransferStatus } from '@/types';

interface TransferStore {
  transfers: Transfer[];
  sessions: TransferSession[];
  
  addTransfer: (transfer: Transfer) => void;
  updateTransfer: (transfer: Partial<Transfer> & { transfer_id: string }) => void;
  updateTransferProgress: (progress: TransferProgress) => void;
  updateTransferStatus: (status: TransferStatus) => void;
  removeTransfer: (transferId: string) => void;
  
  addSession: (session: TransferSession) => void;
  updateSession: (session: Partial<TransferSession> & { session_id: string }) => void;
  updateSessionProgress: (progress: SessionProgress) => void;
  removeSession: (sessionId: string) => void;
  
  clearAll: () => void;
}

export const useTransferStore = create<TransferStore>((set) => ({
  transfers: [],
  sessions: [],
  
  addTransfer: (transfer) => set((state) => ({
    transfers: [...state.transfers, transfer],
  })),
  
  updateTransfer: (update) => set((state) => ({
    transfers: state.transfers.map(t => 
      t.transfer_id === update.transfer_id ? { ...t, ...update } : t
    ),
  })),
  
  updateTransferProgress: (progress) => set((state) => ({
    transfers: state.transfers.map(t => 
      t.transfer_id === progress.transfer_id 
        ? { 
            ...t, 
            progress: progress.progress,
            speed: progress.speed,
            speed_human: progress.speed_human,
            eta_human: progress.eta_human,
            status: progress.status as Transfer['status'],
          }
        : t
    ),
  })),
  
  updateTransferStatus: (status) => set((state) => ({
    transfers: state.transfers.map(t => 
      t.transfer_id === status.transfer_id 
        ? { ...t, status: status.status }
        : t
    ),
  })),
  
  removeTransfer: (transferId) => set((state) => ({
    transfers: state.transfers.filter(t => t.transfer_id !== transferId),
  })),
  
  addSession: (session) => set((state) => ({
    sessions: [...state.sessions, session],
  })),
  
  updateSession: (update) => set((state) => ({
    sessions: state.sessions.map(s => 
      s.session_id === update.session_id ? { ...s, ...update } : s
    ),
  })),
  
  updateSessionProgress: (progress) => set((state) => ({
    sessions: state.sessions.map(s => 
      s.session_id === progress.session_id 
        ? { 
            ...s,
            progress: progress.progress,
            speed: progress.speed,
            speed_human: progress.speed_human,
            eta_human: progress.eta_human,
            status: progress.status as TransferSession['status'],
            completed_files: progress.completed_files,
            bytes_transferred: progress.bytes_transferred,
            files: progress.files,
          }
        : s
    ),
  })),
  
  removeSession: (sessionId) => set((state) => ({
    sessions: state.sessions.filter(s => s.session_id !== sessionId),
  })),
  
  clearAll: () => set({ transfers: [], sessions: [] }),
}));
