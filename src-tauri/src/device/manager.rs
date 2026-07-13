use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use super::discovery::{DeviceDiscovery, DeviceInfo};
use crate::utils::config::AppConfig;

pub struct DeviceManager {
    discovery: Mutex<Option<DeviceDiscovery>>,
    config: Arc<Mutex<AppConfig>>,
    runtime: Mutex<Option<Runtime>>,
}

impl DeviceManager {
    pub fn new(config: Arc<Mutex<AppConfig>>) -> Self {
        Self {
            discovery: Mutex::new(None),
            config,
            runtime: Mutex::new(None),
        }
    }
    
    pub fn start_discovery(&self) -> std::io::Result<()> {
        let config = self.config.lock().unwrap();
        
        let mut discovery = DeviceDiscovery::new(
            config.broadcast_port,
            config.tcp_port,
            config.device_id.clone(),
            config.device_name.clone(),
            config.device_type.clone(),
        );
        
        let runtime = tokio::runtime::Runtime::new()?;
        runtime.block_on(discovery.start())?;
        
        *self.runtime.lock().unwrap() = Some(runtime);
        *self.discovery.lock().unwrap() = Some(discovery);
        
        Ok(())
    }
    
    pub fn stop_discovery(&self) {
        if let Some(ref mut discovery) = *self.discovery.lock().unwrap() {
            if let Some(ref mut runtime) = *self.runtime.lock().unwrap() {
                runtime.block_on(discovery.stop());
            }
        }
    }
    
    pub fn get_devices(&self) -> Vec<DeviceInfo> {
        if let Some(ref discovery) = *self.discovery.lock().unwrap() {
            discovery.get_devices()
        } else {
            Vec::new()
        }
    }
    
    pub fn update_device_name(&self, new_name: String) {
        let mut config = self.config.lock().unwrap();
        config.device_name = new_name.clone();
        
        if let Some(ref mut discovery) = *self.discovery.lock().unwrap() {
            discovery.update_device_name(new_name);
        }
        
        let _ = crate::utils::config::save_config(&config);
    }
}
