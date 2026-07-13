export interface Device {
  device_id: string;
  device_name: string;
  device_type: 'windows' | 'macos' | 'android' | 'ios';
  ip_address: string;
  tcp_port: number;
  online: boolean;
  last_seen: number;
}

export interface TransferFile {
  file_id: string;
  file_name: string;
  relative_path: string;
  file_size: number;
  progress: number;
  status: 'pending' | 'sending' | 'receiving' | 'completed' | 'failed' | 'cancelled';
}

export interface TransferProgress {
  transfer_id: string;
  session_id?: string;
  bytes_transferred: number;
  total_bytes: number;
  progress: number;
  speed: number;
  speed_human: string;
  eta: number;
  eta_human: string;
  file_name: string;
  status: 'sending' | 'receiving' | 'paused' | 'completed' | 'failed' | 'cancelled';
}

export interface SessionProgress {
  session_id: string;
  session_name: string;
  total_files: number;
  completed_files: number;
  total_size: number;
  bytes_transferred: number;
  progress: number;
  speed: number;
  speed_human: string;
  eta: number;
  eta_human: string;
  status: 'sending' | 'receiving' | 'paused' | 'completed' | 'failed' | 'cancelled';
  files: Array<{
    file_id: string;
    file_name: string;
    relative_path: string;
    file_size: number;
    progress: number;
    status: 'pending' | 'sending' | 'receiving' | 'completed' | 'failed' | 'cancelled';
  }>;
}

export interface TransferStatus {
  transfer_id: string;
  status: 'pending' | 'sending' | 'receiving' | 'paused' | 'completed' | 'failed';
  error?: string;
}

export interface DeviceUpdate {
  device_id: string;
  device_name: string;
  device_type: 'windows' | 'macos' | 'android' | 'ios';
  ip_address: string;
  tcp_port: number;
  online: boolean;
}

export interface TransferSession {
  session_id: string;
  session_name: string;
  peer_device_id: string;
  peer_device_name: string;
  direction: 'send' | 'receive';
  status: 'pending' | 'sending' | 'receiving' | 'paused' | 'completed' | 'failed' | 'cancelled';
  total_files: number;
  completed_files: number;
  total_size: number;
  bytes_transferred: number;
  progress: number;
  speed: number;
  speed_human: string;
  eta_human: string;
  files: TransferFile[];
  created_at: number;
}

export interface Transfer {
  transfer_id: string;
  session_id?: string;
  file_name: string;
  file_path: string;
  file_size: number;
  peer_device_id: string;
  peer_device_name: string;
  direction: 'send' | 'receive';
  status: 'pending' | 'sending' | 'receiving' | 'paused' | 'completed' | 'failed' | 'cancelled';
  progress: number;
  speed: number;
  speed_human: string;
  eta_human: string;
  created_at: number;
}

export interface Settings {
  device_name: string;
  chunk_size: number;
  max_connections: number;
  broadcast_port: number;
  tcp_port: number;
  save_path: string;
  auto_accept: boolean;
}

export const DEFAULT_CHUNK_SIZE = 64 * 1024;
export const DEFAULT_MAX_CONNECTIONS = 4;
export const DEFAULT_BROADCAST_PORT = 50000;
export const DEFAULT_TCP_PORT = 50001;
