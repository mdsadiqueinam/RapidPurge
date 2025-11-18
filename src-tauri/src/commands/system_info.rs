use crate::utils::path::has_full_disk_access;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub struct SystemInfo {
    pub os: String,
    pub has_system_access: bool,
    pub message: String,
    pub can_open_access_panel: bool,
}

#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    #[cfg(target_os = "windows")]
    {
        SystemInfo {
            os: "Windows".to_string(),
            has_system_access: has_full_disk_access(),
            message: "Full disk access is generally available on Windows.".to_string(),
            can_open_access_panel: false,
        }
    }

    #[cfg(target_os = "linux")]
    {
        let has_access = has_full_disk_access();
        SystemInfo {
            os: "Linux".to_string(),
            has_system_access: has_access,
            message: if has_access {
                "Running with root privileges.".to_string()
            } else {
                "Not running as root. Some directories may be inaccessible.".to_string()
            },
            can_open_access_panel: false,
        }
    }

    #[cfg(target_os = "macos")]
    {
        let has_access = has_full_disk_access();
        SystemInfo {
            os: "macOS".to_string(),
            has_system_access: has_access,
            message: if has_access {
                "Full disk access granted.".to_string()
            } else {
                "Full disk access not granted. Please enable it in System Preferences.".to_string()
            },
            can_open_access_panel: true,
        }
    }
}

#[tauri::command]
pub fn open_access_panel() {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let _ = Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles")
            .status();
    }
}
