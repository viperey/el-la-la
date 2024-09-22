use frankenstein::{
    Api, BotCommand, GetUpdatesParams, KeyboardButton, Message, MethodResponse, ReactionType,
    ReactionTypeEmoji, ReplyKeyboardMarkup, ReplyMarkup, SendMessageParams,
    SetMessageReactionParams, SetMyCommandsParams, TelegramApi, Update,
};
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
        let question_keyboard: ReplyMarkup = Self::build_keyboard();

        let api: Api = Api::new(token.as_str());
        let api_clone: Api = api.clone();
        Self::set_bot_commands(&api).expect("Failed to set bot commands");
        thread::spawn(move || Self::poll_updates(api_clone, channel_sender));

        Self {
            api,
            question_keyboard,
        }
    }

    pub fn send_reaction(
        &self,
        chat_id: i64,
        message_id: i32,
        reaction: &str,
    ) -> Result<(), Box<dyn Error>> {
        let set_message_reaction_params = SetMessageReactionParams::builder()
            .chat_id(chat_id)
            .message_id(message_id)
            .reaction(vec![ReactionType::Emoji(ReactionTypeEmoji {
                emoji: reaction.to_string(),
            })])
            .build();
        self.api.set_message_reaction(&set_message_reaction_params)
            .map(|_| ())
            .map_err(|e| e.into())
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
        api.set_my_commands(&params).map(|_| ()).map_err(|e| e.into())
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
