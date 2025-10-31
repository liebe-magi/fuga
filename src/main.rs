mod commands;
mod config;
mod error;
mod fuga;
mod services;
mod traits;
mod tui;
mod ui;

use clap::{Args, CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::Shell;
use once_cell::sync::Lazy;

// Import new architecture components
use commands::{
    completion::CompletionCommand,
    copy::CopyCommand,
    link::LinkCommand,
    mark::{MarkAction, MarkCommand},
    preset::{PresetAction, PresetCommand},
    r#move::MoveCommand,
    Command as FugaCommand,
};
use config::FileConfigRepository;
use services::{StandardFileSystemService, StandardPathService};
use tui::dashboard::{run_dashboard, DashboardExit};
use ui::TerminalUIService;

static VERSION: Lazy<String> = Lazy::new(fuga::get_version);

#[derive(Parser, Debug, PartialEq)]
#[command(
    name = fuga::APP_NAME,
    author = "liebe-magi <liebe.magi@gmail.com>",
    version = &**VERSION,
    about = "A CLI tool to operate files or directories in 2 steps."
)]
struct Opt {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
    /// Set the path of the target file or directory
    Mark(Mark),
    /// Copy the marked file or directory
    Copy {
        /// The name for the copied file or directory
        #[arg(value_hint = ValueHint::AnyPath, value_name = "DESTINATION")]
        destination: Option<String>,
    },
    /// Move the marked file or directory
    Move {
        /// The name for the moved file or directory
        #[arg(value_hint = ValueHint::AnyPath, value_name = "DESTINATION")]
        destination: Option<String>,
    },
    /// Make a symbolic link to the marked file or directory
    Link {
        /// The name for the symbolic link
        #[arg(value_hint = ValueHint::AnyPath, value_name = "DESTINATION")]
        destination: Option<String>,
    },
    /// Generate the completion script
    Completion {
        /// The shell to generate the script for
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Show the version of the tool
    Version,
    /// Manage mark presets
    Preset {
        #[command(subcommand)]
        command: PresetSubcommands,
    },
}

#[derive(Args, Debug, PartialEq)]
struct Mark {
    /// Paths you want to mark
    #[arg(value_hint = ValueHint::AnyPath, value_name = "PATH", num_args = 0.., conflicts_with_all = ["list", "reset"])]
    paths: Vec<String>,

    /// Add the provided paths to the existing mark list
    #[arg(long = "add", conflicts_with_all = ["list", "reset"])]
    add: bool,

    /// List the marked targets
    #[arg(long = "list", conflicts_with = "reset")]
    list: bool,

