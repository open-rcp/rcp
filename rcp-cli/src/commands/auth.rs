use crate::error::CliError;

pub async fn handle_auth_command(client: &mut crate::service::ServiceClient, subcommand: &str) -> Result<(), CliError> {
    match subcommand {
        "login" => handle_login(client).await,
        "logout" => handle_logout(client).await,
        _ => Err(CliError::InvalidArgument("Unknown auth command".to_string()))
    }
}

async fn handle_login(client: &mut crate::service::ServiceClient) -> Result<(), CliError> {
    // TODO: Implement login
    Ok(())
}

async fn handle_logout(client: &mut crate::service::ServiceClient) -> Result<(), CliError> {
    // TODO: Implement logout
    Ok(())
}
