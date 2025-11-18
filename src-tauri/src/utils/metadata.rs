use std::fs::Metadata;
use std::path::Path;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::os::unix::fs::MetadataExt;

#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

#[cfg(target_os = "windows")]
use std::os::windows::prelude::AsRawHandle;

#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_os = "windows")]
use std::ptr;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::DWORD;
#[cfg(target_os = "windows")]
use winapi::um::fileapi::GetCompressedFileSizeW;
#[cfg(target_os = "windows")]
use winapi::um::fileapi::GetFileInformationByHandle;
#[cfg(target_os = "windows")]
use winapi::um::minwinbase::BY_HANDLE_FILE_INFORMATION;

/// File identity type for deduplication
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileId {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    Unix { dev: u64, ino: u64 },

    #[cfg(target_os = "windows")]
    Win {
        volume_serial: u32,
        file_index_high: u32,
        file_index_low: u32,
    },
}

/// Return (allocated_bytes, optional FileId)
pub fn file_allocated_bytes_and_id(path: &Path, meta: &Metadata) -> (u128, Option<FileId>) {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        // st_blocks * 512 is number of 512-byte blocks allocated on disk (UNIX)
        let blocks = meta.blocks();
        let bytes = (blocks as u128) * 512u128;
        let fid = FileId::Unix {
            dev: meta.dev() as u64,
            ino: meta.ino() as u64,
        };
        return (bytes, Some(fid));
    }

    #[cfg(target_os = "windows")]
    {
        // Use GetCompressedFileSizeW for allocated size (works for compressed/sparse files)
        // path -> wide string
        let w: Vec<u16> = OsStr::new(&path.as_os_str())
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            let mut high: DWORD = 0;
            let low = GetCompressedFileSizeW(w.as_ptr(), &mut high);
            if low == winapi::shared::minwindef::INVALID_FILE_SIZE {
                // on ERROR we fall back to meta.len()
                let fallback = meta.len() as u128;
                // try to get file id via GetFileInformationByHandle
                let file = std::fs::File::open(path);
                if let Ok(fh) = file {
                    let handle = fh.as_raw_handle();
                    let mut info: BY_HANDLE_FILE_INFORMATION = std::mem::zeroed();
                    if GetFileInformationByHandle(handle as *mut _, &mut info) != 0 {
                        let fid = FileId::Win {
                            volume_serial: info.dwVolumeSerialNumber,
                            file_index_high: info.nFileIndexHigh,
                            file_index_low: info.nFileIndexLow,
                        };
                        return (fallback, Some(fid));
                    }
                }
                return (fallback, None);
            } else {
                let allocated: u128 = ((high as u128) << 32) | (low as u128);
                // fetch file id
                let file = std::fs::File::open(path);
                if let Ok(fh) = file {
                    let handle = fh.as_raw_handle();
                    let mut info: BY_HANDLE_FILE_INFORMATION = std::mem::zeroed();
                    if GetFileInformationByHandle(handle as *mut _, &mut info) != 0 {
                        let fid = FileId::Win {
                            volume_serial: info.dwVolumeSerialNumber,
                            file_index_high: info.nFileIndexHigh,
                            file_index_low: info.nFileIndexLow,
                        };
                        return (allocated, Some(fid));
                    }
                }
                return (allocated, None);
            }
        }
    }

    // Generic fallback
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        (meta.len() as u128, None)
    }
}
