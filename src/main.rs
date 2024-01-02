mod db;
mod mac;
mod slack_bot;
mod wol;

use crate::db::Db;

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        println!("{} <db>", args[0]);
    } else {
        let db = Db::try_new_shared(&args[1])?;
        let bot = slack_bot::SlackBot::new(db);
        bot.start().await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{}", e);
    }
}
