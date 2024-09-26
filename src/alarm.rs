use teloxide::Bot;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{ChatId, Requester};
use teloxide::types::ParseMode;

use chrono::{Datelike, FixedOffset, Timelike, Utc};

const HOUR: i32 = 3600;
static mut SEND_STATE: bool = false;


pub struct Alarm<'bot, 'message_text> {
    pub bot: &'bot Bot,
    pub chat_id: ChatId,
    pub message_text: &'message_text str,
    pub time_zone: i32
}

impl <'bot, 'message_text> Alarm<'bot, 'message_text> {
    pub fn new(bot: &'bot Bot, chat_id: ChatId,  message_text: &'message_text str, time_zone: i32) -> Self {
        Self { bot, chat_id, message_text, time_zone }
    }

    async fn send_reminder(&self) {
        self.bot.send_message(self.chat_id, self.message_text)
            .parse_mode(ParseMode::Html)
            .await
            .unwrap();
    }

    pub async fn send(&self) {
        let tz = FixedOffset::east_opt(self.time_zone * HOUR).expect("Invalid time zone offset");
        let now = Utc::now().with_timezone(&tz);
        let last_day_of_month: bool = (now + chrono::Duration::days(1)).month() != now.month();

        unsafe {
            if last_day_of_month && (now.hour() == 9 || now.hour() == 22) {
                if !SEND_STATE { self.send_reminder().await; }

                SEND_STATE = true;
            } else {
                SEND_STATE = false;
            }
        }
    }
}