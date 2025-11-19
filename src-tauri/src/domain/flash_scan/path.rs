use std::sync::LazyLock;

use crate::domain::models::flash_scan::FlashScanCategory;

#[cfg(target_os = "windows")]
static FLASH_SCAN_CATEGORIES: LazyLock<Vec<FlashScanCategory>> = LazyLock::new(|| {
    vec![
        FlashScanCategory {
            id: "TEMP_FILES".to_string(),
            name: "Temporary Files".to_string(),
            paths: vec![
                "C:\\Windows\\Temp".to_string(),
                "C:\\Users\\%USERNAME%\\AppData\\Local\\Temp".to_string(),
            ],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            id: "RECYCLE_BIN".to_string(),
            name: "Recycle Bin".to_string(),
            paths: vec!["C:\\$Recycle.Bin".to_string()],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            id: "BROWSER_CACHE".to_string(),
            name: "Browser Cache".to_string(),
            paths: Vec::new(),
            regexp: None,
            sub_categories: Some(vec![
                FlashScanCategory {
                    id: "EDGE".to_string(),
                    name: "Edge".to_string(),
                    paths: vec![],
                    regexp: Some(
                        regex::RegexSet::new(&[
                            r".*\\Microsoft\\Edge\\User Data\\Default\\Cache.*",
                        ])
                        .unwrap(),
                    ),
                    sub_categories: None,
                },
                FlashScanCategory {
                    id: "CHROME".to_string(),
                    name: "Chrome".to_string(),
                    paths: vec![],
                    regexp: Some(
                        regex::RegexSet::new(&[r".*\\Google\\Chrome\\User Data\\Default\\Cache.*"])
                            .unwrap(),
                    ),
                    sub_categories: None,
                },
                FlashScanCategory {
                    id: "FIREFOX".to_string(),
                    name: "Firefox".to_string(),
                    paths: vec![],
                    regexp: Some(
                        regex::RegexSet::new(&[r".*\\Mozilla\\Firefox\\Profiles\\.*\\cache2.*"])
                            .unwrap(),
                    ),
                    sub_categories: None,
                },
            ]),
        },
    ]
});

