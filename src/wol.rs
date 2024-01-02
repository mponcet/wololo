use crate::mac::MacAddress;

pub async fn send_wol(mac: &MacAddress) -> Result<(), std::io::Error> {
    let mac = wol::MacAddr(mac.as_bytes());
    tokio::task::spawn_blocking(move || wol::send_wol(mac, None, None)).await?
}
