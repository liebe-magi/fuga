use std::env;

pub use crate::error::FugaError;

/// The application's name.
pub const APP_NAME: &str = "fuga";

/// The type of the target file or directory
#[derive(Debug, Clone, PartialEq)]
pub enum TargetType {
    File,
    Dir,
    None,
}

/// Consolidated file information to reduce system calls
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub exists: bool,
    pub is_file: bool,
    pub is_dir: bool,
    pub name: Option<String>,
}

// Legacy functions removed - functionality migrated to service layer

/// Get version of this tool.
pub fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let version_text = format!("v{version}");
    version_text
}

// All legacy functions have been migrated to the service layer architecture
