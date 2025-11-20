use std::cmp::Reverse;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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

type CategorisedNode = Arc<Mutex<CategorisedFlashScan>>;

#[derive(Clone, Debug)]
pub struct CategorisedFlashScan {
    pub category_id: String,
    pub nodes: Vec<Node>,
    pub sub_categories: Option<Vec<CategorisedNode>>,
}

impl CategorisedFlashScan {
    fn new_from(
        category_id: String,
        nodes: Vec<Node>,
        sub_categories: Option<Vec<CategorisedNode>>,
    ) -> CategorisedNode {
        Arc::new(Mutex::new(Self {
            category_id,
            nodes,
            sub_categories,
        }))
    }
}

pub fn categorise_scanned_paths(node_map: &mut HashMap<String, Node>) -> Vec<CategorisedNode> {
    let mut cats: Vec<&FlashScanCategory> = Vec::new();
    for cat in FLASH_SCAN_CATEGORIES.iter() {
        if let Some(subs) = &cat.sub_categories {
            cats.extend(subs.iter());
        } else {
            cats.push(cat);
        }
    }

    // Sort in descending order of priority
    cats.sort_by_key(|c| Reverse(c.priority));

    let scanned_categories: Vec<CategorisedNode> = cats
        .into_iter()
        .map(|cat| build_category(cat, node_map))
        .collect();

    let mut cat_map: HashMap<String, CategorisedNode> = HashMap::new();
    for category in scanned_categories.iter() {
        let category_id = category.lock().unwrap().category_id.clone();
        cat_map.insert(category_id, category.clone());
    }

    let mut result: Vec<CategorisedNode> = Vec::new();
    for category in FLASH_SCAN_CATEGORIES.iter() {
        if !cat_map.contains_key(&category.id) {
            continue;
        }

        let tree = build_category_tree(category, &cat_map);
        result.push(tree);
    }

    return result;
}

fn build_category_tree(
    category: &FlashScanCategory,
    cat_map: &HashMap<String, CategorisedNode>,
) -> CategorisedNode {
    let cloned_category = cat_map
        .get(&category.id)
        .cloned()
        .unwrap_or_else(|| CategorisedFlashScan::new_from(category.id.clone(), Vec::new(), None));

    if let Some(sub_categories) = &category.sub_categories {
        let mut sub_results: Vec<CategorisedNode> = Vec::new();
        for sub_category in sub_categories.iter() {
            if !cat_map.contains_key(&sub_category.id) {
                continue;
            }

            let sub_tree = build_category_tree(sub_category, cat_map);
            sub_results.push(sub_tree);
        }

        if !sub_results.is_empty() {
            cloned_category.lock().unwrap().sub_categories = Some(sub_results);
        }
    }

    cloned_category
}

fn build_category(
    category: &FlashScanCategory,
    node_map: &mut HashMap<String, Node>,
) -> CategorisedNode {
    let mut nodes: Vec<Node> = Vec::new();
    let regex_set = category.regexp.as_ref();
    let mut node_paths: Vec<String> = node_map.keys().cloned().collect();
    node_paths.sort();

    for node_path in node_paths {
        let matches_prefix = category
            .paths
            .iter()
            .any(|prefix| node_path.starts_with(prefix));
        let matches_regex =
            !matches_prefix && regex_set.map(|re| re.is_match(&node_path)).unwrap_or(false);

        if matches_prefix || matches_regex {
            if let Some(root) = node_map.remove(&node_path) {
                drain_subtree_into(root, &mut nodes, node_map);
            }
        }
    }

    CategorisedFlashScan::new_from(category.id.clone(), nodes, None)
}

fn drain_subtree_into(root: Node, bucket: &mut Vec<Node>, node_map: &mut HashMap<String, Node>) {
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        let (path, children) = {
            let info = node.lock().unwrap();
            (info.path.clone(), info.children.clone())
        };

        bucket.push(node.clone());
        unlink_node_path(&path, node_map);

        // Preserve child order by pushing them in reverse for the DFS stack.
        stack.extend(children.into_iter().rev());
    }
}

fn unlink_node_path(path: &str, node_map: &mut HashMap<String, Node>) {
    node_map.remove(path);
}
