use crate::error::{FugaError, FugaResult};
use crate::fuga::FileInfo;
use crate::traits::{FileSystemService, PathService};
use std::path::PathBuf;

/// Standard path service implementation
#[derive(Default)]
pub struct StandardPathService;

impl StandardPathService {
    pub fn new() -> Self {
        Self
    }
}

impl PathService for StandardPathService {
    fn get_destination_name_with_info(
        &self,
        target: &str,
        target_info: &FileInfo,
        name: Option<String>,
        fs_service: &dyn FileSystemService,
    ) -> FugaResult<String> {
        match name {
            Some(dest) => {
                // Check if destination is a directory using file system service
                match fs_service.get_file_info(&dest)? {
                    dest_info if dest_info.exists && dest_info.is_dir => {
                        // Get target name from pre-fetched info
                        match &target_info.name {
                            Some(target_name) => {
                                let mut composed = PathBuf::from(&dest);
                                composed.push(target_name);
                                composed.into_os_string().into_string().map_err(|_| {
                                    FugaError::OperationFailed(
                                        "Destination path contains invalid UTF-8 after composition"
                                            .to_string(),
                                    )
                                })
                            }
                            None => Ok(dest), // Fallback if can't get name
                        }
                    }
                    _ => Ok(dest), // Not a directory or doesn't exist, use as-is
                }
            }
            None => {
                // Use target name from pre-fetched info
                Ok(target_info
                    .name
                    .clone()
                    .unwrap_or_else(|| target.to_string()))
            }
        }
    }
}
