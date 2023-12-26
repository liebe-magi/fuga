mod fuga;

use clap::{ArgGroup, Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = fuga::APP_NAME,
    author = "liebe-magi <liebe.magi@gmail.com>",
    version = fuga::VERSION,
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
    format!(
        "{} ",
        emojis::get_by_shortcode("information_source").unwrap()
    )
}

fn main() {
    let cli = AppArg::parse();
    match cli.action {
        Action::Mark(mark) => {
            if mark.show {
                // show the marked path
                let target = match fuga::get_marked_path() {
                    Ok(target) => target,
                    Err(e) => {
                        panic!("{} : {}", get_icon_information(), e);
                    }
                };
                if target.is_empty() {
                    println!("{} : No path has been marked.", get_icon_information());
                } else {
                    match fuga::get_file_type(&target) {
                        fuga::TargetType::None => {
                            println!("{} : ❓ {}", get_icon_information(), target)
                        }
                        _ => println!(
                            "{} : {} {}",
                            get_icon_information(),
                            fuga::get_icon(&target),
                            target
                        ),
                    }
                }
            };
            if mark.reset {
                // Reset the target
                match fuga::reset_mark() {
                    Ok(()) => println!("✅ : The marked path has reset."),
                    Err(e) => println!("❌ : {e}"),
                }
            };
            if let Some(target) = mark.target {
                // Set the target
                match fuga::get_file_type(&target) {
                    fuga::TargetType::None => {
                        println!(
                            "❌ : {} is not found.",
                            fuga::get_colorized_text(&target, true)
                        );
                    }
                    _ => {
                        let abs_path = fuga::get_abs_path(&target);
                        match fuga::store_path(&abs_path) {
                            Ok(_) => {
                                println!(
                                    "✅ : {} {} has marked.",
                                    fuga::get_icon(&target),
                                    fuga::get_colorized_text(&target, true)
                                );
                            }
                            Err(e) => println!("❌ : {e}"),
                        }
                    }
                }
            }
        }
        Action::Copy { name } => {
            // show the marked path
            let target = match fuga::get_marked_path() {
                Ok(target) => target,
                Err(e) => {
                    panic!("❌ : {e}");
                }
            };
            match fuga::get_file_type(&target) {
                fuga::TargetType::None => {
                    if target.is_empty() {
                        println!("❌ : No path has been marked.");
                    } else {
                        println!("❌ : {target} is not found.");
                    }
                }
                _ => {
                    // Copy the files or directories
                    let dst_name = match name {
                        Some(name) => name,
                        None => fuga::get_name(&target),
                    };
                    let dst_name = match fuga::get_file_type(&dst_name) {
                        fuga::TargetType::Dir => {
                            format!("{}/{}", dst_name, fuga::get_name(&target))
                        }
                        _ => dst_name,
                    };
                    println!(
                        "{} : Start copying {} {} from {}",
                        get_icon_information(),
                        fuga::get_icon(&target),
                        fuga::get_colorized_text(&dst_name, true),
                        target
                    );
                    match fuga::copy_items(&target, &dst_name) {
                        Ok(_) => {
                            println!(
                                "✅ : {} {} has copied.",
                                fuga::get_icon(&dst_name),
                                fuga::get_colorized_text(&dst_name, true)
                            );
                        }
                        Err(e) => println!("❌ : {e}"),
                    }
                }
            }
        }
        Action::Move { name } => {
            // show the marked path
            let target = match fuga::get_marked_path() {
                Ok(target) => target,
                Err(e) => {
                    panic!("❌ : {e}");
                }
            };
            match fuga::get_file_type(&target) {
                fuga::TargetType::None => {
                    if target.is_empty() {
                        println!("❌ : No path has been marked.");
                    } else {
                        println!("❌ : {target} is not found.");
                    }
                }
                _ => {
                    // Move the files or directories
                    let dst_name = match name {
                        Some(name) => name,
                        None => fuga::get_name(&target),
                    };
                    let dst_name = match fuga::get_file_type(&dst_name) {
                        fuga::TargetType::Dir => {
                            format!("{}/{}", dst_name, fuga::get_name(&target))
                        }
                        _ => dst_name,
                    };
                    println!(
                        "{} : Start moving {} {} from {}",
                        get_icon_information(),
                        fuga::get_icon(&target),
                        fuga::get_colorized_text(&dst_name, true),
                        target
                    );
                    match fuga::move_items(&target, &dst_name) {
                        Ok(_) => {
                            println!(
                                "✅ : {} {} has moved.",
                                fuga::get_icon(&dst_name),
                                fuga::get_colorized_text(&dst_name, true)
                            );
                            match fuga::reset_mark() {
                                Ok(_) => (),
                                Err(e) => println!("❌ : {e}"),
                            }
                        }
                        Err(e) => println!("❌ : {e}"),
                    }
                }
            }
        }
        Action::Link { name } => {
            // show the marked path
            let target = match fuga::get_marked_path() {
                Ok(target) => target,
                Err(e) => {
                    panic!("❌ : {e}");
                }
            };
            match fuga::get_file_type(&target) {
                fuga::TargetType::None => {
                    if target.is_empty() {
                        println!("❌ : No path has been marked.");
                    } else {
                        println!("❌ : {target} is not found.");
                    }
                }
                _ => {
                    // Move the files or directories
                    let dst_name = match name {
                        Some(name) => name,
                        None => fuga::get_name(&target),
                    };
                    let dst_name = match fuga::get_file_type(&dst_name) {
                        fuga::TargetType::Dir => {
                            format!("{}/{}", dst_name, fuga::get_name(&target))
                        }
                        _ => dst_name,
                    };
                    println!(
                        "{} : Start making symbolic link {} {} from {}",
                        get_icon_information(),
                        fuga::get_icon(&target),
                        fuga::get_colorized_text(&dst_name, true),
                        target
                    );
                    match fuga::link_items(&target, &dst_name) {
                        Ok(_) => {
                            println!(
                                "✅ : {} {} has made.",
                                fuga::get_icon(&dst_name),
                                fuga::get_colorized_text(&dst_name, true)
                            );
                        }
                        Err(e) => println!("❌ : {e}"),
                    }
                }
            }
        }
        Action::Version => {
            println!("{} {}", fuga::APP_NAME, fuga::get_version());
        }
    }
}
