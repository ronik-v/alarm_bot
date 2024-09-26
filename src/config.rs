use std::env;
use teloxide::types::ChatId;

pub struct AppData {
    pub bot_token: String,
    pub chat_id: ChatId,
    pub time_zone: i32,
    pub message_text: String
}

pub fn app_data() -> AppData {
    let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN must be set in .env");
    let chat_id: ChatId = ChatId(env::var("CHAT_ID").expect("CHAT_ID must be set in .env").parse().unwrap());
    let time_zone: i32 = env::var("TIME_ZONE").expect("TIME_ZONE must be set in .env").parse().unwrap();
    let message_text = env::var("MESSAGE_TEXT").expect("MESSAGE_TEXT must be set in .env");

    AppData {
        bot_token, chat_id, time_zone, message_text
    }
}
