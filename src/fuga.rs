use dirs::config_dir;

use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::env;
use std::fs::metadata;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use termion::{color, style};

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

/// The struct of the config file
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub user_config: UserConfig,
    pub data: Data,
}

/// The struct of the user config.
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub box_path: String,
}

/// The struct of data.
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Data {
    pub target: String,
}

/// Get the path of the config file.
fn get_config_path() -> Option<PathBuf> {
    config_dir()
}

/// Get the bold text.
fn get_bold_text(text: &str) -> String {
    format!("{}{}{}", style::Bold, text, style::Reset)
}

/// Get the colorized text.
pub fn get_colorized_text(text: &str, is_bold: bool) -> String {
    match is_bold {
        true => format!(
            "{}{}{}",
            color::Fg(color::LightGreen),
            get_bold_text(text),
            color::Fg(color::Reset)
        ),
        false => format!(
            "{}{}{}",
            color::Fg(color::LightGreen),
            text,
            color::Fg(color::Reset)
        ),
    }
}

/// load AppConfig.
pub fn load_config() -> Result<AppConfig, confy::ConfyError> {
    match confy::load::<AppConfig>(APP_NAME, APP_NAME) {
        Ok(mut config) => match config.user_config.box_path.is_empty() {
            true => {
                let config_path = match get_config_path() {
                    Some(path) => path,
                    None => {
                        panic!("Failed to get config path.");
                    }
                };
                config.user_config.box_path = format!("{}", config_path.join("box").display());
                match confy::store(APP_NAME, APP_NAME, &config) {
                    Ok(_) => Ok(config),
                    Err(e) => Err(e),
                }
            }
            false => Ok(config),
        },
        Err(err) => Err(err),
    }
}

/// Store the target path into the config file.
pub fn store_path(target: &str) -> Result<(), confy::ConfyError> {
    match load_config() {
        Ok(mut config) => {
            config.data.target = target.to_string();
            match confy::store(APP_NAME, APP_NAME, &config) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        Err(err) => Err(err),
    }
}

/// Get version of this tool.
pub fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let version_text = format!("v{version}");
    version_text
}

/// Get comprehensive file information with a single system call.
pub fn get_file_info(path: &str) -> Result<FileInfo, std::io::Error> {
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
            _ => Err(e),
        },
    }
}

/// Get the type of the target file or directory with proper error handling.
pub fn get_file_type_result(path: &str) -> Result<TargetType, std::io::Error> {
    match get_file_info(path) {
        Ok(info) => {
            if !info.exists {
                Ok(TargetType::None)
            } else if info.is_file {
                Ok(TargetType::File)
            } else {
                Ok(TargetType::Dir)
            }
        }
        Err(e) => Err(e),
    }
}

/// Get the type of the target file or directory.
/// Note: This function treats I/O errors as TargetType::None for backward compatibility.
/// For proper error handling, use get_file_type_result() instead.
pub fn get_file_type(path: &str) -> TargetType {
    match get_file_type_result(path) {
        Ok(target_type) => target_type,
        Err(_) => TargetType::None,
    }
}

/// Check if the path is an absolute path.
fn is_abs_path(path: &str) -> bool {
    path.starts_with('/')
}

/// Get the absolute path of the target file or directory.
pub fn get_abs_path(path: &str) -> String {
    match is_abs_path(path) {
        true => path.to_string(),
        false => match env::current_dir() {
            Ok(current) => current.join(path).display().to_string(),
            Err(_) => panic!("Failed to get current directory."),
        },
    }
}

/// Get the name of file or directory from the path with optimized file info retrieval.
pub fn get_name(path: &str) -> Result<String, std::io::Error> {
    match get_file_info(path) {
        Ok(info) => {
            if !info.exists {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("{path} does not exist."),
                ));
            }
            match info.name {
                Some(name) => Ok(name),
                None => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Failed to get file name for {path}."),
                )),
            }
        }
        Err(e) => Err(e),
    }
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

