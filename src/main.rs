mod commands;
mod config;
mod error;
mod fuga;
mod services;
mod traits;
mod ui;

use clap::{ArgGroup, Args, CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::Shell;
use once_cell::sync::Lazy;

// Import new architecture components
use commands::{
    completion::CompletionCommand,
    copy::CopyCommand,
    link::LinkCommand,
    mark::{MarkAction, MarkCommand},
    r#move::MoveCommand,
    Command as FugaCommand,
};
use config::FileConfigRepository;
use services::{StandardFileSystemService, StandardPathService};
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
    command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
    /// Set the path of the target file or directory
    Mark(Mark),
    /// Copy the marked file or directory
    Copy {
        /// The name for the copied file or directory
        #[arg(value_hint = ValueHint::AnyPath)]
        name: Option<String>,
    },
    /// Move the marked file or directory
    Move {
        /// The name for the moved file or directory
        #[arg(value_hint = ValueHint::AnyPath)]
        name: Option<String>,
    },
    /// Make a symbolic link to the marked file or directory
    Link {
        /// The name for the symbolic link
        #[arg(value_hint = ValueHint::AnyPath)]
        name: Option<String>,
    },
    /// Generate the completion script
    Completion {
        /// The shell to generate the script for
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Show the version of the tool
    Version,
}

#[derive(Args, Debug, PartialEq)]
#[command(group(
            ArgGroup::new("mark")
                .required(true)
                .args(&["target", "show", "reset"]),
        ))]
struct Mark {
    /// The path you want to mark
    #[arg(value_hint = ValueHint::AnyPath)]
    target: Option<String>,

    /// Show the marked path
    #[arg(short = 's', long = "show")]
    show: bool,

    /// Reset the marked path
    #[arg(short = 'r', long = "reset")]
    reset: bool,
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
        Commands::Mark(mark) => {
            let action = if mark.show {
                MarkAction::Show
            } else if mark.reset {
                MarkAction::Reset
            } else if let Some(target) = mark.target {
                MarkAction::Set(target)
            } else {
                // Should not happen due to clap validation
                eprintln!("❌ : No mark action specified");
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
        Commands::Copy { name } => {
            let command = CopyCommand::new(
                &services.config_repo,
                &services.fs_service,
                &services.ui_service,
                &services.path_service,
                name,
            );

            execute_command(command)
        }
        Commands::Move { name } => {
            let command = MoveCommand::new(
                &services.config_repo,
                &services.fs_service,
                &services.ui_service,
                &services.path_service,
                name,
            );

            execute_command(command)
        }
        Commands::Link { name } => {
            let command = LinkCommand::new(
                &services.config_repo,
                &services.fs_service,
                &services.ui_service,
                &services.path_service,
                name,
            );

            execute_command(command)
        }
        Commands::Completion { shell } => {
            let cmd = Opt::command();
            let command = CompletionCommand::new(shell, cmd);
            execute_command(command)
        }
        Commands::Version => {
            println!("{}", fuga::get_version());
            Ok(())
        }
    };

    // Handle any errors that occurred during command execution
    if let Err(e) = result {
        eprintln!("❌ : {e}");
        std::process::exit(1);
    }
}
