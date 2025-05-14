//! CLI completions command module
//!
//! This module provides CLI command for generating shell completions.

#[cfg(feature = "cli")]
use anyhow::Result;
#[cfg(feature = "cli")]
use clap::CommandFactory;
#[cfg(feature = "cli")]
use std::path::Path;

/// Handle completions command
#[cfg(feature = "cli")]
pub fn handle_completions_command(shell: clap_complete::Shell, dir: Option<&Path>) -> Result<()> {
    // Get command from clap
    let cmd = crate::cli::types::Cli::command();

    // Due to version mismatch between clap 3.x and clap_complete 4.x,
    // we use a simpler approach to generate completions
    if let Some(dir) = dir {
        match shell {
            clap_complete::Shell::Bash => {
                let path = dir.join("rcpd.bash");
                std::fs::write(&path, generate_bash_completions())?;
                println!("Bash completions written to: {}", path.display());
            }
            clap_complete::Shell::Zsh => {
                let path = dir.join("rcpd.zsh");
                std::fs::write(&path, generate_zsh_completions())?;
                println!("Zsh completions written to: {}", path.display());
            }
            clap_complete::Shell::Fish => {
                let path = dir.join("rcpd.fish");
                std::fs::write(&path, generate_fish_completions())?;
                println!("Fish completions written to: {}", path.display());
            }
            _ => {
                println!("Completions for {:?} are not currently supported", shell);
            }
        }
    } else {
        match shell {
            clap_complete::Shell::Bash => {
                println!("{}", generate_bash_completions());
            }
            clap_complete::Shell::Zsh => {
                println!("{}", generate_zsh_completions());
            }
            clap_complete::Shell::Fish => {
                println!("{}", generate_fish_completions());
            }
            _ => {
                println!("Completions for {:?} are not currently supported", shell);
            }
        }
    }

    Ok(())
}

/// Generate bash completions
#[cfg(feature = "cli")]
fn generate_bash_completions() -> String {
    r#"
# RCPD bash completion script
# Add this to your .bashrc or equivalent:
# source /path/to/rcpd.bash

_rcpd() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    
    # Main commands
    opts="help version service config diag completions server session user app"
    
    # Complete based on command or subcommand
    case ${prev} in
        rcpd)
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        service)
            COMPREPLY=( $(compgen -W "status start stop restart install uninstall logs" -- ${cur}) )
            return 0
            ;;
        config)
            COMPREPLY=( $(compgen -W "get set update list" -- ${cur}) )
            return 0
            ;;
        completions)
            COMPREPLY=( $(compgen -W "bash zsh fish" -- ${cur}) )
            return 0
            ;;
        server)
            COMPREPLY=( $(compgen -W "start stop restart status info" -- ${cur}) )
            return 0
            ;;
        session)
            COMPREPLY=( $(compgen -W "list disconnect" -- ${cur}) )
            return 0
            ;;
        user)
            COMPREPLY=( $(compgen -W "list add remove update info" -- ${cur}) )
            return 0
            ;;
        app)
            COMPREPLY=( $(compgen -W "list launch stop" -- ${cur}) )
            return 0
            ;;
    esac
    
    return 0
}

complete -F _rcpd rcpd
"#
    .trim()
    .to_string()
}

/// Generate zsh completions
#[cfg(feature = "cli")]
fn generate_zsh_completions() -> String {
    r#"
#compdef rcpd

_rcpd() {
    local -a commands
    local -a subcommands
    
    commands=(
        'help:Show help information'
        'version:Show version information'
        'service:Manage RCPD service'
        'config:Manage RCPD configuration'
        'diag:Run diagnostics'
        'completions:Generate shell completions'
        'server:Manage server'
        'session:Manage sessions'
        'user:Manage users'
        'app:Manage applications'
    )
    
    if (( CURRENT == 2 )); then
        _describe -t commands 'rcpd commands' commands
        return
    fi
    
    case "$words[2]" in
        service)
            subcommands=(
                'status:Show service status'
                'start:Start the service'
                'stop:Stop the service'
                'restart:Restart the service'
                'install:Install as a service'
                'uninstall:Uninstall service'
                'logs:Show service logs'
            )
            _describe -t subcommands 'service subcommands' subcommands
            ;;
        config)
            subcommands=(
                'get:Get configuration value'
                'set:Set configuration value'
                'update:Update configuration'
                'list:List configuration'
            )
            _describe -t subcommands 'config subcommands' subcommands
            ;;
        completions)
            subcommands=(
                'bash:Generate bash completions'
                'zsh:Generate zsh completions'
                'fish:Generate fish completions'
            )
            _describe -t subcommands 'completions subcommands' subcommands
            ;;
        server)
            subcommands=(
                'start:Start the server'
                'stop:Stop the server'
                'restart:Restart the server'
                'status:Show server status'
                'info:Show server information'
            )
            _describe -t subcommands 'server subcommands' subcommands
            ;;
        session)
            subcommands=(
                'list:List sessions'
                'disconnect:Disconnect session'
            )
            _describe -t subcommands 'session subcommands' subcommands
            ;;
        user)
            subcommands=(
                'list:List users'
                'add:Add a user'
                'remove:Remove a user'
                'update:Update a user'
                'info:Show user information'
            )
            _describe -t subcommands 'user subcommands' subcommands
            ;;
        app)
            subcommands=(
                'list:List applications'
                'launch:Launch an application'
                'stop:Stop an application'
            )
            _describe -t subcommands 'app subcommands' subcommands
            ;;
    esac
}

