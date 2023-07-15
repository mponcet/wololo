use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::mac::MacAddress;

type SlackUserId = String;

pub struct Db {
    devices_by_user: HashMap<SlackUserId, MacAddress>,
}

pub type SharedDb = std::sync::Arc<Db>;

#[derive(Debug)]
pub enum DbError {
    IoError(std::io::Error),
    DeserializeError(serde_yaml::Error),
}

impl std::error::Error for DbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DbError::IoError(e) => Some(e),
            DbError::DeserializeError(e) => Some(e),
        }
    }
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::IoError(e) => write!(f, "Error: {}", e),
            DbError::DeserializeError(e) => write!(f, "Error: {}", e),
        }
    }
}

impl From<std::io::Error> for DbError {
    fn from(e: std::io::Error) -> Self {
        DbError::IoError(e)
    }
}

impl From<serde_yaml::Error> for DbError {
    fn from(e: serde_yaml::Error) -> Self {
        DbError::DeserializeError(e)
    }
}

impl Db {
    pub fn try_new<P: AsRef<Path>>(path: P) -> Result<Self, DbError> {
        let raw_yaml = std::fs::read_to_string(path)?;
        let devices_by_user = serde_yaml::from_str(&raw_yaml).map_err(DbError::DeserializeError)?;

        Ok(Self { devices_by_user })
    }

    pub fn try_new_shared<P: AsRef<Path>>(path: P) -> Result<SharedDb, DbError> {
        Ok(Arc::new(Self::try_new(path)?))
    }

    pub fn get_mac_by_slack_user_id(&self, slack_user_id: &SlackUserId) -> Option<&MacAddress> {
        self.devices_by_user.get(slack_user_id)
    }
}
