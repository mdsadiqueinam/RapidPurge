use std::collections::HashMap;

use crate::{
    domain::flash_scan::model::Node, domain::walker::walk_and_build_tree,
    utils::path::default_flash_scan_paths,
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
    let scan_paths = default_flash_scan_paths();
    let (total_junk_bytes, root_nodes) = perform_flash_scan(&scan_paths, &event).await;

    event
        .send(ScanEvent::Finished {
            junk_found: total_junk_bytes,
            root_nodes,
        })
        .unwrap();
}

async fn perform_flash_scan(paths: &[String], event: &Channel<ScanEvent>) -> (u64, Vec<Node>) {
    let mut total_junk_bytes: u128 = 0;
    let mut node_map: HashMap<String, Node> = HashMap::new();
    let mut root_nodes: Vec<Node> = Vec::new();

    for root in paths {
        walk_and_build_tree(root, &mut node_map, |node| {
            let dir_info = node.lock().unwrap();

            if dir_info.is_file {
                total_junk_bytes = total_junk_bytes.saturating_add(dir_info.size);
            } else {
                let _ = event.send(ScanEvent::Progress {
                    junk_found: total_junk_bytes,
                    current_path_str: dir_info.path.clone(),
                });
            }
        })
        .await;

        if let Some(root_node) = node_map.get(root) {
            root_nodes.push(root_node.clone());
        }
    }

    (total_junk_bytes as u64, root_nodes)
}
