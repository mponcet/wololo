use std::collections::HashMap;
use std::path::Path;

use crate::mac::MacAddress;

type SlackUserId = String;

pub struct Db {
    devices_by_user: HashMap<SlackUserId, MacAddress>,
}

impl Db {
    pub fn with_file<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let raw_yaml = std::fs::read_to_string(path.as_ref())?;
        let devices_by_user = serde_yaml::from_str(&raw_yaml).unwrap();

        Ok(Self { devices_by_user })
    }

    pub fn get_mac_by_slack_user_id(&self, slack_user_id: &SlackUserId) -> Option<&MacAddress> {
        self.devices_by_user.get(slack_user_id)
    }
}
