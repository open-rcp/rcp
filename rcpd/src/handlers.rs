// Session handler
#[cfg(feature = "cli")]
async fn handle_session_command(command: SessionCommand, _config: &config::ServiceConfig, json_output: bool) -> Result<()> {
    use crate::cli::utils::OutputFormatter;
    use crate::cli::service::ServiceClient;
    
    let formatter = OutputFormatter::new(json_output, true, false);
    let client = ServiceClient::new("localhost".to_string(), 8080);
    
    match command {
        SessionCommand::List => {
            crate::cli::commands::session::handle_list(&client, &formatter).await?;
        },
        SessionCommand::Info { session_id } => {
            crate::cli::commands::session::handle_info(&session_id, &client, &formatter).await?;
        },
        SessionCommand::Close { session_id } => {
            crate::cli::commands::session::handle_close(&session_id, &client, &formatter).await?;
        }
    }
    
    Ok(())
}

// Configuration handler
#[cfg(feature = "cli")]
async fn handle_config_command(command: ConfigCommand, _config: &config::ServiceConfig, json_output: bool) -> Result<()> {
    use crate::cli::utils::OutputFormatter;
    
    let formatter = OutputFormatter::new(json_output, true, false);
    
    match command {
        ConfigCommand::Show => {
            crate::cli::commands::config::handle_show(&formatter)?;
        },
        ConfigCommand::Set { key, value } => {
            crate::cli::commands::config::handle_set(&key, &value, &formatter)?;
        },
        ConfigCommand::Get { key } => {
            crate::cli::commands::config::handle_get(&key, &formatter)?;
        },
        ConfigCommand::Remove { key } => {
            crate::cli::commands::config::handle_remove(&key, &formatter)?;
        }
    }
    
    Ok(())
}

// User handler
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
