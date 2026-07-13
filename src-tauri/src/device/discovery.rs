use tokio::net::{UdpSocket, TcpListener};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub ip_address: String,
    pub tcp_port: u16,
    pub online: bool,
    pub last_seen: u64,
}

pub struct DeviceDiscovery {
    socket: Option<Arc<UdpSocket>>,
    tcp_listener: Option<TcpListener>,
    broadcast_port: u16,
    tcp_port: u16,
    device_id: String,
    device_name: String,
    device_type: String,
    devices: Arc<Mutex<HashMap<String, DeviceInfo>>>,
    running: Arc<Mutex<bool>>,
}

impl DeviceDiscovery {
    pub fn new(
        broadcast_port: u16,
        tcp_port: u16,
        device_id: String,
        device_name: String,
        device_type: String,
    ) -> Self {
        Self {
            socket: None,
            tcp_listener: None,
            broadcast_port,
            tcp_port,
            device_id,
            device_name,
            device_type,
            devices: Arc::new(Mutex::new(HashMap::new())),
            running: Arc::new(Mutex::new(false)),
        }
    }
    
    pub async fn start(&mut self) -> std::io::Result<()> {
        *self.running.lock().unwrap() = true;
        
        let socket = Arc::new(UdpSocket::bind(format!("0.0.0.0:{}", self.broadcast_port)).await?);
        socket.set_broadcast(true)?;
        self.socket = Some(Arc::clone(&socket));
        
        self.tcp_listener = Some(TcpListener::bind(format!("0.0.0.0:{}", self.tcp_port)).await?);
        
        let devices_clone = Arc::clone(&self.devices);
        let running_clone = Arc::clone(&self.running);
        let socket_clone = Arc::clone(&socket);
        
        tokio::spawn(async move {
            Self::receive_loop(socket_clone, devices_clone, running_clone).await;
        });
        
        let socket_clone = Arc::clone(&socket);
        let device_id = self.device_id.clone();
        let device_name = self.device_name.clone();
        let device_type = self.device_type.clone();
        let tcp_port = self.tcp_port;
        let broadcast_port = self.broadcast_port;
        let running_clone = Arc::clone(&self.running);
        
        tokio::spawn(async move {
            Self::broadcast_loop(socket_clone, broadcast_port, device_id, device_name, device_type, tcp_port, running_clone).await;
        });
        
        Ok(())
    }
    
    pub async fn stop(&mut self) {
        *self.running.lock().unwrap() = false;
        
        self.socket = None;
        self.tcp_listener = None;
        
        self.devices.lock().unwrap().clear();
    }
    
    pub fn get_devices(&self) -> Vec<DeviceInfo> {
        let mut devices = self.devices.lock().unwrap();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        devices.retain(|_, d| now - d.last_seen < 10);
        
        devices.values().cloned().collect()
    }
    
    async fn receive_loop(
        socket: Arc<UdpSocket>,
        devices: Arc<Mutex<HashMap<String, DeviceInfo>>>,
        running: Arc<Mutex<bool>>,
    ) {
        let mut buf = [0u8; 1024];
        
        while *running.lock().unwrap() {
            match socket.recv_from(&mut buf).await {
                Ok((len, addr)) => {
                    let data = &buf[..len];
                    if let Ok(msg) = serde_json::from_slice::<serde_json::Value>(data) {
                        if let (Some(device_id), Some(device_name), Some(device_type), Some(tcp_port)) = (
                            msg.get("device_id").and_then(|v| v.as_str()),
                            msg.get("device_name").and_then(|v| v.as_str()),
                            msg.get("device_type").and_then(|v| v.as_str()),
                            msg.get("tcp_port").and_then(|v| v.as_u64()),
                        ) {
                            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                            
                            devices.lock().unwrap().insert(
                                device_id.to_string(),
                                DeviceInfo {
                                    device_id: device_id.to_string(),
                                    device_name: device_name.to_string(),
                                    device_type: device_type.to_string(),
                                    ip_address: addr.ip().to_string(),
                                    tcp_port: tcp_port as u16,
                                    online: true,
                                    last_seen: now,
                                },
                            );
                        }
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
    
    async fn broadcast_loop(
        socket: Arc<UdpSocket>,
        broadcast_port: u16,
        device_id: String,
        device_name: String,
        device_type: String,
        tcp_port: u16,
        running: Arc<Mutex<bool>>,
    ) {
        let message = json!({
            "device_id": device_id,
            "device_name": device_name,
            "device_type": device_type,
            "tcp_port": tcp_port,
            "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        });
        
        let message_bytes = serde_json::to_vec(&message).unwrap();
        
        while *running.lock().unwrap() {
            let _ = socket.send_to(&message_bytes, format!("255.255.255.255:{}", broadcast_port)).await;
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }
    
    pub fn update_device_name(&mut self, new_name: String) {
        self.device_name = new_name;
    }
}