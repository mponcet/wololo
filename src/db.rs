use std::collections::HashMap;
use std::path::Path;

use crate::mac::MacAddress;

type SlackUserId = String;

pub struct Db {
    devices_by_user: HashMap<SlackUserId, MacAddress>,
}

pub type SharedDb = std::sync::Arc<Db>;

#[derive(Debug)]
pub enum DbError {
    IoError(std::io::Error),
    DeserializeError,
}

impl std::error::Error for DbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            DbError::IoError(ref err) => Some(err),
            DbError::DeserializeError => None,
        }
    }
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DbError::IoError(ref err) => err.fmt(f),
            DbError::DeserializeError => write!(f, "Deserialize error"),
        }
    }
}

impl From<std::io::Error> for DbError {
    fn from(err: std::io::Error) -> Self {
        DbError::IoError(err)
    }
}

impl Db {
    pub fn with_file<P: AsRef<Path>>(path: P) -> Result<Self, DbError> {
        let raw_yaml = std::fs::read_to_string(path)?;
        let devices_by_user =
            serde_yaml::from_str(&raw_yaml).map_err(|_| DbError::DeserializeError)?;

        Ok(Self { devices_by_user })
    }

    pub fn get_mac_by_slack_user_id(&self, slack_user_id: &SlackUserId) -> Option<&MacAddress> {
        self.devices_by_user.get(slack_user_id)
    }
}
