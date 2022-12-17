use crate::device::MacAddress;

pub enum WakeError {
    Io,
}

impl std::fmt::Display for WakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WakeError::Io => write!(f, "IO error while sending magic packet"),
        }
    }
}

pub fn wake(mac: &MacAddress) -> Result<(), WakeError> {
    // as_ref will not be needed in future release :
    // https://github.com/LesnyRumcajs/wakey/commit/c453a88c998999a2b9b7dcfa365435df6f2857f5
    let wol = wakey::WolPacket::from_string(mac.as_ref(), ':');

    match wol.send_magic() {
        Ok(_) => Ok(()),
        Err(_) => Err(WakeError::Io),
    }
}
