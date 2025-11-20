use std::collections::HashMap;

use crate::domain::category::{categorise_scanned_paths, CategorisedNode};
use crate::domain::models::path::Node;
use crate::domain::walker::walk_and_build_tree;

pub async fn perform_flash_scan<F>(paths: &[String], mut on_event: F) -> (u64, Vec<CategorisedNode>)
where
    F: FnMut(&u128, &str) -> (),
{
    let mut total_junk_bytes: u128 = 0;
    let mut node_map: HashMap<String, Node> = HashMap::new();

    for root in paths {
        walk_and_build_tree(root, &mut node_map, false, |node| {
            let node_info = node.lock().unwrap();

            if node_info.is_file {
                total_junk_bytes = total_junk_bytes.saturating_add(node_info.size);
            } else {
                on_event(&total_junk_bytes, &node_info.path);
            }
        })
        .await;
    }

    // Clear file children from all nodes to keep only folders
    node_map
        .values()
        .for_each(|n| n.lock().unwrap().clear_file_children());

    // remove file nodes from the map
    node_map.retain(|_, node| {
        let info = node.lock().unwrap();
        !info.is_file
    });

    let categorised = categorise_scanned_paths(&mut node_map);

    (total_junk_bytes as u64, categorised)
}
