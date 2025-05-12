use crate::cli::Cli;
use crate::error::CliError;
use anyhow::Result;
use clap::{Args, Parser, Subcommand};

/// User command and subcommands for user management operations
#[derive(Parser, Debug)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub subcommand: UserSubcommand,
}

/// User management subcommands
#[derive(Subcommand, Debug)]
pub enum UserSubcommand {
    /// List all users
    #[clap(name = "list")]
    List,
    
    /// Add a new user
    #[clap(name = "add")]
    Add(AddUserArgs),
    
    /// Delete a user
    #[clap(name = "delete")]
    Delete(DeleteUserArgs),
    
    /// Update a user's role
    #[clap(name = "update-role")]
    UpdateRole(UpdateRoleArgs),
    
    /// Reset a user's password (admin only)
    #[clap(name = "reset-password")]
    ResetPassword(ResetPasswordArgs),
}

/// Arguments for adding a user
#[derive(Args, Debug)]
pub struct AddUserArgs {
    /// Username
    #[clap(name = "username")]
    pub username: String,
    
    /// Password (will prompt if not provided)
    #[clap(name = "password", long)]
    pub password: Option<String>,
    
    /// Role (admin, user, guest)
    #[clap(name = "role", long, default_value = "user")]
    pub role: String,
}

/// Arguments for deleting a user
#[derive(Args, Debug)]
pub struct DeleteUserArgs {
    /// Username
    #[clap(name = "username")]
    pub username: String,
}

/// Arguments for updating a user's role
#[derive(Args, Debug)]
pub struct UpdateRoleArgs {
    /// Username
    #[clap(name = "username")]
    pub username: String,
    
    /// New role (admin, user, guest)
    #[clap(name = "role")]
    pub role: String,
}

/// Arguments for resetting a user's password
#[derive(Args, Debug)]
pub struct ResetPasswordArgs {
    /// Username
    #[clap(name = "username")]
    pub username: String,
    
    /// New password (will prompt if not provided)
    #[clap(name = "password", long)]
    pub password: Option<String>,
}

/// Handle user management commands
#[allow(dead_code)]
pub async fn handle_user_command(cli: &mut Cli, cmd: &UserCommand) -> Result<(), CliError> {
    match &cmd.subcommand {
        UserSubcommand::List => {
            list_users(cli).await?;
        }
        UserSubcommand::Add(args) => {
            add_user(cli, args).await?;
        }
        UserSubcommand::Delete(args) => {
            delete_user(cli, args).await?;
        }
        UserSubcommand::UpdateRole(args) => {
            update_role(cli, args).await?;
        }
        UserSubcommand::ResetPassword(args) => {
            reset_password(cli, args).await?;
        }
    }
    
    Ok(())
}

/// List all users
async fn list_users(cli: &mut Cli) -> Result<(), CliError> {
    let users = cli.list_users().await?;
    
    if users.is_empty() {
        println!("No users found");
    } else {
        println!("{:<36} {:<20} {:<10}", "ID", "Username", "Role");
        println!("{}", "-".repeat(70));
        
        for user in users {
            println!("{:<36} {:<20} {:<10}", user.id, user.username, user.role);
        }
    }
    
    Ok(())
}

/// Add a new user
async fn add_user(cli: &mut Cli, args: &AddUserArgs) -> Result<(), CliError> {
    let password = match &args.password {
        Some(p) => p.clone(),
        None => {
            // Prompt for password
            crate::utils::prompt("Password", None)?
        }
    };
    
    // Validate the password
    if password.len() < 8 {
        return Err(CliError::InvalidArgument(
            "Password must be at least 8 characters".to_string(),
        ));
    }
    
    // Confirm the password
    if args.password.is_none() {
        let confirm = crate::utils::prompt("Confirm password", None)?;
        if confirm != password {
            return Err(CliError::InvalidArgument("Passwords do not match".to_string()));
        }
    }
    
    cli.add_user(&args.username, &password, &args.role).await?;
    println!("User '{}' added successfully with role '{}'", args.username, args.role);
    
    Ok(())
}

/// Delete a user
async fn delete_user(cli: &mut Cli, args: &DeleteUserArgs) -> Result<(), CliError> {
    // Ask for confirmation
    let confirm = crate::utils::prompt(
        &format!("Are you sure you want to delete user '{}'? (y/N)", args.username),
        Some("N"),
    )?;
    
    if !confirm.eq_ignore_ascii_case("y") && !confirm.eq_ignore_ascii_case("yes") {
        println!("Operation cancelled");
        return Ok(());
    }
    
    cli.delete_user(&args.username).await?;
    println!("User '{}' deleted successfully", args.username);
    
    Ok(())
}

/// Update a user's role
async fn update_role(cli: &mut Cli, args: &UpdateRoleArgs) -> Result<(), CliError> {
    cli.update_user_role(&args.username, &args.role).await?;
    println!("Updated role for user '{}' to '{}'", args.username, args.role);
    
    Ok(())
}

/// Reset a user's password
async fn reset_password(cli: &mut Cli, args: &ResetPasswordArgs) -> Result<(), CliError> {
    let password = match &args.password {
        Some(p) => p.clone(),
        None => {
            // Prompt for password
            crate::utils::prompt("New password", None)?
        }
    };
    
    // Validate the password
    if password.len() < 8 {
        return Err(CliError::InvalidArgument(
            "Password must be at least 8 characters".to_string(),
        ));
    }
    
    // Confirm the password
    if args.password.is_none() {
        let confirm = crate::utils::prompt("Confirm new password", None)?;
        if confirm != password {
            return Err(CliError::InvalidArgument("Passwords do not match".to_string()));
        }
    }
    
    cli.reset_user_password(&args.username, &password).await?;
    println!("Password for user '{}' reset successfully", args.username);
    
    Ok(())
}
