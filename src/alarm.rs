use teloxide::Bot;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{ChatId, Requester};
use teloxide::types::ParseMode;

use chrono::{Datelike, FixedOffset, Timelike, Utc};

const HOUR: i32 = 3600;
static mut SEND_STATE: bool = false;

async fn send_reminder(bot: &Bot, chat_id: ChatId, message_text: &str) {
    bot.send_message(chat_id, message_text)
        .parse_mode(ParseMode::Html)
        .await
        .unwrap();
}


pub async fn send(bot: &Bot, chat_id: ChatId, message_text: String, time_zone: i32) {
    let tz = FixedOffset::east_opt(time_zone * HOUR).expect("Invalid time zone offset");
    let now = Utc::now().with_timezone(&tz);
    let last_day_of_month: bool = (now + chrono::Duration::days(1)).month() != now.month();

    unsafe {
        if last_day_of_month && (now.hour() == 9 || now.hour() == 22) {
            if !SEND_STATE { send_reminder(bot, chat_id, &message_text).await; }

            SEND_STATE = true;
        } else {
            SEND_STATE = false;
        }
    }
}
