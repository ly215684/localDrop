use crc32fast::Hasher;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{self, Read};

pub fn calculate_crc32(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

pub fn calculate_sha256_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 65536];
    
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

pub fn format_speed(bytes_per_second: u64) -> String {
    if bytes_per_second < 1024 {
        format!("{} B/s", bytes_per_second)
    } else if bytes_per_second < 1024 * 1024 {
        format!("{:.1} KB/s", bytes_per_second as f64 / 1024.0)
    } else {
        format!("{:.1} MB/s", bytes_per_second as f64 / (1024.0 * 1024.0))
    }
}

pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

pub fn format_eta(seconds: u64) -> String {
    if seconds == 0 {
        return "--".to_string();
    }
    
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    
    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}:{:02}", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}
