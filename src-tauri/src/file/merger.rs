use std::fs::{self, File};
use std::io::{self, Write, Seek, SeekFrom};
use std::path::Path;
use crate::utils::hash::calculate_crc32;

pub struct FileMerger {
    file: File,
    chunk_size: usize,
    file_size: u64,
}

impl FileMerger {
    pub fn new(file_path: &str, file_size: u64, chunk_size: usize) -> io::Result<Self> {
        let path = Path::new(file_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let mut file = File::create(file_path)?;
        if file_size > 0 {
            #[cfg(unix)]
            {
                use std::os::unix::fs::FileExt;
                file.set_len(file_size)?;
            }
            #[cfg(windows)]
            {
                use std::os::windows::fs::FileExt;
                file.set_len(file_size)?;
            }
        }
        
        Ok(Self {
            file,
            chunk_size,
            file_size,
        })
    }
    
    pub fn write_chunk(&mut self, chunk_index: i64, data: &[u8], expected_crc32: u32) -> io::Result<bool> {
        let actual_crc32 = calculate_crc32(data);
        
        if actual_crc32 != expected_crc32 {
            return Ok(false);
        }
        
        let offset = chunk_index as u64 * self.chunk_size as u64;
        
        if offset >= self.file_size {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Chunk index out of range"));
        }
        
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(data)?;
        self.file.flush()?;
        
        Ok(true)
    }
    
    pub fn file_size(&self) -> u64 {
        self.file_size
    }
    
    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }
}
