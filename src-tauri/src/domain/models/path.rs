use serde::Serialize;
use std::sync::{Arc, Mutex};

pub type Node = Arc<Mutex<PathInfo>>;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PathInfo {
    pub name: String,
    pub path: String,
    pub size: u128,
    pub children: Vec<Node>,
    pub is_file: bool,
}

impl PathInfo {
    pub fn from_path(path: String, size: u128, is_file: bool) -> Node {
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
