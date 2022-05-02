mod fugue;

use clap::{Args, Parser, Subcommand};
// use std::env;
// use std::fs::metadata;
use std::ops::RangeInclusive;
// use std::path::Path;
use std::str::FromStr;

const INDEX_RANGE: RangeInclusive<usize> = 0..=9;

// enum FileType {
//     File,
//     Dir,
//     None,
// }

#[derive(Parser)]
#[clap(
    name = fugue::APP_NAME,
    author = "MagicalLiebe <magical.liebe@gmail.com>",
    version = fugue::VERSION,
    about = "A CLI tool to make C&P files and directories more easier."
)]
struct AppArg {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Create a new box
    Create {
        /// The name of the box to create
        box_name: String,
    },
    /// Delete the box
    Delete {
        /// The name of the box to delete
        box_name: String,
    },
    /// Clean up all the items in the box
    Clean {
        /// The name of the box to clean up
        box_name: Option<String>,
    },
    /// List all the items in the box
    Show,
    /// Add a file or directory path to the temp
    Add(Add),
    /// Pop a file or directory from the temp
    Pop(Pop),
    /// Drop a file or directory from the temp
    Drop,
    /// List all boxes
    List,
    /// Show the config
    Config,
    /// Show the version of the tool
    Version,
}

#[derive(Args)]
struct Add {
    /// The target path (file or directory).
    path: String,
}

#[derive(Args)]
struct Pop {
    /// Delete a file or directory after copying
    #[clap(short = 'm', long = "move")]
    is_move: bool,

    /// Index of
    #[clap(short = 'i', long = "index", validator = index_in_range, default_value = "0")]
    index: usize,

    /// The destination path to be copied or moved.
    path: Option<String>,
}

// fn is_absolute_path(path: &str) -> bool {
//     return path.starts_with("/");
// }

// fn organize_path(path: &str) -> String {
//     if is_absolute_path(path) {
//         return path.to_string();
//     } else {
//         let abs_path = env::current_dir()
//             .unwrap()
//             .join(path)
//             .to_str()
//             .unwrap()
//             .to_string();
//         return abs_path;
//     }
// }

// fn is_exist(path: &str) -> bool {
//     return Path::new(path).exists();
// }

// fn is_file(path: &str) -> bool {
//     return metadata(path).unwrap().is_file();
// }

// fn get_file_type(path: &str) -> FileType {
//     if is_exist(path) {
//         if is_file(path) {
//             return FileType::File;
//         } else {
//             return FileType::Dir;
//         }
//     } else {
//         return FileType::None;
//     }
// }

// fn get_file_name(path: &str) -> String {
//     return Path::new(path)
//         .file_name()
//         .unwrap()
//         .to_str()
//         .unwrap()
//         .to_string();
// }

// fn get_dir_name(path: &str) -> String {
//     return Path::new(path)
//         .parent()
//         .unwrap()
//         .to_str()
//         .unwrap()
//         .to_string();
// }

// fn get_path_list() -> Vec<String> {
//     let config = confy::load::<AppConfig>("fugue").unwrap_or_default();
//     return config.path_list;
// }

// fn add_path(path: &str) {
//     let mut path_list = get_path_list();
//     path_list.insert(0, path.to_string());
//     if path_list.len() == 11 {
//         path_list.remove(10);
//     }
//     confy::store("fugue", &AppConfig { path_list }).unwrap();
// }

// fn save_path(path_list: Vec<String>) {
//     confy::store("fugue", &AppConfig { path_list }).unwrap();
// }

// fn show_path_list() {
//     let path_list = get_path_list();
//     for (i, path) in path_list.iter().enumerate() {
//         match get_file_type(&path) {
//             FileType::File => println!("[{}] 📄 {}", i, path),
//             FileType::Dir => println!("[{}] 📁 {}", i, path),
//             FileType::None => println!("[{}] ❌ {}", i, path),
//         }
//     }
// }

fn index_in_range(s: &str) -> Result<(), String> {
    usize::from_str(s)
        .map(|index| INDEX_RANGE.contains(&index))
        .map_err(|e| e.to_string())
        .and_then(|result| match result {
            true => Ok(()),
            false => Err(format!(
                "Index not in range {}-{}",
                INDEX_RANGE.start(),
                INDEX_RANGE.end()
            )),
        })
}

