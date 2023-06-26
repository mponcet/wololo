use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(try_from = "&str")]
pub struct MacAddress([u8; 6]);

#[derive(Debug)]
pub enum MacAddressError {
    Length,
    Format,
    Separator,
}

impl std::fmt::Display for MacAddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MacAddressError::Length => {
                write!(f, "Invalid mac address length")
            }
            MacAddressError::Format => {
                write!(f, "Invalid mac address format")
            }
            MacAddressError::Separator => {
                write!(f, "Invalid mac address separator (should be ':' or '-')")
            }
        }
    }
}

impl std::fmt::Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl TryFrom<&str> for MacAddress {
    type Error = MacAddressError;

    fn try_from(mac: &str) -> Result<Self, Self::Error> {
        if mac.len() != 17 {
            return Err(MacAddressError::Length);
        }

        let sep = mac.chars().nth(2).unwrap();
        if sep != ':' && sep != '-' {
            return Err(MacAddressError::Separator);
        }

        let mut mac_addr = [0u8; 6];
        for (i, c) in (0..6).zip(mac.split(sep)) {
            match u8::from_str_radix(c, 16) {
                Ok(byte) => mac_addr[i] = byte,
                Err(_) => return Err(MacAddressError::Format),
            }
        }

        Ok(MacAddress(mac_addr))
    }
}

impl MacAddress {
    pub fn as_bytes(&self) -> [u8; 6] {
        self.0
    }
}
