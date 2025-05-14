//! CLI tests for rcpd
//!
//! These tests verify that the CLI command structure works as expected.
//! They test the subcommands and argv parsing, but don't exercise functionality.

#[cfg(feature = "cli")]
mod cli_tests {
    use clap::Parser;
    use rcpd::cli::types::{
        AppCommand, Cli, ConfigCommand, DiagCommand, RcpdCommand, ServerCommand,
        ServiceCommand, SessionCommand, UserCommand,
    };

    #[test]
    fn test_parse_daemon_command() {
        let cli = Cli::parse_from(&["rcpd", "daemon"]);
        match cli.command {
            Some(RcpdCommand::Daemon { command: None }) => {}
            _ => panic!("Expected Daemon command"),
        }
    }

    #[test]
    fn test_parse_server_command() {
        let cli = Cli::parse_from(&["rcpd", "server", "status"]);
        match cli.command {
            Some(RcpdCommand::Server { command }) => {
                assert!(matches!(command, ServerCommand::Status));
            }
            _ => panic!("Expected Server command"),
        }
    }

    #[test]
    fn test_parse_service_command() {
        let cli = Cli::parse_from(&["rcpd", "service", "status"]);
        match cli.command {
            Some(RcpdCommand::Service { command }) => {
                assert!(matches!(command, ServiceCommand::Status));
            }
            _ => panic!("Expected Service command"),
        }
    }

    #[test]
    fn test_parse_app_command() {
        let cli = Cli::parse_from(&["rcpd", "app", "list"]);
        match cli.command {
            Some(RcpdCommand::App { command }) => {
                assert!(matches!(command, AppCommand::List));
            }
            _ => panic!("Expected App command"),
        }
    }

    #[test]
    fn test_parse_session_command() {
        let cli = Cli::parse_from(&["rcpd", "session", "list"]);
        match cli.command {
            Some(RcpdCommand::Session { command }) => {
                assert!(matches!(command, SessionCommand::List));
            }
            _ => panic!("Expected Session command"),
        }
    }

    #[test]
    fn test_parse_user_command() {
        let cli = Cli::parse_from(&["rcpd", "user", "list"]);
        match cli.command {
            Some(RcpdCommand::User { command }) => {
                assert!(matches!(command, UserCommand::List));
            }
            _ => panic!("Expected User command"),
        }
    }

    #[test]
    fn test_parse_config_command() {
        let cli = Cli::parse_from(&["rcpd", "config", "get", "server.port"]);
        match cli.command {
            Some(RcpdCommand::Config { command }) => {
                assert!(matches!(command, ConfigCommand::Get { key } if key == "server.port"));
            }
            _ => panic!("Expected Config command"),
        }
    }

    #[test]
    fn test_parse_diag_command() {
        let cli = Cli::parse_from(&["rcpd", "diag", "system"]);
        match cli.command {
            Some(RcpdCommand::Diag { command }) => {
                assert!(matches!(command, DiagCommand::System));
            }
            _ => panic!("Expected Diag command"),
        }
    }

    #[test]
    fn test_parse_completions_command() {
        let cli = Cli::parse_from(&["rcpd", "completions", "bash"]);
        match cli.command {
            Some(RcpdCommand::Completions { shell }) => {
                assert_eq!(shell.to_string(), "bash");
            }
            _ => panic!("Expected Completions command"),
        }
    }

    // Types are already imported at the top of the module
}
