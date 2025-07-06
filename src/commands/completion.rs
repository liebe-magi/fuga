use crate::commands::{Command, CommandResult};
use clap_complete::{generate, Generator, Shell};

/// Completion command for generating shell completion scripts
pub struct CompletionCommand {
    shell: Shell,
    cmd: clap::Command,
}

impl CompletionCommand {
    pub fn new(shell: Shell, cmd: clap::Command) -> Self {
        Self { shell, cmd }
    }

    fn print_completions<G: Generator>(&self, gen: G, cmd: &mut clap::Command) {
        generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
    }
}

impl Command for CompletionCommand {
    fn execute(&self) -> CommandResult {
        let mut cmd = self.cmd.clone();
        self.print_completions(self.shell, &mut cmd);
        Ok(())
    }
}
