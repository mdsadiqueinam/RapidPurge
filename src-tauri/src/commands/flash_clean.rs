use crate::{
    domain::flash_scan::{path::default_flash_scan_paths, scan::perform_flash_scan},
    domain::models::event::ScanEvent,
};
use tauri::{ipc::Channel, AppHandle};

#[tauri::command]
pub async fn flash_scan(_app: AppHandle, event: Channel<ScanEvent>) {
    let scan_paths = default_flash_scan_paths();
    let (total_junk_bytes, root_nodes) =
        perform_flash_scan(&scan_paths, |total_junk_bytes, path_str| {
            event
                .send(ScanEvent::Progress {
                    junk_found: *total_junk_bytes,
                    current_path_str: path_str.to_string(),
                })
                .unwrap();
        })
        .await;

    event
        .send(ScanEvent::Finished {
            junk_found: total_junk_bytes,
            root_nodes,
        })
        .unwrap();
}
