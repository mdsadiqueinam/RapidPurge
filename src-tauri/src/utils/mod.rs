pub fn get_roots() -> Vec<String> {
    #[cfg(target_os = "windows")]
    {
        return ('A'..='Z')
            .filter(|d| std::path::Path::new(&format!("{}:\\", d)).exists())
            .map(|d| format!("{}:\\", d))
            .collect();
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        vec!["/".to_string()]
    }
}

pub fn has_system_access() {}