    /// Reset the mark list
    #[arg(long = "reset", conflicts_with = "list")]
    reset: bool,
}

#[derive(Subcommand, Debug, PartialEq)]
enum PresetSubcommands {
    /// Save the current mark list to the named preset
    Save {
        /// Preset name to create or overwrite
        #[arg(value_name = "NAME")]
        name: String,
    },
    /// Load the named preset into the mark list
    Load {
        /// Preset name to load
        #[arg(value_name = "NAME")]
        name: String,
    },
    /// List available presets
    List,
    /// Show the contents of a preset
    Show {
        /// Preset name to display
        #[arg(value_name = "NAME")]
        name: String,
    },
    /// Delete a preset
    Delete {
        /// Preset name to delete
        #[arg(value_name = "NAME")]
        name: String,
    },
}

/// Initialize all services for dependency injection
struct ServiceContainer {
    config_repo: FileConfigRepository,
    fs_service: StandardFileSystemService,
    ui_service: TerminalUIService,
    path_service: StandardPathService,
}

impl ServiceContainer {
    fn new() -> Self {
        Self {
            config_repo: FileConfigRepository::new(),
            fs_service: StandardFileSystemService::new(),
            ui_service: TerminalUIService::new(),
            path_service: StandardPathService::new(),
        }
    }
}

/// Execute a command using the new architecture
fn execute_command<T: FugaCommand>(command: T) -> Result<(), crate::error::FugaError> {
    command.execute()
}

fn main() {
    let opt = Opt::parse();
    let services = ServiceContainer::new();

    let result = match opt.command {
        Some(Commands::Mark(mark)) => {
            let action = if mark.list {
                MarkAction::List
            } else if mark.reset {
                MarkAction::Reset
            } else if mark.add {
                if mark.paths.is_empty() {
                    eprintln!("❌ : --add requires at least one path to mark");
                    std::process::exit(1);
                }
                MarkAction::Add(mark.paths)
            } else if !mark.paths.is_empty() {
                MarkAction::Set(mark.paths)
            } else {
                eprintln!(
                    "❌ : Provide at least one path, --add with paths, or use --list/--reset"
                );
                std::process::exit(1);
            };

            let command = MarkCommand::new(
                &services.config_repo,
                &services.fs_service,
                &services.ui_service,
                action,
            );

            execute_command(command)
        }
        Some(Commands::Copy { destination }) => {
            let command = CopyCommand::new(
                &services.config_repo,
                &services.fs_service,
                &services.ui_service,
                &services.path_service,
                destination,
            );

            execute_command(command)
        }
        Some(Commands::Move { destination }) => {
            let command = MoveCommand::new(
                &services.config_repo,
                &services.fs_service,
                &services.ui_service,
                &services.path_service,
                destination,
            );

            execute_command(command)
        }
        Some(Commands::Link { destination }) => {
            let command = LinkCommand::new(
                &services.config_repo,
                &services.fs_service,
                &services.ui_service,
                &services.path_service,
                destination,
            );

            execute_command(command)
        }
        Some(Commands::Completion { shell }) => {
            let cmd = Opt::command();
            let command = CompletionCommand::new(shell, cmd);
            execute_command(command)
        }
        Some(Commands::Preset {
            command: subcommand,
        }) => {
            let action = match subcommand {
                PresetSubcommands::Save { name } => PresetAction::Save { name },
                PresetSubcommands::Load { name } => PresetAction::Load { name },
                PresetSubcommands::List => PresetAction::List,
                PresetSubcommands::Show { name } => PresetAction::Show { name },
                PresetSubcommands::Delete { name } => PresetAction::Delete { name },
            };
            let command = PresetCommand::new(
                &services.config_repo,
                &services.fs_service,
                &services.ui_service,
                action,
            );
            execute_command(command)
        }
        Some(Commands::Version) => {
            println!("{}", fuga::get_version());
            Ok(())
        }
        None => match run_dashboard(&services.config_repo, &services.fs_service) {
            Ok(DashboardExit::Quit) => Ok(()),
            Ok(DashboardExit::Copy(dest)) => {
                let destination = Some(dest.to_string_lossy().into_owned());
                let command = CopyCommand::new(
                    &services.config_repo,
                    &services.fs_service,
                    &services.ui_service,
                    &services.path_service,
                    destination,
                );
                execute_command(command)
            }
            Ok(DashboardExit::Move(dest)) => {
                let destination = Some(dest.to_string_lossy().into_owned());
                let command = MoveCommand::new(
                    &services.config_repo,
                    &services.fs_service,
                    &services.ui_service,
                    &services.path_service,
                    destination,
                );
                execute_command(command)
            }
            Ok(DashboardExit::Link(dest)) => {
                let destination = Some(dest.to_string_lossy().into_owned());
                let command = LinkCommand::new(
                    &services.config_repo,
                    &services.fs_service,
                    &services.ui_service,
                    &services.path_service,
                    destination,
                );
                execute_command(command)
            }
            Err(err) => Err(err),
        },
    };

    // Handle any errors that occurred during command execution
    if let Err(e) = result {
        eprintln!("❌ : {e}");
        std::process::exit(1);
    }
}
