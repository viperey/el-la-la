mod client;
mod domain;
mod repository;
mod service;

use crate::service::bot::BotService;
use crate::client::telegram::TelegramClient;
use crate::repository::migration;
use dotenv::dotenv;
use std::error::Error;
use std::sync::mpsc::Receiver;
use frankenstein::Update;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    migration::run()?;

    let (telegram_client, update_receiver): (TelegramClient, Receiver<Update>) = TelegramClient::new();
    let mut bot_service: BotService = BotService::new(telegram_client);

    bot_service.run(update_receiver)
}
