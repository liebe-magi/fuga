use crate::error::{FugaError, FugaResult};
use crate::fuga::{FileInfo, TargetType};
use crate::traits::FileSystemService;
use indicatif::{ProgressBar, ProgressStyle};
use std::cell::RefCell;
use std::env;
use std::fs::metadata;
#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::{symlink_dir, symlink_file};
use std::path::Path;
use std::rc::Rc;

/// Progress bar template constants
const PRIMARY_PROGRESS_BAR_TEMPLATE: &str =
    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})";
const FALLBACK_PROGRESS_BAR_TEMPLATE: &str = "{bar:40} {bytes}/{total_bytes}";

/// Type alias for file operation setup result
type FileOperationSetup = (String, String, Box<dyn Fn(u64, u64, &str)>);

/// Standard file system service implementation
///
/// Provides cross-platform file system operations including file/directory manipulation,
/// path resolution, progress tracking, and symbolic link creation. Uses appropriate
/// platform-specific APIs (Unix symlink vs Windows symlink_file/symlink_dir).
#[derive(Default)]
pub struct StandardFileSystemService;

impl StandardFileSystemService {
    pub fn new() -> Self {
        Self
    }

    /// Create a progress bar with shared styling.
    fn create_progress_bar(total: u64) -> ProgressBar {
        let pbr = ProgressBar::new(total);

        // Use a fallback template if the primary one fails
        let style = ProgressStyle::default_bar()
            .template(PRIMARY_PROGRESS_BAR_TEMPLATE)
            .unwrap_or_else(|_| {
                // Fallback to simple template if primary fails
                ProgressStyle::default_bar()
                    .template(FALLBACK_PROGRESS_BAR_TEMPLATE)
                    .unwrap_or_else(|_| ProgressStyle::default_bar())
            })
            .progress_chars("#>-");

        pbr.set_style(style);
        pbr
    }

    /// Create shared progress bar update closure
    fn create_progress_update_closure(
        pbr: Rc<RefCell<Option<ProgressBar>>>,
    ) -> impl Fn(u64, u64, &str) {
        move |copied, total, item_name: &str| {
            let mut pbr = pbr.borrow_mut();
            let pbr = pbr.get_or_insert_with(|| Self::create_progress_bar(total));
            pbr.set_position(copied);
            pbr.set_message(item_name.to_string());
        }
    }

    /// Common setup for copy/move operations: path resolution, duplicate check, and progress setup
    fn setup_file_operation(&self, src: &str, dst: &str) -> FugaResult<FileOperationSetup> {
        let abs_src = self.get_abs_path(src)?;
        let abs_dst = self.get_abs_path(dst)?;

        if abs_src == abs_dst {
            return Err(FugaError::DuplicatePath {
                source: abs_src,
                destination: abs_dst,
            });
        }

        let pbr = Rc::new(RefCell::new(None));
        let update_pbr = Self::create_progress_update_closure(pbr);

        Ok((abs_src, abs_dst, Box::new(update_pbr)))
    }

    /// Check if the path is an absolute path.
    fn is_abs_path(&self, path: &str) -> bool {
        Path::new(path).is_absolute()
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
                _ => Err(FugaError::from_io_error(e, path)),
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

    fn copy_items(&self, src: &str, dst: &str) -> FugaResult<()> {
        let (abs_src, abs_dst, update_pbr) = self.setup_file_operation(src, dst)?;

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
        let (abs_src, abs_dst, update_pbr) = self.setup_file_operation(src, dst)?;

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
                #[cfg(unix)]
                {
                    symlink(&abs_src, &abs_dst)
                        .map_err(|e| FugaError::FileSystemError(format!("Link failed: {e}")))?;
                }
                #[cfg(windows)]
                {
                    // On Windows, use appropriate symlink function based on target type
                    match self.get_file_type(&abs_src) {
                        TargetType::Dir => {
                            symlink_dir(&abs_src, &abs_dst).map_err(|e| {
                                FugaError::FileSystemError(format!("Link failed: {e}"))
                            })?;
                        }
                        _ => {
                            symlink_file(&abs_src, &abs_dst).map_err(|e| {
                                FugaError::FileSystemError(format!("Link failed: {e}"))
                            })?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
