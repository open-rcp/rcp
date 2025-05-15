use crate::error::ServiceError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ServiceUser {
    pub username: String,
    pub role: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCredentials {
    pub username: String,
    pub password_hash: String,
}

#[allow(dead_code)]
pub struct UserManager {
    users: HashMap<String, ServiceUser>,
}

impl Default for UserManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UserManager {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_user(&mut self, username: String, role: String) -> Result<(), ServiceError> {
        if self.users.contains_key(&username) {
            return Err(ServiceError::Service("User already exists".to_string()));
        }

        let user = ServiceUser {
            username: username.clone(),
            role,
            permissions: Vec::new(),
        };

        self.users.insert(username, user);
        Ok(())
    }
}
