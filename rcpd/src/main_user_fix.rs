#[cfg(feature = "cli")]
async fn handle_user_command(command: UserCommand, _config: &config::ServiceConfig, json_output: bool) -> Result<()> {
    use crate::cli::utils::OutputFormatter;
    use crate::cli::service::ServiceClient;
    
    let formatter = OutputFormatter::new(json_output, true, false);
    let client = ServiceClient::new("localhost".to_string(), 8080);
    
    match command {
        UserCommand::List => {
            crate::cli::commands::user::handle_list(&client, &formatter).await?;
        },
        UserCommand::Info { user } => {
            crate::cli::commands::user::handle_info(&user, &client, &formatter).await?;
        },
        UserCommand::Create { username, password, admin } => {
            crate::cli::commands::user::handle_create(&username, &password, admin, &client, &formatter).await?;
        },
        UserCommand::Delete { user } => {
            crate::cli::commands::user::handle_delete(&user, &client, &formatter).await?;
        },
        UserCommand::SetPassword { user_id, password } => {
            crate::cli::commands::user::handle_set_password(&user_id, &password, &client, &formatter).await?;
        }
    }
    
    Ok(())
}
