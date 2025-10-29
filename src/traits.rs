/// Traits for abstracting external dependencies and enabling dependency injection
use crate::config::AppConfig;
use crate::error::FugaResult;
use crate::fuga::{FileInfo, TargetType};

/// Trait for configuration management operations
pub trait ConfigRepository {
    /// Load the application configuration
    fn load_config(&self) -> FugaResult<AppConfig>;

    /// Store the configuration
    fn store_config(&self, config: &AppConfig) -> FugaResult<()>;

    /// Persist the full list of marked targets
    fn set_marked_targets(&self, targets: &[String]) -> FugaResult<()>;

    /// Retrieve the list of marked targets, migrating legacy state if required
    fn get_marked_targets(&self) -> FugaResult<Vec<String>>;

    /// Reset the marked targets to an empty list
    fn reset_marks(&self) -> FugaResult<()>;
}

/// Trait for file system operations
pub trait FileSystemService {
    /// Get comprehensive file information
    fn get_file_info(&self, path: &str) -> FugaResult<FileInfo>;

    /// Get the absolute path of a file or directory
    fn get_abs_path(&self, path: &str) -> FugaResult<String>;

    /// Get the type of the target file or directory
    fn get_file_type(&self, path: &str) -> TargetType;

    /// Copy files or directories
    fn copy_items(&self, src: &str, dst: &str) -> FugaResult<()>;

    /// Move files or directories
    fn move_items(&self, src: &str, dst: &str) -> FugaResult<()>;

    /// Create symbolic links
    fn link_items(&self, src: &str, dst: &str) -> FugaResult<()>;
}

/// Trait for UI operations
pub trait UIService {
    /// Get colorized text
    fn get_colorized_text(&self, text: &str, is_bold: bool) -> String;

    /// Get information icon
    fn get_icon_information(&self) -> String;

    /// Get icon for target type
    fn get_icon_for_target_type(&self, target_type: TargetType) -> String;
}

/// Trait for path operations
pub trait PathService {
    /// Get destination name for file operations with pre-fetched file info
    fn get_destination_name_with_info(
        &self,
        target: &str,
        target_info: &FileInfo,
        name: Option<&str>,
        fs_service: &dyn FileSystemService,
    ) -> FugaResult<String>;
}
