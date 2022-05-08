mod fugue;

use clap::{ArgGroup, Args, Parser, Subcommand};
use emojis;

// const INDEX_RANGE: RangeInclusive<usize> = 0..=9;

#[derive(Parser)]
#[clap(
    name = fugue::APP_NAME,
    author = "MagicalLiebe <magical.liebe@gmail.com>",
    version = fugue::VERSION,
    about = "A CLI tool to operate files or directories in 2 steps."
)]
struct AppArg {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Set the path of the target file or directory
    Mark(Mark),
    /// Copy the marked files or directories
    Copy {
        /// The name for the copied files or directories
        name: Option<String>,
    },
    /// Move the marked files or directories
    Move {
        /// The name for the moved files or directories
        name: Option<String>,
    },
    /// Make a symbolic link of the marked files or directories
    Link {
        /// The name for the linked files or directories
        name: Option<String>,
    },
    // Create a new box
    // Create {
    // The name of the box to create
    // box_name: String,
    // },
    // Delete the box
    // Delete {
    // The name of the box to delete
    // box_name: String,
    // },
    // Clean up all the items in the box
    // Clean {
    // The name of the box to clean up
    // box_name: Option<String>,
    // },
    // Show the list of all the boxes or the marked path
    // Show,
    // Add a file or directory path to the temp
    // Add(Add),
    // Pop a file or directory from the temp
    // Pop(Pop),
    // Drop a file or directory from the temp
    // Drop,
    // List all boxes
    // List,
    // Show the config
    // Config,
    /// Show the version of the tool
    Version,
}

#[derive(Args)]
#[clap(group(
            ArgGroup::new("mark")
                .required(true)
                .args(&["target", "show", "reset"]),
        ))]
struct Mark {
    /// The path that you want to mark
    target: Option<String>,

    /// Show the marked path
    #[clap(short = 's', long = "show")]
    show: bool,

    #[clap(short = 'r', long = "reset")]
    reset: bool,
}

// #[derive(Args)]
// struct Add {
//     /// The target path (file or directory).
//     path: String,
// }

// #[derive(Args)]
// struct Pop {
//     /// Delete a file or directory after copying
//     #[clap(short = 'm', long = "move")]
//     is_move: bool,

//     /// Index of
//     #[clap(short = 'i', long = "index", validator = index_in_range, default_value = "0")]
//     index: usize,

//     /// The destination path to be copied or moved.
//     path: Option<String>,
// }

// fn index_in_range(s: &str) -> Result<(), String> {
//     usize::from_str(s)
//         .map(|index| INDEX_RANGE.contains(&index))
//         .map_err(|e| e.to_string())
//         .and_then(|result| match result {
//             true => Ok(()),
//             false => Err(format!(
//                 "Index not in range {}-{}",
//                 INDEX_RANGE.start(),
//                 INDEX_RANGE.end()
//             )),
//         })
// }

fn get_icon_information() -> String {
    return format!(
        "{} ",
        emojis::get_by_shortcode("information_source").unwrap()
    );
}