#[cfg(target_os = "macos")]
static FLASH_SCAN_CATEGORIES: LazyLock<Vec<FlashScanCategory>> = LazyLock::new(|| {
    let home = dirs::home_dir()
        .unwrap_or(std::path::PathBuf::from("/Users/Unknown"))
        .to_string_lossy()
        .to_string();

    vec![
        FlashScanCategory {
            id: "USER_CACHE".to_string(),
            name: "User Cache".to_string(),
            paths: vec![format!("{}/Library/Caches", home)],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            id: "SYSTEM_CACHE".to_string(),
            name: "System Cache".to_string(),
            paths: vec![
                "/System/Library/Caches".to_string(),
                "/private/var/folders".to_string(),
                "/var/folders".to_string(),
            ],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            id: "BROWSER_CACHE".to_string(),
            name: "Browser Cache".to_string(),
            paths: Vec::new(),
            regexp: None,
            sub_categories: Some(vec![
                FlashScanCategory {
                    id: "SAFARI".to_string(),
                    name: "Safari".to_string(),
                    paths: Vec::new(),
                    regexp: Some(
                        regex::RegexSet::new(&[
                            r"^/private/var/folders/.+/com\.apple\.Safari.*$",
                            r".*/Library/Caches/Safari$",
                        ])
                        .unwrap(),
                    ),
                    sub_categories: None,
                },
                FlashScanCategory {
                    id: "CHROME".to_string(),
                    name: "Chrome".to_string(),
                    paths: Vec::new(),
                    regexp: Some(
                        regex::RegexSet::new(&[
                            r"^/private/var/folders/.+/com\.google\.Chrome.*$",
                            r".*/Library/Caches/Google/Chrome$",
                        ])
                        .unwrap(),
                    ),
                    sub_categories: None,
                },
                FlashScanCategory {
                    id: "FIREFOX".to_string(),
                    name: "Firefox".to_string(),
                    paths: Vec::new(),
                    regexp: Some(
                        regex::RegexSet::new(&[
                            r"^/private/var/folders/.+/org\.mozilla\.firefox.*$",
                            r".*/Library/Caches/Firefox$",
                        ])
                        .unwrap(),
                    ),
                    sub_categories: None,
                },
                FlashScanCategory {
                    id: "EDGE".to_string(),
                    name: "Edge".to_string(),
                    paths: Vec::new(),
                    regexp: Some(
                        regex::RegexSet::new(&[
                            r"^/private/var/folders/.+/com\.microsoft\.Edge.*$",
                            r".*/Library/Caches/Microsoft Edge$",
                        ])
                        .unwrap(),
                    ),
                    sub_categories: None,
                },
                FlashScanCategory {
                    id: "OPERA".to_string(),
                    name: "Opera".to_string(),
                    paths: Vec::new(),
                    regexp: Some(
                        regex::RegexSet::new(&[
                            r"^/private/var/folders/.+/com\.opera\.Opera.*$",
                            r".*/Library/Caches/com.operasoftware.Opera$",
                        ])
                        .unwrap(),
                    ),
                    sub_categories: None,
                },
                FlashScanCategory {
                    id: "BRAVE".to_string(),
                    name: "Brave".to_string(),
                    paths: Vec::new(),
                    regexp: Some(
                        regex::RegexSet::new(&[
                            r"^/private/var/folders/.+/com\.brave\.Browser.*$",
                            r".*/Library/Caches/BraveSoftware/Brave-Browser$",
                        ])
                        .unwrap(),
                    ),
                    sub_categories: None,
                },
            ]),
        },
        FlashScanCategory {
            id: "LOG_FILES".to_string(),
            name: "System Log Files".to_string(),
            paths: vec![
                "/Library/Logs".to_string(),
                "/var/log".to_string(),
                "/private/var/log".to_string(),
                "/Library/Logs/DiagnosticReports".to_string(),
            ],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            id: "USER_LOG_FILES".to_string(),
            name: "User Log Files".to_string(),
            paths: vec![
                format!("{}/Library/Logs", home),
                format!("{}/Library/Logs/DiagnosticReports", home),
            ],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            id: "TEMP_FILES".to_string(),
            name: "Temporary Files".to_string(),
            paths: vec![
                "/tmp".to_string(),
                "/private/tmp".to_string(),
                "/var/tmp".to_string(),
                "/private/var/tmp".to_string(),
                "/private/var/vm".to_string(),
            ],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            id: "DOWNLOADS".to_string(),
            name: "Downloads".to_string(),
            paths: vec![format!("{}/Downloads", home)],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            id: "TRASH".to_string(),
            name: "Trash".to_string(),
            paths: vec![format!("{}/.Trash", home)],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            id: "LAUNCH_AGENTS_DAEMONS".to_string(),
            name: "LaunchAgents and LaunchDaemons".to_string(),
            paths: vec![
                format!("{}/Library/LaunchAgents", home),
                "/Library/LaunchAgents".to_string(),
                "/Library/LaunchDaemons".to_string(),
            ],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            id: "DMG_FILES".to_string(),
            name: "Unused DMG Files".to_string(),
            paths: vec![],
            regexp: Some(regex::RegexSet::new(&[r".*\.dmg$"]).unwrap()),
            sub_categories: None,
        },
    ]
});

#[cfg(target_os = "linux")]
static FLASH_SCAN_CATEGORIES: LazyLock<Vec<FlashScanCategory>> = LazyLock::new(|| {
    let home = dirs::home_dir().unwrap().to_string_lossy().to_string();
    vec![
        FlashScanCategory {
            id: "CACHE_FILES".to_string(),
            name: "Cache Files".to_string(),
            paths: vec!["/var/cache".to_string(), format!("{}/.cache", home)],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            id: "LOG_FILES".to_string(),
            name: "Log Files".to_string(),
            paths: vec!["/var/log".to_string()],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            id: "TEMP_FILES".to_string(),
            name: "Temporary Files".to_string(),
            paths: vec!["/tmp".to_string(), "/var/tmp".to_string()],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            id: "DOWNLOADS".to_string(),
            name: "Downloads".to_string(),
            paths: vec![format!("{}/Downloads", home)],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            id: "TRASH_FOLDERS".to_string(),
            name: "Trash Folders".to_string(),
            paths: vec![format!("{}/.local/share/Trash", home)],
            regexp: None,
            sub_categories: None,
        },
    ]
});

pub fn default_flash_scan_paths() -> Vec<String> {
    return FLASH_SCAN_CATEGORIES
        .iter()
        .flat_map(|category| category.paths.clone())
        .collect();
}