_rcpd
"#
    .trim()
    .to_string()
}

/// Generate fish completions
#[cfg(feature = "cli")]
fn generate_fish_completions() -> String {
    r#"
# RCPD fish completions

# Main commands
complete -c rcpd -f
complete -c rcpd -n "__fish_use_subcommand" -a help -d "Show help information"
complete -c rcpd -n "__fish_use_subcommand" -a version -d "Show version information"
complete -c rcpd -n "__fish_use_subcommand" -a service -d "Manage RCPD service"
complete -c rcpd -n "__fish_use_subcommand" -a config -d "Manage RCPD configuration"
complete -c rcpd -n "__fish_use_subcommand" -a diag -d "Run diagnostics"
complete -c rcpd -n "__fish_use_subcommand" -a completions -d "Generate shell completions"
complete -c rcpd -n "__fish_use_subcommand" -a server -d "Manage server"
complete -c rcpd -n "__fish_use_subcommand" -a session -d "Manage sessions"
complete -c rcpd -n "__fish_use_subcommand" -a user -d "Manage users"
complete -c rcpd -n "__fish_use_subcommand" -a app -d "Manage applications"

# Service subcommands
complete -c rcpd -n "__fish_seen_subcommand_from service" -a status -d "Show service status"
complete -c rcpd -n "__fish_seen_subcommand_from service" -a start -d "Start the service"
complete -c rcpd -n "__fish_seen_subcommand_from service" -a stop -d "Stop the service"
complete -c rcpd -n "__fish_seen_subcommand_from service" -a restart -d "Restart the service"
complete -c rcpd -n "__fish_seen_subcommand_from service" -a install -d "Install as a service"
complete -c rcpd -n "__fish_seen_subcommand_from service" -a uninstall -d "Uninstall service"
complete -c rcpd -n "__fish_seen_subcommand_from service" -a logs -d "Show service logs"

# Config subcommands
complete -c rcpd -n "__fish_seen_subcommand_from config" -a get -d "Get configuration value"
complete -c rcpd -n "__fish_seen_subcommand_from config" -a set -d "Set configuration value"
complete -c rcpd -n "__fish_seen_subcommand_from config" -a update -d "Update configuration"
complete -c rcpd -n "__fish_seen_subcommand_from config" -a list -d "List configuration"

# Completions subcommands
complete -c rcpd -n "__fish_seen_subcommand_from completions" -a bash -d "Generate bash completions"
complete -c rcpd -n "__fish_seen_subcommand_from completions" -a zsh -d "Generate zsh completions"
complete -c rcpd -n "__fish_seen_subcommand_from completions" -a fish -d "Generate fish completions"

# Server subcommands
complete -c rcpd -n "__fish_seen_subcommand_from server" -a start -d "Start the server"
complete -c rcpd -n "__fish_seen_subcommand_from server" -a stop -d "Stop the server"
complete -c rcpd -n "__fish_seen_subcommand_from server" -a restart -d "Restart the server"
complete -c rcpd -n "__fish_seen_subcommand_from server" -a status -d "Show server status"
complete -c rcpd -n "__fish_seen_subcommand_from server" -a info -d "Show server information"

# Session subcommands
complete -c rcpd -n "__fish_seen_subcommand_from session" -a list -d "List sessions"
complete -c rcpd -n "__fish_seen_subcommand_from session" -a disconnect -d "Disconnect session"

# User subcommands
complete -c rcpd -n "__fish_seen_subcommand_from user" -a list -d "List users"
complete -c rcpd -n "__fish_seen_subcommand_from user" -a add -d "Add a user"
complete -c rcpd -n "__fish_seen_subcommand_from user" -a remove -d "Remove a user"
complete -c rcpd -n "__fish_seen_subcommand_from user" -a update -d "Update a user"
complete -c rcpd -n "__fish_seen_subcommand_from user" -a info -d "Show user information"

# App subcommands
complete -c rcpd -n "__fish_seen_subcommand_from app" -a list -d "List applications"
complete -c rcpd -n "__fish_seen_subcommand_from app" -a launch -d "Launch an application"
complete -c rcpd -n "__fish_seen_subcommand_from app" -a stop -d "Stop an application"
"#
    .trim()
    .to_string()
}

/// Auto-detect current shell and generate completions
#[cfg(feature = "cli")]
pub fn handle_auto_completions(dir: Option<&Path>) -> Result<()> {
    // Try to detect shell from environment
    let shell = if let Ok(shell_env) = std::env::var("SHELL") {
        if shell_env.contains("bash") {
            clap_complete::Shell::Bash
        } else if shell_env.contains("zsh") {
            clap_complete::Shell::Zsh
        } else if shell_env.contains("fish") {
            clap_complete::Shell::Fish
        } else {
            // Default to bash if we can't identify the shell
            clap_complete::Shell::Bash
        }
    } else {
        // Default to bash if SHELL is not set
        clap_complete::Shell::Bash
    };

    handle_completions_command(shell, dir)
}
