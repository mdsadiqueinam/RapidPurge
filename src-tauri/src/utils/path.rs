use walkdir::DirEntry;

pub fn list_root_volumes() -> Vec<String> {
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

pub fn default_flash_scan_paths() -> Vec<String> {
    let home = dirs::home_dir().unwrap().to_string_lossy().to_string();

    #[cfg(target_os = "windows")]
    {
        return ('A'..='Z')
            .filter(|d| std::path::Path::new(&format!("{}:\\", d)).exists())
            .map(|d| format!("{}:\\", d))
            .collect();
    }

    #[cfg(target_os = "macos")]
    {
        vec![
            // ────────────────────────
            // Cache and "Junk" Files
            // ────────────────────────
            "/System/Library/Caches".to_string(),
            format!("{}/Library/Caches", home),
            "/private/var/folders".to_string(),
            // ────────────────────────
            // Log Files
            // ────────────────────────
            "/Library/Logs".to_string(),
            format!("{}/Library/Logs", home),
            "/var/log".to_string(),
            "/private/var/log".to_string(),
            format!("{}/Library/Logs/DiagnosticReports", home),
            "/Library/Logs/DiagnosticReports".to_string(),
            // ────────────────────────
            // Temporary Files
            // ────────────────────────
            "/tmp".to_string(),
            "/private/tmp".to_string(),
            "/var/tmp".to_string(),
            "/private/var/tmp".to_string(),
            "/private/var/vm".to_string(),
            // ────────────────────────
            // LaunchAgents and LaunchDaemons
            // ────────────────────────
            format!("{}/Library/LaunchAgents", home),
            "/Library/LaunchAgents".to_string(),
            "/Library/LaunchDaemons".to_string(),
            // ────────────────────────
            // Downloads Folder
            // ────────────────────────
            format!("{}/Downloads", home),
            // ────────────────────────
            // Trash Folders
            // ────────────────────────
            format!("{}/.Trash", home),
        ]
    }

    #[cfg(target_os = "linux")]
    {
        vec![
            // ────────────────────────
            // Cache and "Junk" Files
            // ────────────────────────
            "/var/cache".to_string(),
            format!("{}/.cache", home),
            // ────────────────────────
            // Log Files
            // ────────────────────────
            "/var/log".to_string(),
            // ────────────────────────
            // Temporary Files
            // ────────────────────────
            "/tmp".to_string(),
            "/var/tmp".to_string(),
            // ────────────────────────
            // Downloads Folder
            // ────────────────────────
            format!("{}/Downloads", home),
            // ────────────────────────
            // Trash Folders
            // ────────────────────────
            format!("{}/.local/share/Trash", home),
        ]
    }
}

pub fn has_full_disk_access() -> bool {
    #[cfg(target_os = "windows")]
    {
        // using app.embeded manifest with "requireAdministrator" should suffice
        return true;
    }

    #[cfg(target_os = "linux")]
    {
        // Must be root for full disk access
        return nix::unistd::Uid::effective().is_root();
    }

    #[cfg(target_os = "macos")]
    {
        // macOS TCC restrictions
        let test_paths = vec!["Library/Containers/com.apple.stocks", "Library/Safari"];

        return test_paths.iter().all(|p| std::fs::read_dir(p).is_ok());
    }
}

pub fn is_hidden_entry(entry: &DirEntry) -> bool {
    #[cfg(target_os = "windows")]
    {
        return true; // TODO: need to iplement later
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        entry
            .file_name()
            .to_str()
            .map(
                |s| s.starts_with(".") && s != ".Trash", // exclude .Trash from being hidden
            )
            .unwrap_or(false)
    }
}
