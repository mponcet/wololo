use regex::Regex;

#[derive(PartialEq, Eq)]
pub struct MacAddress(String);

impl TryFrom<&str> for MacAddress {
    type Error = ();

    fn try_from(mac: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();

        if re.is_match(mac) {
            Ok(Self(mac.to_owned()))
        } else {
            Err(())
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

pub struct Device {
    pub name: String,
    pub mac: MacAddress,
}

impl Device {
    pub fn new(name: &str, mac: &str) -> Result<Self, ()> {
        Ok(Self {
            name: name.to_owned(),
            mac: MacAddress::try_from(mac)?,
        })
    }
}
