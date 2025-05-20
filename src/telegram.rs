use crate::location::Location;
use dotenv::dotenv;
use std::error::Error;
use teloxide::prelude::*;

pub async fn send_location_to_telegram(location: &Location) -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let bot = Bot::from_env();

    let chat_id = ChatId(std::env::var("TELEGRAM_CHAT_ID")?.parse::<i64>()?);

    let message = format!(
        "*Location Update*\n\n\
         *IP Address*: {}\n\
         *Time*: {}\n\n",
        location.ip.replace(".", "\\."),
        location.timestamp.format("%Y\\-%m\\-%d %H:%M:%S UTC"),
    );
    bot.send_message(chat_id, message)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;

    Ok(())
}
