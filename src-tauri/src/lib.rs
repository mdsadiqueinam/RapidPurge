mod commands;
mod domain;
mod utils;

use commands::flash_clean::flash_scan;
use commands::system_info::{get_system_info, open_access_panel};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            flash_scan,
            get_system_info,
            open_access_panel
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
