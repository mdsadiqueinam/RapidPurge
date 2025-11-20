use std::cmp::Reverse;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use regex::RegexSet;
use serde::Serialize;

use crate::domain::flash_scan::path::FLASH_SCAN_CATEGORIES;
use crate::domain::models::path::Node;

#[derive(Clone, Debug)]
pub struct ScanCategory {
    pub id: String,
    #[allow(unused)]
    pub name: String,
    pub priority: u32,
    pub paths: Vec<String>,
    pub regexp: Option<RegexSet>,
    pub sub_categories: Option<Vec<ScanCategory>>,
}

pub type CategorisedNode = Arc<Mutex<CategorisedScan>>;

#[derive(Clone, Serialize, Debug)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub enum CategoryContents {
    Nodes(Vec<Node>),
    SubCategories(Vec<CategorisedNode>),
}

#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorisedScan {
    pub category_id: String,
    pub nodes: CategoryContents,
}

impl CategorisedScan {
    fn new_from(category_id: String, nodes: CategoryContents) -> CategorisedNode {
        Arc::new(Mutex::new(Self { category_id, nodes }))
    }

    fn new_node(category_id: String, nodes: Vec<Node>) -> CategorisedNode {
        CategorisedScan::new_from(category_id, CategoryContents::Nodes(nodes))
    }

    fn new_subcategories(
        category_id: String,
        sub_categories: Vec<CategorisedNode>,
    ) -> CategorisedNode {
        CategorisedScan::new_from(category_id, CategoryContents::SubCategories(sub_categories))
    }
}

pub fn categorise_scanned_paths(node_map: &mut HashMap<String, Node>) -> Vec<CategorisedNode> {
    let mut cats: Vec<&ScanCategory> = Vec::new();
    for cat in FLASH_SCAN_CATEGORIES.iter() {
        if let Some(subs) = &cat.sub_categories {
            cats.extend(subs.iter());
        } else {
            cats.push(cat);
        }
    }

    // Sort in descending order of priority
    cats.sort_by_key(|c| Reverse(c.priority));

    let categorised_nodes: Vec<CategorisedNode> = cats
        .into_iter()
        .map(|cat| build_category(cat, node_map))
        .collect();

    let mut cat_map: HashMap<String, CategorisedNode> = HashMap::new();
    for category in categorised_nodes.iter() {
        let category_id = category.lock().unwrap().category_id.clone();
        cat_map.insert(category_id, category.clone());
    }

    let mut result: Vec<CategorisedNode> = Vec::new();
    for category in FLASH_SCAN_CATEGORIES.iter() {
        let tree = build_category_tree(category, &cat_map);
        if category_has_content(&tree) {
            result.push(tree);
        }
    }

    return result;
}

fn build_category_tree(
    category: &ScanCategory,
    cat_map: &HashMap<String, CategorisedNode>,
) -> CategorisedNode {
    if let Some(sub_categories) = &category.sub_categories {
        let mut sub_results: Vec<CategorisedNode> = Vec::new();
        for sub_category in sub_categories.iter() {
            let sub_tree = build_category_tree(sub_category, cat_map);
            if category_has_content(&sub_tree) {
                sub_results.push(sub_tree);
            }
        }

        if !sub_results.is_empty() {
            return CategorisedScan::new_subcategories(category.id.clone(), sub_results);
        }
    }

    cat_map
        .get(&category.id)
        .cloned()
        .unwrap_or_else(|| CategorisedScan::new_node(category.id.clone(), Vec::new()))
}

fn build_category(
    category: &ScanCategory,
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

    CategorisedScan::new_node(category.id.clone(), nodes)
}

fn category_has_content(node: &CategorisedNode) -> bool {
    let snapshot = {
        let guard = node.lock().unwrap();
        match &guard.nodes {
            CategoryContents::Nodes(nodes) => return !nodes.is_empty(),
            CategoryContents::SubCategories(children) => children.clone(),
        }
    };

    snapshot
        .into_iter()
        .any(|child| category_has_content(&child))
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
