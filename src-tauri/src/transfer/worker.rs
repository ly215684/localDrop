use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tokio::sync::Mutex as TokioMutex;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use futures::StreamExt;
use futures::SinkExt;
use crate::protocol::codec::MessageCodec;
use crate::protocol::message::{Message, MessageType, MessagePayload, ChunkRequest, ChunkData, ChunkAck, FileMetadata, FileListMetadata, CheckResume, ResumeInfo, TransferCancel};
use crate::file::splitter::FileSplitter;
use crate::file::merger::FileMerger;
use crate::persistence::sqlite::Database;
use crate::utils::hash::calculate_sha256_file;
use uuid::Uuid;

pub struct TransferWorker {
    stream: Arc<TokioMutex<Framed<TcpStream, MessageCodec>>>,
    database: Arc<Mutex<Database>>,
    chunk_size: usize,
    max_connections: usize,
}

impl TransferWorker {
    pub fn new(stream: TcpStream, database: Arc<Mutex<Database>>, chunk_size: usize, max_connections: usize) -> Self {
        Self {
            stream: Arc::new(TokioMutex::new(Framed::new(stream, MessageCodec))),
            database,
            chunk_size,
            max_connections,
        }
    }
    
    pub async fn run_sender(&self, file_paths: Vec<String>, peer_device_id: String, peer_device_name: String) -> Result<(), Box<dyn std::error::Error>> {
        let session_id = Uuid::new_v4().to_string();
        
        let mut file_metadata_list = Vec::new();
        let mut total_size: u64 = 0;
        
        for file_path in &file_paths {
            let file_hash = calculate_sha256_file(file_path)?;
            let file_size = std::fs::metadata(file_path)?.len();
            let total_chunks = ((file_size + self.chunk_size as u64 - 1) / self.chunk_size as u64) as i64;
            
            file_metadata_list.push(FileMetadata {
                file_id: Uuid::new_v4().to_string(),
                file_name: std::path::Path::new(file_path).file_name().unwrap().to_string_lossy().to_string(),
                relative_path: String::new(),
                file_size,
                file_hash,
                total_chunks,
                chunk_size: self.chunk_size,
            });
            
            total_size += file_size;
        }
        
        let session_name = if file_paths.len() == 1 {
            std::path::Path::new(&file_paths[0]).file_name().unwrap().to_string_lossy().to_string()
        } else {
            "多个文件".to_string()
        };
        
        let file_list_msg = Message::new(
            MessageType::FileListMetadata,
            MessagePayload::FileListMetadata(FileListMetadata {
                session_id: session_id.clone(),
                session_name: session_name.clone(),
                total_files: file_paths.len() as i64,
                total_size,
                files: file_metadata_list.clone(),
            })
        );
        
        self.stream.lock().await.send(file_list_msg).await?;
        
        for (file_path, file_metadata) in file_paths.iter().zip(file_metadata_list.into_iter()) {
            let msg = Message::new(
                MessageType::FileMetadata,
                MessagePayload::FileMetadata(file_metadata.clone())
            );
            self.stream.lock().await.send(msg).await?;
            
            if let Some(Message { message_type: MessageType::CheckResume, payload: MessagePayload::CheckResume(req) }) = self.stream.lock().await.next().await.transpose()? {
                let completed_chunks = self.database.lock().unwrap().get_completed_chunks(&req.file_id)?;
                
                let resume_msg = Message::new(
                    MessageType::ResumeInfo,
                    MessagePayload::ResumeInfo(ResumeInfo {
                        file_id: req.file_id,
                        completed_chunks,
                    })
                );
                self.stream.lock().await.send(resume_msg).await?;
            }
            
            self.send_file(file_path, &file_metadata).await?;
            
            let complete_msg = Message::new(
                MessageType::TransferComplete,
                MessagePayload::TransferComplete(crate::protocol::message::TransferComplete {
                    file_id: file_metadata.file_id.clone(),
                    file_hash: file_metadata.file_hash.clone(),
                })
            );
            self.stream.lock().await.send(complete_msg).await?;
        }
        
        let session_complete_msg = Message::new(
            MessageType::SessionComplete,
            MessagePayload::SessionComplete(crate::protocol::message::SessionComplete {
                session_id,
            })
        );
        self.stream.lock().await.send(session_complete_msg).await?;
        
        Ok(())
    }
    
