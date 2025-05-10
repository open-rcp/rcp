use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub address: String,
    pub port: u16,
    pub tls: TlsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TlsConfig {
    pub enabled: bool,
    pub cert_path: String,
    pub key_path: String,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".to_string(),
            port: 8716,
            tls: TlsConfig {
                enabled: false,
                cert_path: "cert.pem".to_string(),
                key_path: "key.pem".to_string(),
            },
        }
    }
}