/// Copy the file or directiory.
pub fn copy_items(src: &str, dst: &str) -> Result<(), fs_extra::error::Error> {
    let abs_src = get_abs_path(src);
    let abs_dst = get_abs_path(dst);
    if abs_src == abs_dst {
        return Err(fs_extra::error::Error::new(
            fs_extra::error::ErrorKind::InvalidPath,
            "The source and destination path are the same.",
        ));
    }
    let pbr = Rc::new(RefCell::new(None));
    let update_pbr = |copied, total, item_name: &str| {
        let mut pbr = pbr.borrow_mut();
        let pbr = pbr.get_or_insert_with(|| create_progress_bar(total));
        pbr.set_position(copied);
        pbr.set_message(item_name.to_string());
    };
    match get_file_type(&abs_src) {
        TargetType::File => {
            let mut options = fs_extra::file::CopyOptions::new();
            options.overwrite = true;
            let handle = |process_info: fs_extra::file::TransitProcess| {
                update_pbr(process_info.copied_bytes, process_info.total_bytes, dst);
            };
            match fs_extra::file::copy_with_progress(abs_src, abs_dst, &options, handle) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        TargetType::Dir => {
            let mut options = fs_extra::dir::CopyOptions::new();
            options.overwrite = true;
            options.copy_inside = true;
            let handle = |process_info: fs_extra::dir::TransitProcess| {
                update_pbr(process_info.copied_bytes, process_info.total_bytes, dst);
                fs_extra::dir::TransitProcessResult::ContinueOrAbort
            };
            match fs_extra::dir::copy_with_progress(abs_src, abs_dst, &options, handle) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        TargetType::None => Err(fs_extra::error::Error::new(
            fs_extra::error::ErrorKind::InvalidPath,
            "The source path is not exist.",
        )),
    }
}

/// Move the file or directory.
pub fn move_items(src: &str, dst: &str) -> Result<(), fs_extra::error::Error> {
    let abs_src = get_abs_path(src);
    let abs_dst = get_abs_path(dst);
    if abs_src == abs_dst {
        return Err(fs_extra::error::Error::new(
            fs_extra::error::ErrorKind::InvalidPath,
            "The source and destination path are the same.",
        ));
    }
    let pbr = Rc::new(RefCell::new(None));
    let update_pbr = |copied, total, item_name: &str| {
        let mut pbr = pbr.borrow_mut();
        let pbr = pbr.get_or_insert_with(|| create_progress_bar(total));
        pbr.set_position(copied);
        pbr.set_message(item_name.to_string());
    };
    match get_file_type(&abs_src) {
        TargetType::File => {
            let mut options = fs_extra::file::CopyOptions::new();
            options.overwrite = true;
            let handle = |process_info: fs_extra::file::TransitProcess| {
                update_pbr(process_info.copied_bytes, process_info.total_bytes, dst);
            };
            match fs_extra::file::move_file_with_progress(abs_src, abs_dst, &options, handle) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        TargetType::Dir => {
            let mut options = fs_extra::dir::CopyOptions::new();
            options.overwrite = true;
            options.copy_inside = true;
            let handle = |process_info: fs_extra::dir::TransitProcess| {
                update_pbr(process_info.copied_bytes, process_info.total_bytes, dst);
                fs_extra::dir::TransitProcessResult::ContinueOrAbort
            };
            match fs_extra::dir::move_dir_with_progress(abs_src, abs_dst, &options, handle) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        TargetType::None => Err(fs_extra::error::Error::new(
            fs_extra::error::ErrorKind::InvalidPath,
            "The source path is not exist.",
        )),
    }
}

/// Make the symbolic link.
pub fn link_items(src: &str, dst: &str) -> Result<(), std::io::Error> {
    let abs_src = get_abs_path(src);
    let abs_dst = get_abs_path(dst);
    if abs_src == abs_dst {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "The source and destination path are the same.",
        ));
    }
    match get_file_type(&abs_src) {
        TargetType::None => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "The source path is not exist.",
        )),
        _ => match symlink(&abs_src, &abs_dst) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
    }
}

/// Get the icon of the file or directory.
pub fn get_icon(path: &str) -> String {
    match get_file_type(path) {
        TargetType::File => "📄".to_string(),
        TargetType::Dir => "📁".to_string(),
        TargetType::None => "❌".to_string(),
    }
}

/// Reset the mark.
pub fn reset_mark() -> Result<(), confy::ConfyError> {
    match load_config() {
        Ok(mut config) => {
            config.data.target = "".to_string();
            match confy::store(APP_NAME, APP_NAME, &config) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

/// Check if the target is file or directory and return the destination name with optimized file operations.
pub fn get_destination_name(target: &str, name: Option<String>) -> String {
    match name {
        Some(name) => {
            // Get file info for destination to check if it's a directory
            match get_file_info(&name) {
                Ok(dest_info) if dest_info.exists && dest_info.is_dir => {
                    // Get target name efficiently
                    match get_file_info(target) {
                        Ok(target_info) if target_info.exists => {
                            match target_info.name {
                                Some(target_name) => format!("{name}/{target_name}"),
                                None => name, // Fallback if can't get name
                            }
                        }
                        _ => name, // Fallback if target doesn't exist
                    }
                }
                _ => name, // Not a directory or doesn't exist, use as-is
            }
        }
        None => match get_name(target) {
            Ok(name) => name,
            Err(_) => target.to_string(), // Fallback to full path if name extraction fails
        },
    }
}

/// Get the marked path.
pub fn get_marked_path() -> Result<String, confy::ConfyError> {
    match load_config() {
        Ok(config) => Ok(config.data.target),
        Err(e) => Err(e),
    }
}
