mod fugue;

use clap::{ArgGroup, Args, Parser, Subcommand};

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
    /// Copy the marked file or directory
    Copy {
        /// The name for the copied file or directory
        name: Option<String>,
    },
    /// Move the marked file or directory
    Move {
        /// The name for the moved file or directory
        name: Option<String>,
    },
    /// Make a symbolic link to the marked file or directory
    Link {
        /// The name for the symbolic link
        name: Option<String>,
    },
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
    /// The path you want to mark
    target: Option<String>,

    /// Show the marked path
    #[clap(short = 's', long = "show")]
    show: bool,

    /// Reset the marked path
    #[clap(short = 'r', long = "reset")]
    reset: bool,
}

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
        Action::Version => {
            println!("{} {}", fugue::APP_NAME, fugue::get_version());
        }
    }
}
