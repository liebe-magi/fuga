use crate::fuga::FileInfo;
use crate::traits::{FileSystemService, PathService};

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
    ) -> String {
        match name {
            Some(name) => {
                // Check if destination is a directory using file system service
                match fs_service.get_file_info(&name) {
                    Ok(dest_info) if dest_info.exists && dest_info.is_dir => {
                        // Get target name from pre-fetched info
                        match &target_info.name {
                            Some(target_name) => format!("{name}/{target_name}"),
                            None => name, // Fallback if can't get name
                        }
                    }
                    _ => name, // Not a directory or doesn't exist, use as-is
                }
            }
            None => {
                // Use target name from pre-fetched info
                target_info
                    .name
                    .clone()
                    .unwrap_or_else(|| target.to_string())
            }
        }
    }
}
