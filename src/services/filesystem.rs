use crate::error::{FugaError, FugaResult};
use crate::fuga::{FileInfo, TargetType};
use crate::traits::FileSystemService;
use indicatif::{ProgressBar, ProgressStyle};
use std::cell::RefCell;
use std::env;
use std::fs::metadata;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::rc::Rc;

/// Standard file system service implementation
pub struct StandardFileSystemService;

impl StandardFileSystemService {
    pub fn new() -> Self {
        Self
    }

    /// Create a progress bar with shared styling.
    fn create_progress_bar(total: u64) -> ProgressBar {
        let pbr = ProgressBar::new(total);
        pbr.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .expect("Invalid progress bar template")
            .progress_chars("#>-"));
        pbr
    }

    /// Get the type of the target file or directory.
    fn get_file_type(&self, path: &str) -> TargetType {
        match self.get_file_info(path) {
            Ok(info) => {
                if !info.exists {
                    TargetType::None
                } else if info.is_file {
                    TargetType::File
                } else {
                    TargetType::Dir
                }
            }
            Err(_) => TargetType::None,
        }
    }

    /// Check if the path is an absolute path.
    fn is_abs_path(&self, path: &str) -> bool {
        path.starts_with('/')
    }
}

impl FileSystemService for StandardFileSystemService {
    fn get_file_info(&self, path: &str) -> FugaResult<FileInfo> {
        match metadata(path) {
            Ok(metadata) => Ok(FileInfo {
                exists: true,
                is_file: metadata.is_file(),
                is_dir: metadata.is_dir(),
                name: Path::new(path)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string()),
            }),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Ok(FileInfo {
                    exists: false,
                    is_file: false,
                    is_dir: false,
                    name: None,
                }),
                _ => Err(FugaError::from(e)),
            },
        }
    }

    fn get_abs_path(&self, path: &str) -> FugaResult<String> {
        if self.is_abs_path(path) {
            Ok(path.to_string())
        } else {
            let current = env::current_dir().map_err(|e| {
                FugaError::OperationFailed(format!("Failed to get current directory: {e}"))
            })?;
            Ok(current.join(path).display().to_string())
        }
    }

    fn copy_items(&self, src: &str, dst: &str) -> FugaResult<()> {
        let abs_src = self.get_abs_path(src)?;
        let abs_dst = self.get_abs_path(dst)?;

        if abs_src == abs_dst {
            return Err(FugaError::DuplicatePath {
                source: abs_src,
                destination: abs_dst,
            });
        }

        let pbr = Rc::new(RefCell::new(None));
        let update_pbr = |copied, total, item_name: &str| {
            let mut pbr = pbr.borrow_mut();
            let pbr = pbr.get_or_insert_with(|| Self::create_progress_bar(total));
            pbr.set_position(copied);
            pbr.set_message(item_name.to_string());
        };

        match self.get_file_type(&abs_src) {
            TargetType::File => {
                let mut options = fs_extra::file::CopyOptions::new();
                options.overwrite = true;
                let handle = |process_info: fs_extra::file::TransitProcess| {
                    update_pbr(process_info.copied_bytes, process_info.total_bytes, dst);
                };
                fs_extra::file::copy_with_progress(abs_src, abs_dst, &options, handle)
                    .map_err(|e| FugaError::FileSystemError(format!("Copy failed: {e}")))?;
            }
            TargetType::Dir => {
                let mut options = fs_extra::dir::CopyOptions::new();
                options.overwrite = true;
                options.copy_inside = true;
                let handle = |process_info: fs_extra::dir::TransitProcess| {
                    update_pbr(process_info.copied_bytes, process_info.total_bytes, dst);
                    fs_extra::dir::TransitProcessResult::ContinueOrAbort
                };
                fs_extra::dir::copy_with_progress(abs_src, abs_dst, &options, handle)
                    .map_err(|e| FugaError::FileSystemError(format!("Copy failed: {e}")))?;
            }
            TargetType::None => {
                return Err(FugaError::FileNotFound(abs_src));
            }
        }

        Ok(())
    }

    fn move_items(&self, src: &str, dst: &str) -> FugaResult<()> {
        let abs_src = self.get_abs_path(src)?;
        let abs_dst = self.get_abs_path(dst)?;

        if abs_src == abs_dst {
            return Err(FugaError::DuplicatePath {
                source: abs_src,
                destination: abs_dst,
            });
        }

        let pbr = Rc::new(RefCell::new(None));
        let update_pbr = |copied, total, item_name: &str| {
            let mut pbr = pbr.borrow_mut();
            let pbr = pbr.get_or_insert_with(|| Self::create_progress_bar(total));
            pbr.set_position(copied);
            pbr.set_message(item_name.to_string());
        };

        match self.get_file_type(&abs_src) {
            TargetType::File => {
                let mut options = fs_extra::file::CopyOptions::new();
                options.overwrite = true;
                let handle = |process_info: fs_extra::file::TransitProcess| {
                    update_pbr(process_info.copied_bytes, process_info.total_bytes, dst);
                };
                fs_extra::file::move_file_with_progress(abs_src, abs_dst, &options, handle)
                    .map_err(|e| FugaError::FileSystemError(format!("Move failed: {e}")))?;
            }
            TargetType::Dir => {
                let mut options = fs_extra::dir::CopyOptions::new();
                options.overwrite = true;
                options.copy_inside = true;
                let handle = |process_info: fs_extra::dir::TransitProcess| {
                    update_pbr(process_info.copied_bytes, process_info.total_bytes, dst);
                    fs_extra::dir::TransitProcessResult::ContinueOrAbort
                };
                fs_extra::dir::move_dir_with_progress(abs_src, abs_dst, &options, handle)
                    .map_err(|e| FugaError::FileSystemError(format!("Move failed: {e}")))?;
            }
            TargetType::None => {
                return Err(FugaError::FileNotFound(abs_src));
            }
        }

        Ok(())
    }

    fn link_items(&self, src: &str, dst: &str) -> FugaResult<()> {
        let abs_src = self.get_abs_path(src)?;
        let abs_dst = self.get_abs_path(dst)?;

        if abs_src == abs_dst {
            return Err(FugaError::DuplicatePath {
                source: abs_src,
                destination: abs_dst,
            });
        }

        match self.get_file_type(&abs_src) {
            TargetType::None => {
                return Err(FugaError::FileNotFound(abs_src));
            }
            _ => {
                symlink(&abs_src, &abs_dst)
                    .map_err(|e| FugaError::FileSystemError(format!("Link failed: {e}")))?;
            }
        }

        Ok(())
    }
}
