use std::collections::HashMap;

use crate::domain::flash_scan::path::FLASH_SCAN_CATEGORIES;
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

#[derive(Clone, Debug)]
pub struct CategorisedFlashScan {
    pub category_id: String,
    pub nodes: Vec<Node>,
    pub sub_categories: Vec<CategorisedFlashScan>,
}

pub fn categorise_scanned_paths(node_map: &HashMap<String, Node>) -> Vec<CategorisedFlashScan> {
    FLASH_SCAN_CATEGORIES
        .iter()
        .map(|cat| build_category(cat, node_map))
        .collect()
}

fn build_category(
    category: &crate::domain::models::flash_scan::FlashScanCategory,
    node_map: &HashMap<String, Node>,
) -> CategorisedFlashScan {
    let mut nodes: Vec<Node> = Vec::new();
    let regex_set = category.regexp.as_ref();

    // Single pass over all nodes: match by prefix (paths) and regex (if any)
    for (node_path, node) in node_map.iter() {
        let mut matched = false;

        // path prefix check
        for prefix in &category.paths {
            if node_path.starts_with(prefix) {
                matched = true;
                break;
            }
        }

        // regex check
        if !matched {
            if let Some(re) = regex_set {
                if re.is_match(node_path) {
                    matched = true;
                }
            }
        }

        if matched {
            nodes.push(node.clone());
        }
    }

    let sub_categories = category
        .sub_categories
        .as_ref()
        .map(|subs| {
            subs.iter()
                .map(|sub| build_category(sub, node_map))
                .filter(|c| !c.nodes.is_empty() || !c.sub_categories.is_empty())
                .collect()
        })
        .unwrap_or_default();

    CategorisedFlashScan {
        category_id: category.id.clone(),
        nodes,
        sub_categories,
    }
}