    async fn send_file(&self, file_path: &str, metadata: &FileMetadata) -> Result<(), Box<dyn std::error::Error>> {
        let completed_chunks = Arc::new(Mutex::new(HashSet::new()));
        
        let msg = Message::new(
            MessageType::TransferStart,
            MessagePayload::TransferStart,
        );
        self.stream.lock().await.send(msg).await?;
        
        let mut active_workers = Vec::new();
        let stream_clone = Arc::clone(&self.stream);
        let database_clone = Arc::clone(&self.database);
        
        for _ in 0..self.max_connections {
            let stream_clone = Arc::clone(&stream_clone);
            let completed_chunks_clone = Arc::clone(&completed_chunks);
            let file_path_clone = file_path.to_string();
            let metadata_clone = metadata.clone();
            let chunk_size = self.chunk_size;
            
            let worker = tokio::spawn(async move {
                let mut splitter = match FileSplitter::new(&file_path_clone, chunk_size) {
                    Ok(s) => s,
                    Err(_) => return,
                };
                
                while completed_chunks_clone.lock().unwrap().len() < metadata_clone.total_chunks as usize {
                    let chunk_index = {
                        let mut completed = completed_chunks_clone.lock().unwrap();
                        let mut idx = 0;
                        while idx < metadata_clone.total_chunks && completed.contains(&idx) {
                            idx += 1;
                        }
                        if idx >= metadata_clone.total_chunks {
                            return;
                        }
                        idx
                    };
                    
                    match splitter.read_chunk(chunk_index) {
                        Ok((data, crc32)) => {
                            let chunk_msg = Message::new(
                                MessageType::ChunkData,
                                MessagePayload::ChunkData(ChunkData {
                                    file_id: metadata_clone.file_id.clone(),
                                    chunk_index,
                                    chunk_size: data.len(),
                                    chunk_hash: format!("{:x}", crc32),
                                    data,
                                })
                            );
                            
                            if let Err(_) = stream_clone.lock().await.send(chunk_msg).await {
                                return;
                            }
                            
                            if let Some(Message { message_type: MessageType::ChunkAck, payload: MessagePayload::ChunkAck(ack) }) = stream_clone.lock().await.next().await.transpose().unwrap_or(None) {
                                if ack.success {
                                    completed_chunks_clone.lock().unwrap().insert(chunk_index);
                                }
                            }
                        }
                        Err(_) => {
                            return;
                        }
                    }
                }
            });
            
            active_workers.push(worker);
        }
        
        for worker in active_workers {
            let _ = worker.await;
        }
        
        Ok(())
    }
    
    pub async fn run_receiver(&self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(msg) = self.stream.lock().await.next().await.transpose()? {
            match msg.message_type {
                MessageType::FileListMetadata => {
                    if let MessagePayload::FileListMetadata(list) = msg.payload {
                        self.receive_session(list).await?;
                    }
                }
                MessageType::TransferCancel => {
                    if let MessagePayload::TransferCancel(cancel) = msg.payload {
                        self.handle_cancel(&cancel).await?;
                        break;
                    }
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    async fn receive_session(&self, list: FileListMetadata) -> Result<(), Box<dyn std::error::Error>> {
        for file_metadata in list.files {
            let check_resume_msg = Message::new(
                MessageType::CheckResume,
                MessagePayload::CheckResume(CheckResume {
                    file_id: file_metadata.file_id.clone(),
                })
            );
            self.stream.lock().await.send(check_resume_msg).await?;
            
            if let Some(Message { message_type: MessageType::ResumeInfo, payload: MessagePayload::ResumeInfo(resume) }) = self.stream.lock().await.next().await.transpose()? {
                self.receive_file(&file_metadata, &resume.completed_chunks).await?;
            }
        }
        
        Ok(())
    }
    
    async fn receive_file(&self, metadata: &FileMetadata, completed_chunks: &[i64]) -> Result<(), Box<dyn std::error::Error>> {
        let save_path = "./downloads";
        std::fs::create_dir_all(save_path)?;
        let file_path = format!("{}/{}", save_path, metadata.file_name);
        
        let mut merger = FileMerger::new(&file_path, metadata.file_size, metadata.chunk_size)?;
        
        for chunk_index in 0..metadata.total_chunks {
            if completed_chunks.contains(&chunk_index) {
                continue;
            }
            
            let request_msg = Message::new(
                MessageType::ChunkRequest,
                MessagePayload::ChunkRequest(ChunkRequest {
                    file_id: metadata.file_id.clone(),
                    chunk_index,
                })
            );
            self.stream.lock().await.send(request_msg).await?;
            
            if let Some(Message { message_type: MessageType::ChunkData, payload: MessagePayload::ChunkData(chunk) }) = self.stream.lock().await.next().await.transpose()? {
                let crc32 = u32::from_str_radix(&chunk.chunk_hash, 16).unwrap_or(0);
                
                if merger.write_chunk(chunk.chunk_index, &chunk.data, crc32)? {
                    let ack_msg = Message::new(
                        MessageType::ChunkAck,
                        MessagePayload::ChunkAck(ChunkAck {
                            file_id: metadata.file_id.clone(),
                            chunk_index: chunk.chunk_index,
                            success: true,
                            error: None,
                        })
                    );
                    self.stream.lock().await.send(ack_msg).await?;
                } else {
                    let ack_msg = Message::new(
                        MessageType::ChunkAck,
                        MessagePayload::ChunkAck(ChunkAck {
                            file_id: metadata.file_id.clone(),
                            chunk_index: chunk.chunk_index,
                            success: false,
                            error: Some("CRC32 mismatch".to_string()),
                        })
                    );
                    self.stream.lock().await.send(ack_msg).await?;
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_cancel(&self, cancel: &TransferCancel) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.database.lock().unwrap().delete_session(&cancel.transfer_id);
        
        let _ = std::fs::remove_dir_all("./downloads");
        
        Ok(())
    }
}