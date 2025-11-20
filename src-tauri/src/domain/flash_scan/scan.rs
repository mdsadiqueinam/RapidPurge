use std::cmp::Reverse;
use std::collections::HashMap;
use std::hash::Hash;

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

pub fn categorise_scanned_paths(node_map: &mut HashMap<String, Node>) -> Vec<CategorisedFlashScan> {
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

    let scanned_categories = cats
        .into_iter()
        .map(|cat| build_category(cat, node_map))
        .collect();

    return scanned_categories;
}

fn build_category_tree(categorised: &Vec<CategorisedFlashScan>) -> Vec<CategorisedFlashScan> {
    let mut result: Vec<CategorisedFlashScan> = Vec::new();
    let mut cat_map: HashMap<String, &CategorisedFlashScan> = HashMap::new();

    for cat in categorised.iter() {
        cat_map.insert(cat.category_id.clone(), &cat);
    }

    for cat in FLASH_SCAN_CATEGORIES.iter() {
        let category_id = &cat.id;
        let scanned_cat = match cat_map.get(category_id) {
            Some(c) => (*c).clone(),
            None => continue,
        };

        result.push(scanned_cat);

        if let Some(sub_cats) = &cat.sub_categories {
            let mut scanned_sub_cats: Vec<CategorisedFlashScan> = Vec::new();
            for sub_cat in sub_cats.iter() {
                if let Some(scanned_sub_cat) = cat_map.get(&sub_cat.id) {
                    scanned_sub_cats.push((*scanned_sub_cat).clone());
                }
            }

            if let Some(last) = result.last_mut() {
                last.sub_categories = Some(scanned_sub_cats);
            }
        }
    }

    return result;
}

fn build_category(
    category: &FlashScanCategory,
    node_map: &mut HashMap<String, Node>,
) -> CategorisedFlashScan {
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

    CategorisedFlashScan {
        category_id: category.id.clone(),
        nodes,
        sub_categories: None,
    }
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
