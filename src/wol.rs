use crate::mac::MacAddress;

pub fn send_wol(mac: &MacAddress) -> Result<(), std::io::Error> {
    wol::send_wol(wol::MacAddr(mac.as_bytes()), None, None)
}
