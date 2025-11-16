use crate::utils::get_roots;
use serde::Serialize;
use walkdir::WalkDir;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub struct Dir {
    pub path: String,
    pub size: u64,
    pub children: Vec<Dir>,
}

impl Dir {
    pub fn new(path: String, size: u64) -> Self {
        Self {
            path,
            size,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Dir) {
        self.children.push(child);
    }
}

pub async fn iterate_roots() {
    for root in get_roots() {
        for entry in WalkDir::new(&root).into_iter().filter_map(|e| e.ok())
        // skip permission errors
        {
            let path = entry.path();
            if let Ok(metadata) = entry.metadata() {
                let size = metadata.len();
                println!("{:?} -> {} bytes", path, size);
            }
        }
    }
}
