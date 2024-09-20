use frankenstein::{
    Api, GetUpdatesParams, KeyboardButton, Message, MethodResponse, ReplyKeyboardMarkup,
    ReplyMarkup, SendMessageParams, TelegramApi, Update,
};
use std::sync::mpsc::Sender;
use std::{env, thread};

pub struct TelegramClient {
    api: Api,
    question_keyboard: ReplyMarkup
}

impl TelegramClient {
    pub fn new(channel_sender: Sender<Update>) -> Self {
        let token: String = env::var("TELEGRAM_BOT_TOKEN").expect("Bot token not found");
        let api: Api = Api::new(token.as_str());
        let question_keyboard: ReplyMarkup = Self::build_keyboard();

        let api_clone: Api = api.clone();
        thread::spawn(move || {
            Self::poll_updates(api_clone, channel_sender);
        });

        Self { api, question_keyboard }
    }

    pub fn send_message(
        &self,
        chat_id: i64,
        text: &str,
    ) -> Result<MethodResponse<Message>, frankenstein::Error> {
        let send_message_params = SendMessageParams::builder()
            .chat_id(chat_id)
            .text(text)
            .build();

        self.api.send_message(&send_message_params)
    }

    pub fn send_question(
        &self,
        chat_id: i64,
        text: &str,
    ) -> Result<MethodResponse<Message>, frankenstein::Error> {
        let send_message_params = SendMessageParams::builder()
            .chat_id(chat_id)
            .text(text)
            .reply_markup(self.question_keyboard.clone())
            .build();

        self.api.send_message(&send_message_params)
    }

    fn poll_updates(api: Api, update_sender: Sender<Update>) {
        let mut update_params: GetUpdatesParams = GetUpdatesParams::builder().timeout(10).build();

        loop {
            match api.get_updates(&update_params) {
                Ok(response) => {
                    for update in response.result {
                        if let Err(err) = update_sender.send(update.clone()) {
                            eprintln!("Failed to send update: {}", err);
                            return;
                        }
                        update_params.offset = Some(i64::from(update.update_id) + 1);
                    }
                }
                Err(error) => {
                    eprintln!("Error getting updates: {:?}", error);
                    thread::sleep(std::time::Duration::from_secs(5));
                }
            }
        }
    }

    fn build_keyboard() -> ReplyMarkup {
        let row: Vec<KeyboardButton> = vec![
            KeyboardButton::builder().text("Masculine").build(),
            KeyboardButton::builder().text("Feminine").build(),
            KeyboardButton::builder().text("Any").build(),
        ];
        let keyboard_markup: ReplyKeyboardMarkup = ReplyKeyboardMarkup::builder()
            .keyboard(vec![row])
            .build();
        ReplyMarkup::ReplyKeyboardMarkup(keyboard_markup)
    }
}
