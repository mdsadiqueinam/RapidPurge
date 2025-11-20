use std::cmp::Reverse;
use std::collections::HashMap;

use crate::domain::flash_scan::path::FLASH_SCAN_CATEGORIES;
use crate::domain::models::flash_scan::FlashScanCategory;
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
        walk_and_build_tree(root, &mut node_map, false, |node| {
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
    pub sub_categories: Option<Vec<CategorisedFlashScan>>,
}

pub fn categorise_scanned_paths(node_map: &HashMap<String, Node>) -> Vec<CategorisedFlashScan> {
    let mut cats: Vec<FlashScanCategory> = FLASH_SCAN_CATEGORIES
        .iter()
        .flat_map(|cat| {
            cat.sub_categories
                .as_ref()
                .cloned()
                .unwrap_or_else(|| vec![cat.clone()])
                .into_iter()
        })
        .collect();

    // Sort in descending order of priority
    cats.sort_by_key(|c| Reverse(c.priority));

    cats.into_iter()
        .map(|cat| build_category(&cat, node_map))
        .collect()
}

fn build_category(
    category: &crate::domain::models::flash_scan::FlashScanCategory,
    node_map: &HashMap<String, Node>,
) -> CategorisedFlashScan {
    let mut nodes: Vec<Node> = Vec::new();
    let regex_set = category.regexp.as_ref();
    let mut node_paths: Vec<&String> = node_map.keys().collect();
    node_paths.sort();

    // Single pass over all nodes: match by prefix (paths) and regex (if any)
    for node_path in node_paths.iter() {
        let node_opt = node_map.get(*node_path);
        if node_opt.is_none() {
            continue;
        }
        let node = node_opt.unwrap();
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

    CategorisedFlashScan {
        category_id: category.id.clone(),
        nodes,
        sub_categories: None,
    }
}

fn push_node_and_children(
    node: &Node,
    nodes: &mut Vec<Node>,
    node_map: &mut HashMap<String, Node>,
) {
    nodes.push(node.clone());
    remove_node(node, node_map);
    let node_info = node.lock().unwrap();
    for child in &node_info.children {
        push_node_and_children(child, nodes, node_map);
    }
}

fn remove_node(node: &Node, node_map: &mut HashMap<String, Node>) {
    let key = node.lock().unwrap().path.clone();
    node_map.remove(&key);
}
