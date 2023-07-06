mod db;
mod mac;
mod slack_bot;
mod wol;

use std::sync::Arc;

use crate::db::Db;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        println!("{} <db>", args[0]);
    } else {
        let db = Arc::new(Db::with_file(&args[1])?);
        let bot = slack_bot::SlackBot::new(db);
        let _ = bot.start();
    }

    Ok(())
}
