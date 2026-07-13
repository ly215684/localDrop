use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::utils::hash::calculate_crc32;

pub struct FileSplitter {
    file: File,
    chunk_size: usize,
    file_size: u64,
    current_pos: u64,
}

impl FileSplitter {
    pub fn new(file_path: &str, chunk_size: usize) -> io::Result<Self> {
        let mut file = File::open(file_path)?;
        let file_size = file.metadata()?.len();
        Ok(Self {
            file,
            chunk_size,
            file_size,
            current_pos: 0,
        })
    }
    
    pub fn file_size(&self) -> u64 {
        self.file_size
    }
    
    pub fn total_chunks(&self) -> i64 {
        ((self.file_size + self.chunk_size as u64 - 1) / self.chunk_size as u64) as i64
    }
    
    pub fn read_chunk(&mut self, chunk_index: i64) -> io::Result<(Vec<u8>, u32)> {
        let offset = chunk_index as u64 * self.chunk_size as u64;
        
        if offset >= self.file_size {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Chunk index out of range"));
        }
        
        self.file.seek(SeekFrom::Start(offset))?;
        
        let remaining = (self.file_size - offset) as usize;
        let bytes_to_read = std::cmp::min(self.chunk_size, remaining);
        
        let mut buffer = vec![0u8; bytes_to_read];
        let bytes_read = self.file.read(&mut buffer)?;
        
        buffer.truncate(bytes_read);
        let crc32 = calculate_crc32(&buffer);
        
        Ok((buffer, crc32))
    }
    
    pub fn read_chunk_raw(&mut self, chunk_index: i64) -> io::Result<Vec<u8>> {
        let (data, _) = self.read_chunk(chunk_index)?;
        Ok(data)
    }
}