fn main() {
    let cli = AppArg::parse();
    match cli.action {
        Action::Mark(mark) => {
            if mark.show {
                // show the marked path
                let target = match fugue::get_marked_path() {
                    Ok(target) => target,
                    Err(e) => {
                        panic!("{} : {}", get_icon_information(), e);
                    }
                };
                if target.is_empty() {
                    println!("{} : No path has been marked.", get_icon_information());
                } else {
                    match fugue::get_file_type(&target) {
                        fugue::TargetType::None => {
                            println!("{} : ❓ {}", get_icon_information(), target)
                        }
                        _ => println!(
                            "{} : {} {}",
                            get_icon_information(),
                            fugue::get_icon(&target),
                            target
                        ),
                    }
                }
            };
            if mark.reset {
                // Reset the target
                match fugue::reset_mark() {
                    Ok(()) => println!("✅ : The marked path has reset."),
                    Err(e) => println!("❌ : {}", e),
                }
            };
            match mark.target {
                Some(target) => {
                    // Set the target
                    match fugue::get_file_type(&target) {
                        fugue::TargetType::None => {
                            println!(
                                "❌ : {} is not found.",
                                fugue::get_colorized_text(&target, true)
                            );
                        }
                        _ => {
                            let abs_path = fugue::get_abs_path(&target);
                            match fugue::store_path(&abs_path) {
                                Ok(_) => {
                                    println!(
                                        "✅ : {} {} has marked.",
                                        fugue::get_icon(&target),
                                        fugue::get_colorized_text(&target, true)
                                    );
                                }
                                Err(e) => println!("❌ : {}", e),
                            }
                        }
                    }
                }
                None => (),
            }
        }
        Action::Copy { name } => {
            // show the marked path
            let target = match fugue::get_marked_path() {
                Ok(target) => target,
                Err(e) => {
                    panic!("❌ : {}", e);
                }
            };
            match fugue::get_file_type(&target) {
                fugue::TargetType::None => {
                    if target.is_empty() {
                        println!("❌ : No path has been marked.");
                    } else {
                        println!("❌ : {} is not found.", target);
                    }
                }
                _ => {
                    // Copy the files or directories
                    let dst_name = match name {
                        Some(name) => name,
                        None => fugue::get_name(&target),
                    };
                    let dst_name = match fugue::get_file_type(&dst_name) {
                        fugue::TargetType::Dir => {
                            format!("{}/{}", dst_name, fugue::get_name(&target))
                        }
                        _ => dst_name,
                    };
                    println!(
                        "{} : Start copying {} {} from {}",
                        get_icon_information(),
                        fugue::get_icon(&target),
                        fugue::get_colorized_text(&dst_name, true),
                        target
                    );
                    match fugue::copy_items(&target, &dst_name) {
                        Ok(_) => {
                            println!(
                                "✅ : {} {} has copied.",
                                fugue::get_icon(&dst_name),
                                fugue::get_colorized_text(&dst_name, true)
                            );
                        }
                        Err(e) => println!("❌ : {}", e),
                    }
                }
            }
        }
        Action::Move { name } => {
            // show the marked path
            let target = match fugue::get_marked_path() {
                Ok(target) => target,
                Err(e) => {
                    panic!("❌ : {}", e);
                }
            };
            match fugue::get_file_type(&target) {
                fugue::TargetType::None => {
                    if target.is_empty() {
                        println!("❌ : No path has been marked.");
                    } else {
                        println!("❌ : {} is not found.", target);
                    }
                }
                _ => {
                    // Move the files or directories
                    let dst_name = match name {
                        Some(name) => name,
                        None => fugue::get_name(&target),
                    };
                    let dst_name = match fugue::get_file_type(&dst_name) {
                        fugue::TargetType::Dir => {
                            format!("{}/{}", dst_name, fugue::get_name(&target))
                        }
                        _ => dst_name,
                    };
                    println!(
                        "{} : Start moving {} {} from {}",
                        get_icon_information(),
                        fugue::get_icon(&target),
                        fugue::get_colorized_text(&dst_name, true),
                        target
                    );
                    match fugue::move_items(&target, &dst_name) {
                        Ok(_) => {
                            println!(
                                "✅ : {} {} has moved.",
                                fugue::get_icon(&dst_name),
                                fugue::get_colorized_text(&dst_name, true)
                            );
                            match fugue::reset_mark() {
                                Ok(_) => (),
                                Err(e) => println!("❌ : {}", e),
                            }
                        }
                        Err(e) => println!("❌ : {}", e),
                    }
                }
            }
        }
        Action::Link { name } => {
            // show the marked path
            let target = match fugue::get_marked_path() {
                Ok(target) => target,
                Err(e) => {
                    panic!("❌ : {}", e);
                }
            };
            match fugue::get_file_type(&target) {
                fugue::TargetType::None => {
                    if target.is_empty() {
                        println!("❌ : No path has been marked.");
                    } else {
                        println!("❌ : {} is not found.", target);
                    }
                }
                _ => {
                    // Move the files or directories
                    let dst_name = match name {
                        Some(name) => name,
                        None => fugue::get_name(&target),
                    };
                    let dst_name = match fugue::get_file_type(&dst_name) {
                        fugue::TargetType::Dir => {
                            format!("{}/{}", dst_name, fugue::get_name(&target))
                        }
                        _ => dst_name,
                    };
                    println!(
                        "{} : Start making symbolic link {} {} from {}",
                        get_icon_information(),
                        fugue::get_icon(&target),
                        fugue::get_colorized_text(&dst_name, true),
                        target
                    );
                    match fugue::link_items(&target, &dst_name) {
                        Ok(_) => {
                            println!(
                                "✅ : {} {} has made.",
                                fugue::get_icon(&dst_name),
                                fugue::get_colorized_text(&dst_name, true)
                            );
                        }
                        Err(e) => println!("❌ : {}", e),
                    }
                }
            }
        }
        // Action::Create { box_name } => {
        //     println!(
        //         "✨ Created the new box : 📦️ {}",
        //         fugue::get_colorized_text(&box_name, true)
        //     );
        // }
        // Action::Delete { box_name } => {
        //     println!(
        //         "🔥 Deleted the box : 📦️ {}",
        //         fugue::get_colorized_text(&box_name, true)
        //     );
        // }
        // Action::Clean { box_name } => {
        //     match box_name {
        //         Some(box_name) => println!(
        //             "🧹 Cleaned all the items in the box : 📦️ {}",
        //             fugue::get_colorized_text(&box_name, true)
        //         ),
        //         None => println!("🧹 Cleaned all the items in the temp"),
        //     }
        //     // let path_list = get_path_list();
        //     // let before_len = path_list.len();
        //     // let mut clean_path_list = Vec::new();
        //     // for path in path_list {
        //     //     if is_exist(&path) {
        //     //         clean_path_list.push(path);
        //     //     }
        //     // }
        //     // if before_len - clean_path_list.len() > 0 {
        //     //     println!(
        //     //         "Clean: 🧹 {} paths have been cleaned.",
        //     //         before_len - clean_path_list.len()
        //     //     );
        //     //     save_path(clean_path_list);
        //     //     show_path_list();
        //     // } else {
        //     //     println!("Clean: 🧹 No path has been cleaned.");
        //     // }
        // }
        // Action::Show { is_mark } => match fugue::load_config() {
        //     Ok(config) => {
        //         if is_mark {
        //             if config.data.target.is_empty() {
        //                 println!("❌ No path is marked.");
        //             } else {
        //                 match fugue::get_file_type(&config.data.target) {
        //                     fugue::TargetType::File => {
        //                         println!("Marked path: 📄 {}", config.data.target);
        //                     }
        //                     fugue::TargetType::Dir => {
        //                         println!("Marked path: 📁 {}", config.data.target);
        //                     }
        //                     fugue::TargetType::None => {
        //                         println!("Marked path: ❌ {}", config.data.target);
        //                     }
        //                 }
        //             }
        //         } else {
        //             println!("List of the boxes.")
        //         }
        //     }
        //     Err(e) => println!("❌ {}", e),
        // },
        // Action::Add(add) => {
        //     println!("Adding {} to box", add.path);
        //     // let abs_path = organize_path(&add.path);
        //     // match get_file_type(&abs_path) {
        //     //     FileType::File => {
        //     //         add_path(&abs_path);
        //     //         println!("Add: 📄 {}", &abs_path);
        //     //     }
        //     //     FileType::Dir => {
        //     //         add_path(&abs_path);
        //     //         println!("Add: 📁 {}", abs_path);
        //     //     }
        //     //     FileType::None => {
        //     //         println!("Failed: ❌ \"{}\" is not exist.", abs_path);
        //     //     }
        //     // }
        // }
        // Action::Pop(pop) => {
        //     println!("Popping {} from box", pop.path.unwrap_or("".to_string()));
        //     // let mut cfg = confy::load::<AppConfig>("canp").unwrap_or_default();
        //     // if cfg.path == "" {
        //     //     println!("❌ No file or directory has pushed.");
        //     // } else {
        //     //     let file_name = pop.path.unwrap_or(get_file_name(&cfg.path));
        //     //     let dir_name = get_dir_name(&file_name);
        //     //     if dir_name != "" {
        //     //         if !is_exist(&dir_name) {
        //     //             println!("Dir: {} is not exist", dir_name);
        //     //             return;
        //     //         }
        //     //     }
        //     //     if is_exist(&cfg.path) {
        //     //         if is_file(&cfg.path) {
        //     //             copy(&cfg.path, &file_name).unwrap();
        //     //             println!("File: '{}' has poped.", &cfg.path);
        //     //         } else {
        //     //             println!("Dir: '{}' has poped.", &cfg.path);
        //     //         }
        //     //         if pop.is_move {
        //     //             println!("move");
        //     //         }
        //     //         cfg.path = "".to_string();
        //     //         confy::store("canp", cfg).unwrap();
        //     //     } else {
        //     //         println!("Failed: ❌ \"{}\" is not exist.", cfg.path);
        //     //     }
        //     // }
        // }
        // Action::Drop => {
        // let mut cfg = confy::load::<AppConfig>("canp").unwrap_or_default();
        // if cfg.path == "" {
        //     println!("❌ No file or directory has pushed.");
        // } else {
        //     println!("Path: '{}' has dropped.", cfg.path);
        //     cfg.path = "".to_string();
        //     confy::store("canp", cfg).unwrap();
        // }
        // }
        // Action::List => match fugue::load_config() {
        //     Ok(config) => {
        //         println!("{}", config.user_config.box_path);
        //     }
        //     Err(e) => {
        //         println!("{}", e);
        //     }
        // },
        // Action::Config => match fugue::load_config() {
        //     Ok(config) => {
        //         println!("box_path : {}", config.user_config.box_path);
        //     }
        //     Err(e) => {
        //         println!("{}", e);
        //     }
        // },
        Action::Version => {
            println!("{} {}", fugue::APP_NAME, fugue::get_version());
        }
    }
}