fn main() {
    let cli = AppArg::parse();
    match cli.action {
        Action::Create { box_name } => {
            println!(
                "✨ Created the new box : 📦️ {}",
                fugue::get_colorized_text(&box_name, true)
            );
        }
        Action::Delete { box_name } => {
            println!(
                "🔥 Deleted the box : 📦️ {}",
                fugue::get_colorized_text(&box_name, true)
            );
        }
        Action::Clean { box_name } => {
            match box_name {
                Some(box_name) => println!(
                    "🧹 Cleaned all the items in the box : 📦️ {}",
                    fugue::get_colorized_text(&box_name, true)
                ),
                None => println!("🧹 Cleaned all the items in the temp"),
            }
            // let path_list = get_path_list();
            // let before_len = path_list.len();
            // let mut clean_path_list = Vec::new();
            // for path in path_list {
            //     if is_exist(&path) {
            //         clean_path_list.push(path);
            //     }
            // }
            // if before_len - clean_path_list.len() > 0 {
            //     println!(
            //         "Clean: 🧹 {} paths have been cleaned.",
            //         before_len - clean_path_list.len()
            //     );
            //     save_path(clean_path_list);
            //     show_path_list();
            // } else {
            //     println!("Clean: 🧹 No path has been cleaned.");
            // }
        }

        Action::Show => {
            println!("Showing all boxes");
        }
        Action::Add(add) => {
            println!("Adding {} to box", add.path);
            // let abs_path = organize_path(&add.path);
            // match get_file_type(&abs_path) {
            //     FileType::File => {
            //         add_path(&abs_path);
            //         println!("Add: 📄 {}", &abs_path);
            //     }
            //     FileType::Dir => {
            //         add_path(&abs_path);
            //         println!("Add: 📁 {}", abs_path);
            //     }
            //     FileType::None => {
            //         println!("Failed: ❌ \"{}\" is not exist.", abs_path);
            //     }
            // }
        }
        Action::Pop(pop) => {
            println!("Popping {} from box", pop.path.unwrap_or("".to_string()));
            // let mut cfg = confy::load::<AppConfig>("canp").unwrap_or_default();
            // if cfg.path == "" {
            //     println!("❌ No file or directory has pushed.");
            // } else {
            //     let file_name = pop.path.unwrap_or(get_file_name(&cfg.path));
            //     let dir_name = get_dir_name(&file_name);
            //     if dir_name != "" {
            //         if !is_exist(&dir_name) {
            //             println!("Dir: {} is not exist", dir_name);
            //             return;
            //         }
            //     }
            //     if is_exist(&cfg.path) {
            //         if is_file(&cfg.path) {
            //             copy(&cfg.path, &file_name).unwrap();
            //             println!("File: '{}' has poped.", &cfg.path);
            //         } else {
            //             println!("Dir: '{}' has poped.", &cfg.path);
            //         }
            //         if pop.is_move {
            //             println!("move");
            //         }
            //         cfg.path = "".to_string();
            //         confy::store("canp", cfg).unwrap();
            //     } else {
            //         println!("Failed: ❌ \"{}\" is not exist.", cfg.path);
            //     }
            // }
        }
        Action::Drop => {
            // let mut cfg = confy::load::<AppConfig>("canp").unwrap_or_default();
            // if cfg.path == "" {
            //     println!("❌ No file or directory has pushed.");
            // } else {
            //     println!("Path: '{}' has dropped.", cfg.path);
            //     cfg.path = "".to_string();
            //     confy::store("canp", cfg).unwrap();
            // }
        }
        Action::List => match fugue::load_config() {
            Ok(config) => {
                println!("{}", config.user_config.box_path);
            }
            Err(e) => {
                println!("{}", e);
            }
        },
        Action::Config => match fugue::load_config() {
            Ok(config) => {
                println!("box_path : {}", config.user_config.box_path);
            }
            Err(e) => {
                println!("{}", e);
            }
        },
        Action::Version => {
            println!("{} {}", fugue::APP_NAME, fugue::get_version());
        }
    }
}
