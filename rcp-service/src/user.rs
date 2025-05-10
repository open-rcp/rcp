use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::ServiceError;

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

pub struct UserManager {
    users: HashMap<String, ServiceUser>,
}

impl UserManager {
    pub fn new() -> Self {
        Self {
            users: HashMap::new()
        }
    }

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
