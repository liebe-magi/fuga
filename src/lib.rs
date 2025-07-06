pub mod commands;
pub mod config;
pub mod error;
pub mod fuga;
pub mod services;
pub mod traits;
pub mod ui;

pub use config::{AppConfig, FileConfigRepository};
pub use error::{FugaError, FugaResult};
pub use fuga::{get_version, FileInfo, TargetType, APP_NAME};

pub use traits::{ConfigRepository, FileSystemService, PathService, UIService};

pub use services::{StandardFileSystemService, StandardPathService};
pub use ui::TerminalUIService;
