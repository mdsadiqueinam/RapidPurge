use std::collections::HashMap;

use crate::{
    commands::path::{iterate_dir, Node},
    utils::path::get_flash_scan_paths,
};
use serde::Serialize;
use tauri::{ipc::Channel, AppHandle};

#[derive(Clone, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub enum ScanEvent {
    Progress {
        junk_found: u128,
        current_path_str: String,
    },
    Finished {
        junk_found: u64,
        root_nodes: Vec<Node>,
    },
}

#[tauri::command]
pub async fn flash_scan(_app: AppHandle, event: Channel<ScanEvent>) {
    let mut junk_size: u128 = 0;
    let mut node_map: HashMap<String, Node> = HashMap::new();
    let mut root_nodes: Vec<Node> = Vec::new();

    for root in get_flash_scan_paths() {
        iterate_dir(&root, &mut node_map, |node| {
            let dir_info = node.lock().unwrap();

            if dir_info.is_file {
                junk_size = junk_size.saturating_add(dir_info.size);
            } else {
                let _ = event.send(ScanEvent::Progress {
                    junk_found: junk_size,
                    current_path_str: dir_info.path.clone(),
                });
            }
        })
        .await;

        if let Some(root_node) = node_map.get(&root) {
            root_nodes.push(root_node.clone());
        }
    }

    event
        .send(ScanEvent::Finished {
            junk_found: junk_size as u64,
            root_nodes,
        })
        .unwrap();
}
