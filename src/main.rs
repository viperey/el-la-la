mod client;
mod domain;
mod repository;
mod service;

use crate::service::bot::BotService;
use crate::client::telegram::TelegramClient;
use crate::repository::migration;
use dotenv::dotenv;
use std::error::Error;
use std::sync::mpsc::{channel, Receiver, Sender};
use frankenstein::Update;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    migration::run()?;

    let (channel_sender, channel_receiver): (Sender<Update>, Receiver<Update>) = channel();
    let telegram_client: TelegramClient= TelegramClient::new(channel_sender);
    BotService::new(telegram_client).run(channel_receiver)
}
