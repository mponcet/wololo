use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct DeviceName(String);

impl TryFrom<&str> for DeviceName {
    type Error = String;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        if name.chars().any(|c| !c.is_alphanumeric()) {
            Err("Device name should only contain alphanumeric characters".to_owned())
        } else {
            Ok(Self(name.to_owned()))
        }
    }
}

impl AsRef<str> for DeviceName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for DeviceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct MacAddress(String);

impl TryFrom<&str> for MacAddress {
    type Error = String;

    fn try_from(mac: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();

        if re.is_match(mac) {
            Ok(Self(mac.to_owned()))
        } else {
            Err(format!("{} is not a valid mac address", mac))
        }
    }
}

impl AsRef<str> for MacAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Device {
    pub name: DeviceName,
    pub mac: MacAddress,
}

impl TryFrom<(&str, &str)> for Device {
    type Error = String;

    fn try_from(name_mac: (&str, &str)) -> Result<Self, Self::Error> {
        Ok(Self {
            name: DeviceName::try_from(name_mac.0)?,
            mac: MacAddress::try_from(name_mac.1)?,
        })
    }
}
