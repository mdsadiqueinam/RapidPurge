use std::collections::HashMap;

use crate::domain::models::path::Node;
use crate::domain::walker::walk_and_build_tree;

pub async fn perform_flash_scan<F>(paths: &[String], mut on_event: F) -> (u64, Vec<Node>)
where
    F: FnMut(&u128, &str) -> (),
{
    let mut total_junk_bytes: u128 = 0;
    let mut node_map: HashMap<String, Node> = HashMap::new();
    let mut root_nodes: Vec<Node> = Vec::new();

    for root in paths {
        walk_and_build_tree(root, &mut node_map, |node| {
            let node_info = node.lock().unwrap();

            if node_info.is_file {
                total_junk_bytes = total_junk_bytes.saturating_add(node_info.size);
            } else {
                on_event(&total_junk_bytes, &node_info.path);
            }
        })
        .await;

        if let Some(root_node) = node_map.get(root) {
            root_nodes.push(root_node.clone());
        }
    }

    (total_junk_bytes as u64, root_nodes)
}
