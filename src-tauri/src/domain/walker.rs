use crate::domain::flash_scan::model::{Node, PathInfo};
use crate::utils::{
	metadata::{allocated_size_and_file_id, FileId},
	path::is_hidden_entry,
};
use std::collections::{HashMap, HashSet};
use walkdir::{DirEntry, WalkDir};

fn should_visit_entry(entry: &DirEntry) -> bool {
	!is_hidden_entry(entry)
}

pub async fn walk_and_build_tree<F>(
	root: &str,
	node_map: &mut HashMap<String, Node>,
	mut on_node: F,
) where
	F: FnMut(&Node),
{
	let mut nodes: Vec<Node> = Vec::new();
	let mut seen: HashSet<FileId> = HashSet::new();

	for entry in WalkDir::new(root)
		.into_iter()
		.filter_entry(|e| should_visit_entry(e))
		.filter_map(|e| e.ok())
	{
		let path = entry.path();
		if let Ok(metadata) = entry.metadata() {
			let is_file = metadata.is_file();

			let size = if is_file {
				let (alloc, fid_opt) = allocated_size_and_file_id(path, &metadata);
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

			let node: Node = PathInfo::from_path(path_str.clone(), size, is_file);

			node_map.insert(path_str.clone(), node.clone());
			nodes.push(node.clone());

			on_node(&node);

			if let Some(parent_path) = path.parent() {
				let parent_path_str = parent_path.to_string_lossy().to_string();
				if let Some(parent_node) = node_map.get_mut(&parent_path_str) {
					parent_node.lock().unwrap().add_child(node.clone());
				}
			}
		}
	}

	for node in nodes.iter().rev() {
		let mut node_info = node.lock().unwrap();
		node_info.calculate_size();
	}
}


