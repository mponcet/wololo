mod db;
mod mac;
mod wol;

use crate::db::Db;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 3 {
        println!("{} <file> <slack_user_id>", args[0]);
    } else {
        let db = Db::with_file(&args[1])?;
        if let Some(mac) = db.get_mac_by_slack_user_id(&args[2]) {
            println!("Waking {} with mac {}", &args[2], mac);
            wol::send_wol(mac);
        }
    }

    Ok(())
}
