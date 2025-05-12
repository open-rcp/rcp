use crate::error::{Error, Result};
use std::str::FromStr;
use url::Url;

/// Represents a parsed RCP connection string in the format:
/// rcp://\[user\[:password\]@\]host\[:port\]\[/path\]
/// or the SSH-like format:
/// \[user\[:password\]@\]host\[:port\]\[/path\]
#[derive(Debug, Clone)]
pub struct ConnectionString {
    /// Username for authentication
    pub username: Option<String>,

    /// Password/PSK for authentication
    pub password: Option<String>,

    /// Host to connect to
    pub host: String,

    /// Port to connect to
    pub port: Option<u16>,

    /// Optional path
    pub path: Option<String>,
}

impl ConnectionString {
    /// Parse a connection string
    pub fn parse(input: &str) -> Result<Self> {
        // Try parsing as URL first
        if let Ok(url) = Self::parse_as_url(input) {
            return Ok(url);
        }

        // Fall back to SSH-style parsing
        Self::parse_ssh_style(input)
    }

    /// Parse as a URL (rcp://user:pass@host:port/path)
    fn parse_as_url(input: &str) -> Result<Self> {
        let input = if input.starts_with("rcp://") {
            input.to_string()
        } else {
            format!("rcp://{}", input)
        };

        match Url::parse(&input) {
            Ok(url) => {
                let host = url
                    .host_str()
                    .ok_or_else(|| {
                        Error::Connection("Invalid host in connection string".to_string())
                    })?
                    .to_string();

                let port = url.port();
                let username = if url.username().is_empty() {
                    None
                } else {
                    Some(url.username().to_string())
                };

                // Only set password if it exists and is not empty
                let password = match url.password() {
                    Some(pass) if !pass.is_empty() => Some(pass.to_string()),
                    _ => None,
                };

                // Only set path if it's not just "/"
                let path = if url.path() == "/" || url.path().is_empty() {
                    None
                } else {
                    Some(url.path().to_string())
                };

                Ok(Self {
                    username,
                    password,
                    host,
                    port,
                    path,
                })
            }
            Err(_) => Err(Error::Connection(
                "Invalid connection string format".to_string(),
            )),
        }
    }

    /// Parse as SSH style (user:pass@host:port/path)
    fn parse_ssh_style(input: &str) -> Result<Self> {
        // Create a mutable copy of the input string
        let mut input_str = input.to_string();
        let mut username = None;
        let mut password = None;
        let mut port = None;
        let mut path = None;
        let mut host;

        // Extract path if present
        if let Some(path_idx) = input_str.find('/') {
            let path_str = input_str[path_idx..].to_string();
            if !path_str.is_empty() {
                path = Some(path_str);
            }
            input_str.truncate(path_idx);
        }

        // Extract username:password if present
        if let Some(creds_idx) = input_str.find('@') {
            let creds = input_str[0..creds_idx].to_string();
            input_str = input_str[creds_idx + 1..].to_string();

            if let Some(pass_idx) = creds.find(':') {
                username = Some(creds[0..pass_idx].to_string());
                // Only set password if it's not empty
                let pass_str = creds[pass_idx + 1..].to_string();
                if !pass_str.is_empty() {
                    password = Some(pass_str);
                }
            } else {
                username = Some(creds);
            }
        }

        // Extract port if present
        host = input_str.clone();
        if let Some(port_idx) = input_str.rfind(':') {
            match input_str[port_idx + 1..].parse::<u16>() {
                Ok(port_num) => {
                    port = Some(port_num);
                    host = input_str[0..port_idx].to_string();
                }
                Err(_) => {
                    // Invalid port format
                    return Err(Error::Connection("Invalid port format".to_string()));
                }
            }
        }

        Ok(Self {
            username,
            password,
            host,
            port,
            path,
        })
    }
}

impl FromStr for ConnectionString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        ConnectionString::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn debug_cs(cs: &ConnectionString, test_name: &str) {
        println!(
            "{}: Username: {:?}, Password: {:?}, Host: {}, Port: {:?}, Path: {:?}",
            test_name, cs.username, cs.password, cs.host, cs.port, cs.path
        );
    }

    #[test]
    fn test_parse_ssh_style() {
        // Test with username, password, host, port and path
        let cs = ConnectionString::parse("user:pass@host:8716/path").unwrap();
        debug_cs(&cs, "Test 1");
        assert_eq!(cs.username, Some("user".to_string()));
        assert_eq!(cs.password, Some("pass".to_string()));
        assert_eq!(cs.host, "host");
        assert_eq!(cs.port, Some(8716));
        assert_eq!(cs.path, Some("/path".to_string()));

        // Test with just host and port
        let cs = ConnectionString::parse("host:8716").unwrap();
        debug_cs(&cs, "Test 2");
        assert_eq!(cs.username, None);
        assert_eq!(cs.password, None);
        assert_eq!(cs.host, "host");
        assert_eq!(cs.port, Some(8716));
        assert_eq!(cs.path, None);

        // Test with username and host
        let cs = ConnectionString::parse("user@host").unwrap();
        debug_cs(&cs, "Test 3");
        assert_eq!(cs.username, Some("user".to_string()));
        assert_eq!(cs.password, None);
        assert_eq!(cs.host, "host");
        assert_eq!(cs.port, None);
        assert_eq!(cs.path, None);

        // Test with empty password
        let cs = ConnectionString::parse("user:@host").unwrap();
        debug_cs(&cs, "Test 4");
        assert_eq!(cs.username, Some("user".to_string()));
        assert_eq!(cs.password, None); // Empty password should be None
        assert_eq!(cs.host, "host");
        assert_eq!(cs.port, None);
        assert_eq!(cs.path, None);
    }

    #[test]
    fn test_parse_url_style() {
        let cs = ConnectionString::parse("rcp://user:pass@host:8716/path").unwrap();
        debug_cs(&cs, "URL Test 1");
        assert_eq!(cs.username, Some("user".to_string()));
        assert_eq!(cs.password, Some("pass".to_string()));
        assert_eq!(cs.host, "host");
        assert_eq!(cs.port, Some(8716));
        assert_eq!(cs.path, Some("/path".to_string()));

        // Test with empty password
        let cs = ConnectionString::parse("rcp://user:@host:8716").unwrap();
        debug_cs(&cs, "URL Test 2");
        println!("Raw password: {:?}", cs.password);
        assert_eq!(cs.username, Some("user".to_string()));
        assert_eq!(cs.password, None); // Empty password should be None
        assert_eq!(cs.host, "host");
        assert_eq!(cs.port, Some(8716));
        assert_eq!(cs.path, None);
    }
}
