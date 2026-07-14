use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub chunk_size: usize,
    pub max_connections: usize,
    pub broadcast_port: u16,
    pub tcp_port: u16,
    pub save_path: String,
    pub auto_accept: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            device_id: uuid::Uuid::new_v4().to_string(),
            device_name: format!("LocalDrop-{}", uuid::Uuid::new_v4().to_string().split('-').next().unwrap()),
            device_type: get_device_type(),
            chunk_size: 64 * 1024,
            max_connections: 4,
            broadcast_port: 50000,
            tcp_port: 50001,
            save_path: String::new(),
            auto_accept: false,
        }
    }
}

fn get_device_type() -> String {
    #[cfg(target_os = "windows")]
    return "windows".to_string();
    #[cfg(target_os = "macos")]
    return "macos".to_string();
    #[cfg(target_os = "android")]
    return "android".to_string();
    #[cfg(target_os = "ios")]
    return "ios".to_string();
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "android", target_os = "ios")))]
    return "unknown".to_string();
}

pub fn get_config_dir() -> io::Result<std::path::PathBuf> {
    let dir = dirs::data_dir()
        .or_else(|| {
            #[cfg(target_os = "android")]
            {
                let mut home = std::env::var("HOME").ok().map(std::path::PathBuf::from);
                if let Some(ref mut h) = home {
                    h.push(".local");
                    h.push("share");
                }
                home
            }
            #[cfg(not(target_os = "android"))]
            None
        })
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Cannot find data directory"))?
        .join("localdrop");
    
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn get_config_path() -> io::Result<std::path::PathBuf> {
    Ok(get_config_dir()?.join("config.json"))
}

pub fn load_config() -> io::Result<AppConfig> {
    let path = get_config_path()?;
    
    if path.exists() {
        let content = fs::read_to_string(&path)?;
        serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    } else {
        let config = AppConfig::default();
        save_config(&config)?;
        Ok(config)
    }
}

pub fn save_config(config: &AppConfig) -> io::Result<()> {
    let path = get_config_path()?;
    let content = serde_json::to_string_pretty(config)?;
    
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
