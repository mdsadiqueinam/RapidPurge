use crate::utils::{file_allocated_bytes_and_id, get_roots, is_hidden, FileId};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use tauri::{ipc::Channel, AppHandle};
use walkdir::{DirEntry, WalkDir};

type Node = Rc<RefCell<PathInfo>>;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub struct PathInfo {
    pub path: String,
    pub size: u128,
    pub children: Vec<Node>,
    pub is_file: bool,
}

impl PathInfo {
    pub fn new(path: String, size: u128, is_file: bool) -> Node {
        Rc::new(RefCell::new(Self {
            path,
            size,
            children: Vec::new(),
            is_file,
        }))
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

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
        last_path_str: String,
    },
    Finished {
        junk_found: u64,
    },
}

fn should_include_dir(entry: &DirEntry) -> bool {
    let is_visible = !is_hidden(entry);
    let dir_to_exclude = ["/proc", "/sys", "/dev", "/run"];

    let is_dir_included = entry
        .file_name()
        .to_str()
        .map(|s| !dir_to_exclude.contains(&s))
        .unwrap_or(true);

    return is_visible && is_dir_included;
}

pub async fn iterate_dir<F>(root: &str, mut on_dir: F)
where
    F: FnMut(&Node),
{
    let mut path_map: HashMap<String, Node> = HashMap::new();
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
            path_map.insert(path_str.clone(), node.clone());

            // call the callback with a reference to the Node
            on_dir(&node);

            // link to parent (use clones, do NOT move `node` here)
            if let Some(parent_path) = path.parent() {
                let parent_path_str = parent_path.to_string_lossy().to_string();
                if let Some(parent_node) = path_map.get_mut(&parent_path_str) {
                    // borrow the parent's inner PathInfo mutably and add a clone of node
                    parent_node.borrow_mut().add_child(node.clone());
                }
            }
        }
    }
}

#[tauri::command]
pub async fn iterate_roots(_app: AppHandle, event: Channel<ScanEvent>) {
    let mut junk_size: u128 = 0;

    for root in get_roots() {
        iterate_dir(&root, |node| {
            let dir_info = node.borrow();

            if dir_info.is_file {
                junk_size = junk_size.saturating_add(dir_info.size);
            } else {
                let _ = event.send(ScanEvent::Progress {
                    junk_found: junk_size,
                    last_path_str: dir_info.path.clone(),
                });
            }

            // println!(
            //     "Directory: {}, Size: {}, is_file: {}",
            //     dir_info.path, dir_info.size, dir_info.is_file
            // );
        })
        .await;
    }
}
