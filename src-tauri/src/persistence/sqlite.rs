use rusqlite::{Connection, Result};
use chrono::Utc;
use crate::utils::config::get_config_dir;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let dir = get_config_dir().unwrap_or_else(|_| std::path::PathBuf::from("./data"));
        std::fs::create_dir_all(&dir).unwrap_or_default();
        let path = dir.join("transfers.db");
        
        let conn = Connection::open(&path)?;
        Self::create_tables(&conn)?;
        
        Ok(Self { conn })
    }
    
    fn create_tables(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS transfer_sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL UNIQUE,
                session_name TEXT NOT NULL,
                total_files INTEGER NOT NULL,
                completed_files INTEGER NOT NULL DEFAULT 0,
                total_size INTEGER NOT NULL,
                bytes_transferred INTEGER NOT NULL DEFAULT 0,
                peer_device_id TEXT NOT NULL,
                peer_device_name TEXT NOT NULL,
                direction TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS transfers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT,
                file_id TEXT NOT NULL UNIQUE,
                file_name TEXT NOT NULL,
                relative_path TEXT NOT NULL DEFAULT '',
                file_path TEXT NOT NULL,
                file_size INTEGER NOT NULL,
                file_hash TEXT NOT NULL,
                peer_device_id TEXT NOT NULL,
                peer_device_name TEXT NOT NULL,
                direction TEXT NOT NULL,
                status TEXT NOT NULL,
                total_chunks INTEGER NOT NULL,
                completed_chunks INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                FOREIGN KEY (session_id) REFERENCES transfer_sessions(session_id)
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS chunks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                transfer_id INTEGER NOT NULL,
                chunk_index INTEGER NOT NULL,
                chunk_size INTEGER NOT NULL,
                chunk_hash TEXT NOT NULL,
                status TEXT NOT NULL,
                FOREIGN KEY (transfer_id) REFERENCES transfers(id),
                UNIQUE(transfer_id, chunk_index)
            )",
            [],
        )?;
        
        Ok(())
    }
    
    pub fn insert_session(&self, session: &SessionData) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO transfer_sessions (
                session_id, session_name, total_files, completed_files,
                total_size, bytes_transferred, peer_device_id, peer_device_name,
                direction, status, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            rusqlite::params![
                session.session_id,
                session.session_name,
                session.total_files,
                session.completed_files,
                session.total_size,
                session.bytes_transferred,
                session.peer_device_id,
                session.peer_device_name,
                session.direction,
                session.status,
                session.created_at,
                session.updated_at,
            ],
        )?;
        Ok(())
    }
    
    pub fn update_session_progress(&self, session_id: &str, bytes_transferred: u64, completed_files: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE transfer_sessions SET bytes_transferred = ?1, completed_files = ?2, updated_at = ?3 WHERE session_id = ?4",
            rusqlite::params![bytes_transferred, completed_files, chrono::Utc::now().timestamp(), session_id],
        )?;
        Ok(())
    }
    
    pub fn update_session_status(&self, session_id: &str, status: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE transfer_sessions SET status = ?1, updated_at = ?2 WHERE session_id = ?3",
            rusqlite::params![status, chrono::Utc::now().timestamp(), session_id],
        )?;
        Ok(())
    }
    
    pub fn insert_transfer(&self, transfer: &TransferData) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO transfers (
                session_id, file_id, file_name, relative_path, file_path,
                file_size, file_hash, peer_device_id, peer_device_name,
                direction, status, total_chunks, completed_chunks,
                created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            rusqlite::params![
                transfer.session_id,
                transfer.file_id,
                transfer.file_name,
                transfer.relative_path,
                transfer.file_path,
                transfer.file_size,
                transfer.file_hash,
                transfer.peer_device_id,
                transfer.peer_device_name,
                transfer.direction,
                transfer.status,
                transfer.total_chunks,
                transfer.completed_chunks,
                transfer.created_at,
                transfer.updated_at,
            ],
        )?;
        Ok(())
    }
    
    pub fn update_transfer_progress(&self, file_id: &str, completed_chunks: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE transfers SET completed_chunks = ?1, updated_at = ?2 WHERE file_id = ?3",
            rusqlite::params![completed_chunks, chrono::Utc::now().timestamp(), file_id],
        )?;
        Ok(())
    }
    
    pub fn update_transfer_status(&self, file_id: &str, status: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE transfers SET status = ?1, updated_at = ?2 WHERE file_id = ?3",
            rusqlite::params![status, chrono::Utc::now().timestamp(), file_id],
        )?;
        Ok(())
    }
    
    pub fn insert_chunk(&self, chunk: &ChunkData) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO chunks (transfer_id, chunk_index, chunk_size, chunk_hash, status)
             VALUES ((SELECT id FROM transfers WHERE file_id = ?1), ?2, ?3, ?4, ?5)",
            rusqlite::params![chunk.file_id, chunk.chunk_index, chunk.chunk_size, chunk.chunk_hash, chunk.status],
        )?;
        Ok(())
    }
    
    pub fn update_chunk_status(&self, file_id: &str, chunk_index: i64, status: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE chunks SET status = ?1 WHERE transfer_id = (SELECT id FROM transfers WHERE file_id = ?2) AND chunk_index = ?3",
            rusqlite::params![status, file_id, chunk_index],
        )?;
        Ok(())
    }
    
    pub fn get_incomplete_transfers(&self) -> Result<Vec<TransferData>> {
        let mut stmt = self.conn.prepare(
            "SELECT session_id, file_id, file_name, relative_path, file_path,
                    file_size, file_hash, peer_device_id, peer_device_name,
                    direction, status, total_chunks, completed_chunks,
                    created_at, updated_at
             FROM transfers WHERE status IN ('sending', 'receiving', 'paused')"
        )?;
        
        let transfers = stmt.query_map([], |row| {
            Ok(TransferData {
                session_id: row.get(0)?,
                file_id: row.get(1)?,
                file_name: row.get(2)?,
                relative_path: row.get(3)?,
                file_path: row.get(4)?,
                file_size: row.get(5)?,
                file_hash: row.get(6)?,
                peer_device_id: row.get(7)?,
                peer_device_name: row.get(8)?,
                direction: row.get(9)?,
                status: row.get(10)?,
                total_chunks: row.get(11)?,
                completed_chunks: row.get(12)?,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
            })
        })?.collect::<Result<Vec<_>>>()?;
        
        Ok(transfers)
    }
    
    pub fn get_completed_chunks(&self, file_id: &str) -> Result<Vec<i64>> {
        let mut stmt = self.conn.prepare(
            "SELECT chunk_index FROM chunks 
             WHERE transfer_id = (SELECT id FROM transfers WHERE file_id = ?1) 
             AND status IN ('written', 'acked', 'verified')"
        )?;
        
        let chunks = stmt.query_map([file_id], |row| row.get(0))?.collect::<Result<Vec<_>>>()?;
        Ok(chunks)
    }
    
    pub fn delete_transfer(&self, file_id: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM chunks WHERE transfer_id = (SELECT id FROM transfers WHERE file_id = ?1)",
            [file_id],
        )?;
        self.conn.execute("DELETE FROM transfers WHERE file_id = ?1", [file_id])?;
        Ok(())
    }
    
    pub fn delete_session(&self, session_id: &str) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT file_id FROM transfers WHERE session_id = ?1")?;
        let file_ids: Vec<String> = stmt.query_map([session_id], |row| row.get(0))?.collect::<Result<Vec<_>>>()?;
        
        for file_id in file_ids {
            self.delete_transfer(&file_id)?;
        }
        
        self.conn.execute("DELETE FROM transfer_sessions WHERE session_id = ?1", [session_id])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct SessionData {
    pub session_id: String,
    pub session_name: String,
    pub total_files: i64,
    pub completed_files: i64,
    pub total_size: u64,
    pub bytes_transferred: u64,
    pub peer_device_id: String,
    pub peer_device_name: String,
    pub direction: String,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug)]
pub struct TransferData {
    pub session_id: Option<String>,
    pub file_id: String,
    pub file_name: String,
    pub relative_path: String,
    pub file_path: String,
    pub file_size: u64,
    pub file_hash: String,
    pub peer_device_id: String,
    pub peer_device_name: String,
    pub direction: String,
    pub status: String,
    pub total_chunks: i64,
    pub completed_chunks: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug)]
pub struct ChunkData {
    pub file_id: String,
    pub chunk_index: i64,
    pub chunk_size: i64,
    pub chunk_hash: String,
    pub status: String,
}
