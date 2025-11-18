use walkdir::DirEntry;

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
