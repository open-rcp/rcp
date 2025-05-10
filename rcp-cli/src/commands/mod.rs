pub mod service;
pub mod server;
pub mod session;
pub mod user;
pub mod config;
pub mod diag;
pub mod logs;
pub mod auth;
pub mod shell;
pub mod batch;
pub mod completions;

// Re-export handlers for easier importing
pub use service::handle_service_command;
pub use auth::handle_auth_command;
pub use shell::handle_shell_command;
pub use batch::handle_batch_command;
pub use completions::handle_completions_command;