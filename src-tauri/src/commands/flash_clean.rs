use crate::{
    commands::path::{iterate_dir, Node, ScanEvent},
    utils::path::get_flash_scan_paths,
};
use tauri::{ipc::Channel, AppHandle};

#[tauri::command]
pub async fn flash_scan(_app: AppHandle, event: Channel<ScanEvent>) {
    let mut junk_size: u128 = 0;
    let mut root_nodes: Vec<Node> = Vec::new();

    for root in get_flash_scan_paths() {
        let root_node = iterate_dir(&root, |node| {
            let dir_info = node.lock().unwrap();

            if dir_info.is_file {
                junk_size = junk_size.saturating_add(dir_info.size);
            } else {
                let _ = event.send(ScanEvent::Progress {
                    junk_found: junk_size,
                    last_path_str: dir_info.path.clone(),
                });
            }
        })
        .await;

        if let Ok(_node) = root_node {
            root_nodes.push(_node);
        }
    }

    event
        .send(ScanEvent::Finished {
            junk_found: junk_size as u64,
            root_nodes,
        })
        .unwrap();
}
