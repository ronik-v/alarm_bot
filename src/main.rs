mod config;
mod alarm;

use dotenv::dotenv;

use teloxide::Bot;
use teloxide::prelude::Requester;
use tokio::time::{self, Duration};
use config::{AppData, app_data};
use alarm::Alarm;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_data: AppData = app_data();
    let bot = Bot::new(app_data.bot_token);
    let alarm = Alarm::new(&bot, app_data.chat_id, &app_data.message_text, app_data.time_zone);
    let mut interval = time::interval(Duration::from_secs(60));

    loop {
        interval.tick().await;
        alarm.send().await;
    }
}
