use crate::cli::Cli;
use crate::utils;
use anyhow::Result;

/// User action enum (only unit variants for clap's ValueEnum)
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum UserAction {
    List,
    Add,
    Remove,
    UpdateRole,
    ResetPassword,
}

/// Handle user commands with flat parameters
pub async fn handle_user_command(
    cli: &mut Cli,
    action: UserAction,
    username: Option<&str>,
    password: Option<&str>,
    role: Option<&str>,
) -> Result<()> {
    match action {
        UserAction::List => {
            let users = cli.list_users().await?;

            if users.is_empty() {
                println!("No users found");
            } else {
                println!("{:<20} {:<10}", "Username", "Role");
                println!("{}", "-".repeat(40));

                for user in users {
                    println!("{:<20} {:<10}", user.username, user.role);
                }
            }
        }
        UserAction::Add => {
            // Check if username is provided
            let username =
                username.ok_or(anyhow::anyhow!("Username is required for add action"))?;

            // Get password, prompting if not provided
            let password_value = match password {
                Some(p) => p.to_string(),
                None => {
                    // Prompt for password
                    utils::prompt("Password", None)?
                }
            };

            // Validate the password
            if password_value.len() < 8 {
                return Err(anyhow::anyhow!("Password must be at least 8 characters"));
            }

            // Confirm the password if it was prompted
            if password.is_none() {
                let confirm = utils::prompt("Confirm password", None)?;
                if confirm != password_value {
                    return Err(anyhow::anyhow!("Passwords do not match"));
                }
            }

            // Default role to "user" if not provided
            let role_value = role.unwrap_or("user");

            cli.add_user(username, &password_value, role_value).await?;
            println!(
                "User '{}' added successfully with role '{}'",
                username, role_value
            );
        }
        UserAction::Remove => {
            // Check if username is provided
            let username =
                username.ok_or(anyhow::anyhow!("Username is required for remove action"))?;

            // Ask for confirmation
            let confirm = utils::prompt(
                &format!("Are you sure you want to delete user '{}'? (y/N)", username),
                Some("N"),
            )?;

            if !confirm.eq_ignore_ascii_case("y") && !confirm.eq_ignore_ascii_case("yes") {
                println!("Operation cancelled");
                return Ok(());
            }

            cli.delete_user(username).await?;
            println!("User '{}' deleted successfully", username);
        }
        UserAction::UpdateRole => {
            // Check if username and role are provided
            let username = username.ok_or(anyhow::anyhow!(
                "Username is required for update_role action"
            ))?;
            let role_value =
                role.ok_or(anyhow::anyhow!("Role is required for update_role action"))?;

            cli.update_user_role(username, role_value).await?;
            println!("Updated role for user '{}' to '{}'", username, role_value);
        }
        UserAction::ResetPassword => {
            // Check if username is provided
            let username = username.ok_or(anyhow::anyhow!(
                "Username is required for reset_password action"
            ))?;

            // Get password, prompting if not provided
            let password_value = match password {
                Some(p) => p.to_string(),
                None => {
                    // Prompt for password
                    utils::prompt("New password", None)?
                }
            };

            // Validate the password
            if password_value.len() < 8 {
                return Err(anyhow::anyhow!("Password must be at least 8 characters"));
            }

            // Confirm the password if it was prompted
            if password.is_none() {
                let confirm = utils::prompt("Confirm new password", None)?;
                if confirm != password_value {
                    return Err(anyhow::anyhow!("Passwords do not match"));
                }
            }

            cli.reset_user_password(username, &password_value).await?;
            println!("Password for user '{}' reset successfully", username);
        }
    }

    Ok(())
}
