use crate::utils::get_roots;
use walkdir::WalkDir;

#[tauri::command]
pub async fn iterate_ro() {
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
