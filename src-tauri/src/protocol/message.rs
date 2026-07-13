use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageType {
    Discovery,
    DiscoveryResponse,
    Handshake,
    FileListMetadata,
    FileMetadata,
    ChunkRequest,
    ChunkData,
    ChunkAck,
    TransferStart,
    TransferPause,
    TransferCancel,
    TransferComplete,
    SessionComplete,
    CheckResume,
    ResumeInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscoveryMessage {
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub tcp_port: u16,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HandshakeMessage {
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileMetadata {
    pub file_id: String,
    pub file_name: String,
    pub relative_path: String,
    pub file_size: u64,
    pub file_hash: String,
    pub total_chunks: i64,
    pub chunk_size: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileListMetadata {
    pub session_id: String,
    pub session_name: String,
    pub total_files: i64,
    pub total_size: u64,
    pub files: Vec<FileMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChunkRequest {
    pub file_id: String,
    pub chunk_index: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChunkData {
    pub file_id: String,
    pub chunk_index: i64,
    pub chunk_size: usize,
    pub chunk_hash: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChunkAck {
    pub file_id: String,
    pub chunk_index: i64,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferCancel {
    pub transfer_id: String,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferComplete {
    pub file_id: String,
    pub file_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionComplete {
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckResume {
    pub file_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResumeInfo {
    pub file_id: String,
    pub completed_chunks: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub message_type: MessageType,
    pub payload: MessagePayload,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MessagePayload {
    Discovery(DiscoveryMessage),
    DiscoveryResponse(DiscoveryMessage),
    Handshake(HandshakeMessage),
    FileListMetadata(FileListMetadata),
    FileMetadata(FileMetadata),
    ChunkRequest(ChunkRequest),
    ChunkData(ChunkData),
    ChunkAck(ChunkAck),
    TransferStart,
    TransferPause,
    TransferCancel(TransferCancel),
    TransferComplete(TransferComplete),
    SessionComplete(SessionComplete),
    CheckResume(CheckResume),
    ResumeInfo(ResumeInfo),
}

impl Message {
    pub fn new(message_type: MessageType, payload: MessagePayload) -> Self {
        Self { message_type, payload }
    }
}
