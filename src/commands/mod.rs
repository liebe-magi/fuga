pub mod completion;
pub mod copy;
pub mod link;
pub mod mark;
pub mod r#move;
pub mod preset;

use crate::error::FugaResult;

/// Trait for command execution
pub trait Command {
    /// Execute the command
    fn execute(&self) -> FugaResult<()>;
}

/// Command execution result
pub type CommandResult = FugaResult<()>;
