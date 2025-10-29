pub mod completion;
pub mod copy;
pub mod link;
pub mod mark;
pub mod r#move;

use crate::error::FugaResult;

/// Trait for command execution
pub trait Command {
    /// Execute the command
    fn execute(&self) -> FugaResult<()>;
}

/// Command execution result
pub type CommandResult = FugaResult<()>;
