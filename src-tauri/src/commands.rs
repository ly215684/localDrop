use tauri::{State, command};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use crate::device::manager::DeviceManager;
use crate::transfer::manager::TransferManager;
use crate::utils::config::{AppConfig, save_config};

#[derive(Serialize)]
pub struct DeviceInfo {
    device_id: String,
    device_name: String,
    device_type: String,
    ip_address: String,
    tcp_port: u16,
    online: bool,
}

#[derive(Serialize)]
pub struct MyDeviceInfo {
    device_id: String,
    device_name: String,
    device_type: String,
}

#[command]
pub fn start_discovery(device_manager: State<'_, Arc<Mutex<DeviceManager>>>) -> Result<(), String> {
    device_manager.lock().unwrap().start_discovery().map_err(|e| e.to_string())
}

#[command]
pub fn stop_discovery(device_manager: State<'_, Arc<Mutex<DeviceManager>>>) {
    device_manager.lock().unwrap().stop_discovery();
}

#[command]
pub fn get_devices(device_manager: State<'_, Arc<Mutex<DeviceManager>>>) -> Vec<DeviceInfo> {
    device_manager.lock().unwrap().get_devices().into_iter().map(|d| DeviceInfo {
        device_id: d.device_id,
        device_name: d.device_name,
        device_type: d.device_type,
        ip_address: d.ip_address,
        tcp_port: d.tcp_port,
        online: d.online,
    }).collect()
}

#[command]
pub async fn send_files(
    device_manager: State<'_, Arc<Mutex<DeviceManager>>>,
    transfer_manager: State<'_, Arc<RwLock<TransferManager>>>,
    device_id: String,
    file_paths: Vec<String>,
) -> Result<String, String> {
    let devices = device_manager.lock().unwrap().get_devices();
    
    if let Some(device) = devices.into_iter().find(|d| d.device_id == device_id) {
        transfer_manager.write().await.send_files(
            device.ip_address,
            device.tcp_port,
            file_paths,
            device.device_id,
            device.device_name,
        ).await.map_err(|e| e.to_string())
    } else {
        Err("Device not found".to_string())
    }
}

#[command]
pub async fn accept_transfer(
    transfer_manager: State<'_, Arc<RwLock<TransferManager>>>,
    transfer_id: String,
) -> Result<(), String> {
    transfer_manager.write().await.accept_transfer(transfer_id).await.map_err(|e| e.to_string())
}

#[command]
pub async fn cancel_transfer(
    transfer_manager: State<'_, Arc<RwLock<TransferManager>>>,
    transfer_id: String,
) -> Result<(), String> {
    transfer_manager.write().await.cancel_transfer(transfer_id).await.map_err(|e| e.to_string())
}

#[command]
pub async fn pause_transfer(
    transfer_manager: State<'_, Arc<RwLock<TransferManager>>>,
    transfer_id: String,
) -> Result<(), String> {
    transfer_manager.write().await.pause_transfer(transfer_id).await.map_err(|e| e.to_string())
}

#[command]
pub async fn resume_transfer(
    transfer_manager: State<'_, Arc<RwLock<TransferManager>>>,
    transfer_id: String,
) -> Result<(), String> {
    transfer_manager.write().await.resume_transfer(transfer_id).await.map_err(|e| e.to_string())
}

#[command]
pub fn get_settings(config: State<'_, Arc<Mutex<AppConfig>>>) -> AppConfig {
    config.lock().unwrap().clone()
}

#[command]
pub fn save_settings(config: State<'_, Arc<Mutex<AppConfig>>>, settings: AppConfig) -> Result<(), String> {
    let mut config_mut = config.lock().unwrap();
    *config_mut = settings.clone();
    save_config(&settings).map_err(|e| e.to_string())
}

#[command]
pub fn rename_device(device_manager: State<'_, Arc<Mutex<DeviceManager>>>, new_name: String) {
    device_manager.lock().unwrap().update_device_name(new_name);
}

#[command]
pub fn get_my_device_info(config: State<'_, Arc<Mutex<AppConfig>>>) -> MyDeviceInfo {
    let config = config.lock().unwrap();
    MyDeviceInfo {
        device_id: config.device_id.clone(),
        device_name: config.device_name.clone(),
        device_type: config.device_type.clone(),
    }
}

#[command]
pub fn open_file(file_path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .args(["/c", "start", "", &file_path])
        .spawn()
        .map_err(|e| e.to_string())?;
    
    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(&file_path)
        .spawn()
        .map_err(|e| e.to_string())?;
    
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(&file_path)
        .spawn()
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[command]
pub fn open_folder(folder_path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .args(["/c", "start", "", &folder_path])
        .spawn()
        .map_err(|e| e.to_string())?;
    
    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(&folder_path)
        .spawn()
        .map_err(|e| e.to_string())?;
    
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(&folder_path)
        .spawn()
        .map_err(|e| e.to_string())?;
    
    Ok(())
}
