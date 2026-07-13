pub mod commands;
pub mod device;
pub mod transfer;
pub mod protocol;
pub mod file;
pub mod persistence;
pub mod utils;

use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use tauri::Manager;
use crate::device::manager::DeviceManager;
use crate::transfer::manager::TransferManager;
use crate::utils::config::load_config;
use crate::persistence::sqlite::Database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let config = Arc::new(Mutex::new(load_config().unwrap_or_default()));
            let database = Arc::new(Mutex::new(Database::new().unwrap()));
            let device_manager = Arc::new(Mutex::new(DeviceManager::new(Arc::clone(&config))));
            let transfer_manager = Arc::new(RwLock::new(TransferManager::new(Arc::clone(&database), Arc::clone(&config))));

            app.manage(config);
            app.manage(database);
            app.manage(device_manager);
            app.manage(transfer_manager);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_discovery,
            commands::stop_discovery,
            commands::get_devices,
            commands::send_files,
            commands::accept_transfer,
            commands::cancel_transfer,
            commands::pause_transfer,
            commands::resume_transfer,
            commands::get_settings,
            commands::save_settings,
            commands::rename_device,
            commands::get_my_device_info,
            commands::open_file,
            commands::open_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
