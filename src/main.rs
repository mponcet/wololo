mod db;
mod mac;
mod slack_bot;
mod wol;

use crate::db::Db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();

    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    if args.len() != 2 {
        println!("{} <db>", args[0]);
    } else {
        let db = Db::try_new_shared(&args[1])?;
        let bot = slack_bot::SlackBot::new(db);
        bot.start().await?;
    }

    Ok(())
}
