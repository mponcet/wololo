use crate::mac::MacAddress;

pub async fn send_wol(mac: &MacAddress) -> Result<(), std::io::Error> {
    let mac = wol::MacAddr(mac.as_bytes());

    tokio::task::spawn_blocking(move || match wol::send_wol(mac, None, None) {
        Ok(_) => {
            tracing::info!("magic packet sent to {mac}");
            Ok(())
        }
        Err(e) => {
            tracing::error!("failed to send magic packet to {mac}");
            Err(e)
        }
    })
    .await?
}
