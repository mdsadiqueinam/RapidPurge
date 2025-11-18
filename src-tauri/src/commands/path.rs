use crate::utils::{
    metadata::{file_allocated_bytes_and_id, FileId},
    path::is_hidden,
};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use walkdir::{DirEntry, WalkDir};

pub type Node = Arc<Mutex<PathInfo>>;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub struct PathInfo {
    pub name: String,
    pub path: String,
    pub size: u128,
    pub children: Vec<Node>,
    pub is_file: bool,
}

impl PathInfo {
    pub fn new(path: String, size: u128, is_file: bool) -> Node {
        let name = path
            .rsplit(std::path::MAIN_SEPARATOR)
            .next()
            .unwrap_or("")
            .to_string();
        Arc::new(Mutex::new(Self {
            name,
            path,
            size,
            children: Vec::new(),
            is_file,
        }))
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn calculate_size(&mut self) -> u128 {
        if self.is_file {
            return self.size;
        }

        let mut total_size: u128 = 0;
        for child in &self.children {
            let mut child_info = child.lock().unwrap();
            total_size = total_size.saturating_add(child_info.calculate_size());
        }
        self.size = total_size;
        total_size
    }
}

fn should_include_dir(entry: &DirEntry) -> bool {
    let is_visible = !is_hidden(entry);

    return is_visible;
}

pub async fn iterate_dir<F>(root: &str, mut on_dir: F) -> Result<Node, String>
where
    F: FnMut(&Node),
{
    let mut node_map: HashMap<String, Node> = HashMap::new();
    let mut nodes: Vec<Node> = Vec::new();
    let mut seen: HashSet<FileId> = HashSet::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| should_include_dir(e))
        .filter_map(|e| e.ok())
    // skip permission errors
    {
        let path = entry.path();
        if let Ok(metadata) = entry.metadata() {
            let is_file = metadata.is_file();

            let size = if is_file {
                let (alloc, fid_opt) = file_allocated_bytes_and_id(path, &metadata);
                if let Some(fid) = fid_opt {
                    if !seen.insert(fid) {
                        // already counted same inode/file id
                        continue;
                    }
                }
                alloc
            } else {
                0
            };
            let path_str = path.to_string_lossy().to_string();

            // create the Node
            let node: Node = PathInfo::new(path_str.clone(), size, is_file);

            // insert a clone into the map (cheap — increments refcount)
            node_map.insert(path_str.clone(), node.clone());
            nodes.push(node.clone());

            // call the callback with a reference to the Node
            on_dir(&node);

            // link to parent (use clones, do NOT move `node` here)
            if let Some(parent_path) = path.parent() {
                let parent_path_str = parent_path.to_string_lossy().to_string();
                if let Some(parent_node) = node_map.get_mut(&parent_path_str) {
                    // borrow the parent's inner PathInfo mutably and add a clone of node
                    parent_node.lock().unwrap().add_child(node.clone());
                }
            }
        }
    }

    for node in nodes.iter().rev() {
        let mut node_info = node.lock().unwrap();
        node_info.calculate_size();
    }

    return match node_map.get(root) {
        Some(root_node) => Ok(root_node.clone()),
        None => Err(format!("Root path {} not found in path map", root)),
    };
}
