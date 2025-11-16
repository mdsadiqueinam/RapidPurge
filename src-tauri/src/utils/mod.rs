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

pub fn has_system_access() -> bool {
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
