use std::sync::LazyLock;

use crate::domain::models::flash_scan::FlashScanCategory;

#[cfg(target_os = "windows")]
static FLASH_SCAN_CATEGORIES: LazyLock<Vec<FlashScanCategory>> = LazyLock::new(|| {
    vec![
        FlashScanCategory {
            name: "Temporary Files".to_string(),
            paths: vec![
                "C:\\Windows\\Temp".to_string(),
                "C:\\Users\\%USERNAME%\\AppData\\Local\\Temp".to_string(),
            ],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            name: "Recycle Bin".to_string(),
            paths: vec!["C:\\$Recycle.Bin".to_string()],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            name: "Browser Cache".to_string(),
            paths: Vec::new(),
            regexp: None,
            sub_categories: Some(vec![
                FlashScanCategory {
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
                    name: "Chrome".to_string(),
                    paths: vec![],
                    regexp: Some(
                        regex::RegexSet::new(&[r".*\\Google\\Chrome\\User Data\\Default\\Cache.*"])
                            .unwrap(),
                    ),
                    sub_categories: None,
                },
                FlashScanCategory {
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
            name: "User Cache".to_string(),
            paths: vec![format!("{}/Library/Caches", home)],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
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
            name: "Browser Cache".to_string(),
            paths: Vec::new(),
            regexp: None,
            sub_categories: Some(vec![
                FlashScanCategory {
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
            name: "User Log Files".to_string(),
            paths: vec![
                format!("{}/Library/Logs", home),
                format!("{}/Library/Logs/DiagnosticReports", home),
            ],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
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
            name: "Downloads".to_string(),
            paths: vec![format!("{}/Downloads", home)],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            name: "Trash".to_string(),
            paths: vec![format!("{}/.Trash", home)],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
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
            name: "Cache and Junk Files".to_string(),
            paths: vec!["/var/cache".to_string(), format!("{}/.cache", home)],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            name: "Log Files".to_string(),
            paths: vec!["/var/log".to_string()],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            name: "Temporary Files".to_string(),
            paths: vec!["/tmp".to_string(), "/var/tmp".to_string()],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
            name: "Downloads".to_string(),
            paths: vec![format!("{}/Downloads", home)],
            regexp: None,
            sub_categories: None,
        },
        FlashScanCategory {
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
