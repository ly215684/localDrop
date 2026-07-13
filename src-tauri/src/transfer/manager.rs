use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use std::collections::HashMap;
use crate::transfer::worker::TransferWorker;
use crate::persistence::sqlite::Database;
use crate::utils::config::AppConfig;

pub struct TransferManager {
    database: Arc<Mutex<Database>>,
    config: Arc<Mutex<AppConfig>>,
    active_transfers: Arc<Mutex<HashMap<String, tokio::task::JoinHandle<()>>>>,
}

impl TransferManager {
    pub fn new(database: Arc<Mutex<Database>>, config: Arc<Mutex<AppConfig>>) -> Self {
        Self {
            database,
            config,
            active_transfers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub async fn send_files(&self, device_ip: String, device_port: u16, file_paths: Vec<String>, peer_device_id: String, peer_device_name: String) -> Result<String, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(format!("{}:{}", device_ip, device_port)).await?;
        
        let config = self.config.lock().unwrap();
        let chunk_size = config.chunk_size;
        let max_connections = config.max_connections;
        
        let session_id = uuid::Uuid::new_v4().to_string();
        
        let database_clone = Arc::clone(&self.database);
        
        let handle = tokio::spawn(async move {
            let mut worker = TransferWorker::new(stream, database_clone, chunk_size, max_connections);
            
            if let Err(e) = worker.run_sender(file_paths, peer_device_id, peer_device_name).await {
                eprintln!("Transfer failed: {}", e);
            }
        });
        
        self.active_transfers.lock().unwrap().insert(session_id.clone(), handle);
        
        Ok(session_id)
    }
    
    pub async fn accept_transfer(&self, transfer_id: String) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    
    pub async fn cancel_transfer(&self, transfer_id: String) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(handle) = self.active_transfers.lock().unwrap().remove(&transfer_id) {
            handle.abort();
        }
        
        let _ = self.database.lock().unwrap().delete_session(&transfer_id);
        
        Ok(())
    }
    
    pub async fn pause_transfer(&self, transfer_id: String) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    
    pub async fn resume_transfer(&self, transfer_id: String) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
