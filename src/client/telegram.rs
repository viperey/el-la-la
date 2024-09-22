use frankenstein::{Api, BotCommand, GetUpdatesParams, KeyboardButton, Message, MethodResponse, ReplyKeyboardMarkup, ReplyMarkup, SendMessageParams, SetMyCommandsParams, TelegramApi, Update};
use std::error::Error;
use std::sync::mpsc::Sender;
use std::{env, thread};

pub struct TelegramClient {
    api: Api,
    question_keyboard: ReplyMarkup,
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

        Self::set_bot_commands(&api).expect("Failed to set bot commands");
        Self {
            api,
            question_keyboard,
        }
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
    
    fn set_bot_commands(api: &Api) -> Result<(), Box<dyn Error>> {
        let commands = vec![
            BotCommand::builder()
                .command("start")
                .description("Start the game")
                .build(),
            BotCommand::builder()
                .command("stop")
                .description("Stop the game")
                .build(),
            BotCommand::builder()
                .command("help")
                .description("Show help information")
                .build(),
            BotCommand::builder()
                .command("stats")
                .description("Show your statistics")
                .build(),
        ];

        let params = SetMyCommandsParams::builder().commands(commands).build();

        api.set_my_commands(&params)?;

        Ok(())
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
        let keyboard_markup: ReplyKeyboardMarkup =
            ReplyKeyboardMarkup::builder().keyboard(vec![row]).build();
        ReplyMarkup::ReplyKeyboardMarkup(keyboard_markup)
    }
}
