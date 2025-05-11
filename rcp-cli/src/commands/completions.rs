use clap::Command;
use clap_complete::{generate, Shell};
use std::io;

pub fn handle_completions_command(cmd: &mut Command, shell: Shell) -> Result<(), io::Error> {
    generate(shell, cmd, cmd.get_name().to_string(), &mut io::stdout());
    Ok(())
}
