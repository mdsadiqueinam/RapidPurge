use crate::utils::get_roots;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use walkdir::WalkDir;

type Node = Rc<RefCell<PathInfo>>;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub struct PathInfo {
    pub path: String,
    pub size: u64,
    pub children: Vec<Node>,
    pub is_file: bool,
}

impl PathInfo {
    pub fn new(path: String, size: u64, is_file: bool) -> Node {
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

    pub fn parent_path_string(&self) -> String {
        let path = std::path::Path::new(&self.path);
        if let Some(parent) = path.parent() {
            return parent.to_string_lossy().to_string();
        }
        "".to_string()
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }
}

pub async fn iterate_dir<F>(root: &str, on_dir: F)
where
    F: Fn(&Node) + Send + 'static,
{
    let mut path_map: HashMap<String, Node> = HashMap::new();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok())
    // skip permission errors
    {
        let path = entry.path();
        if let Ok(metadata) = entry.metadata() {
            let size = metadata.len();
            let is_file = metadata.is_file();
            let path_str = path.to_string_lossy().to_string();

            // create the Node
            let node: Node = PathInfo::new(path_str.clone(), size, is_file);

            // insert a clone into the map (cheap — increments refcount)
            path_map.insert(path_str.clone(), node.clone());

            // call the callback with a reference to the Node
            if !is_file {
                on_dir(&node);
            }

            // link to parent (use clones, do NOT move `node` here)
            if let Some(parent_path) = path.parent() {
                let parent_path_str = parent_path.to_string_lossy().to_string();
                if let Some(parent_node) = path_map.get_mut(&parent_path_str) {
                    // borrow the parent's inner PathInfo mutably and add a clone of node
                    parent_node.borrow_mut().add_child(node.clone());
                }
            }
        }

        if path.is_file() {
            break;
        }
    }
}

#[tauri::command]
pub async fn iterate_roots() {
    for root in get_roots() {
        iterate_dir(&root, |node| {
            let dir_info = node.borrow();
            println!(
                "Directory: {}, Size: {}, is_file: {}",
                dir_info.path, dir_info.size, dir_info.is_file
            );
        })
        .await;
    }
}
