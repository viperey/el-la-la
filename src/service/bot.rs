use crate::client::telegram::TelegramClient;
use crate::domain::{Gender, Noun, User, UserPlay};
use crate::repository::nouns::{NounsRepository, NounsRepositoryTrait};
use crate::repository::user_plays::{UserPlaysRepository, UserPlaysRepositoryTrait};
use crate::repository::users::{UsersRepository, UsersRepositoryTrait};
use frankenstein::{Update, UpdateContent};
use std::env::VarError::NotPresent;
use std::error::Error;
use std::sync::mpsc::Receiver;

pub struct BotService {
    telegram_client: TelegramClient,
    users_repo: UsersRepository,
    nouns_repo: NounsRepository,
    user_plays_repo: UserPlaysRepository,
}

impl BotService {
    pub fn new(telegram_client: TelegramClient) -> Self {
        Self {
            telegram_client,
            users_repo: UsersRepository::new(),
            nouns_repo: NounsRepository::new(),
            user_plays_repo: UserPlaysRepository::new(),
        }
    }

    pub fn run(&mut self, update_receiver: Receiver<Update>) -> Result<(), Box<dyn Error>> {
        for update in update_receiver {
            if let UpdateContent::Message(message) = update.content {
                let chat_id: i64 = message.chat.id;
                let telegram_user_id: u64 = message.from.unwrap().id;
                self.ensure_user_exists(telegram_user_id)?;

                if message.text.as_deref() == Some("/stop") {
                    self.handle_stop_command(chat_id, telegram_user_id)?;
                } else if message.text.as_deref() == Some("/help") {
                    self.handle_help_command(chat_id);
                } else if message.text.as_deref() == Some("/start") {
                    self.handle_start_command(chat_id, telegram_user_id)?;
                } else if message.text.as_deref() == Some("/stats") {
                    self.handle_stats_command(chat_id)?;
                } else if let Some(text) = message.text {
                    self.handle_text_answer(text, chat_id, telegram_user_id)?;
                }
            }
        }

        Ok(())
    }

    fn ensure_user_exists(&mut self, telegram_user_id: u64) -> Result<(), Box<dyn Error>> {
        let user: Option<User> = self.users_repo.get(telegram_user_id)?;

        if user.is_none() {
            let new_user = User {
                id: 0,
                telegram_user_id,
            };
            self.users_repo.insert(&new_user)?;
        }
        Ok(())
    }

    fn handle_stats_command(&mut self, chat_id: i64) -> Result<(), Box<dyn Error>> {
        let welcome_message = "Coming soon";
        self.telegram_client
            .send_question(chat_id, welcome_message)?;
        Ok(())
    }

    fn handle_start_command(
        &mut self,
        chat_id: i64,
        telegram_user_id: u64,
    ) -> Result<(), Box<dyn Error>> {
        let welcome_message = "\
        Welcome to 'El la la' game.\n\n\
        Your knowledge on Spanish nouns' gender is going to be tested.\n\
        Type /help for further information.\
        ";
        self.telegram_client
            .send_question(chat_id, welcome_message)?;

        self.clean_up_plays(telegram_user_id)?;

        let noun: Noun = self.nouns_repo.get_random()?;
        let user: User = self.users_repo.get(telegram_user_id)?.unwrap();
        self.user_plays_repo.insert(user.id, noun.id)?;

        let message_text = format!(
            "What's the gender of '{}' ({})?",
            noun.spanish, noun.english
        );

        self.telegram_client.send_question(chat_id, &message_text)?;
        Ok(())
    }

    fn handle_stop_command(
        &mut self,
        chat_id: i64,
        telegram_user_id: u64,
    ) -> Result<(), Box<dyn Error>> {
        println!("Stopping the game for {}", telegram_user_id);

        self.clean_up_plays(telegram_user_id)?;

        let text = "Stopping the game for now.\n\
        Send /help for further information\
        ";
        self.telegram_client.send_message(chat_id, text)?;
        Ok(())
    }

    fn clean_up_plays(&mut self, telegram_user_id: u64) -> Result<(), Box<dyn Error>> {
        self.get_user(telegram_user_id)
            .and_then(|user| self.remove_user_last_play(user.id))
    }

    fn remove_user_last_play(&mut self, user_id: i32) -> Result<(), Box<dyn Error>> {
        self.user_plays_repo
            .get_last(user_id)
            .and_then(|maybe_user| match maybe_user {
                Some(user_play) => self.user_plays_repo.remove(user_play.id),
                None => Ok(()),
            })
    }

    fn get_user(&mut self, telegram_user_id: u64) -> Result<User, Box<dyn Error>> {
        self.users_repo
            .get(telegram_user_id)
            .and_then(|maybe_user| match maybe_user {
                Some(user) => Ok(user),
                None => Err(NotPresent.into()),
            })
    }

    fn handle_help_command(&self, chat_id: i64) {
        let text: &str = "\
            /start -> Play the game\n\
            /stop -> Stop the game\n\
            /stats -> Check your current playing statistics\n\
            ";
        self.telegram_client.send_message(chat_id, text).unwrap();
    }

    fn handle_text_answer(
        &mut self,
        text_answer: String,
        chat_id: i64,
        telegram_user_id: u64,
    ) -> Result<(), Box<dyn Error>> {
        let current_play: UserPlay = self.get_current_play(telegram_user_id)?;
        let playing_noun: Noun = self.nouns_repo.get(current_play.noun_id)?;
        let correct_answer: &str = match playing_noun.gender {
            Gender::Masculine => "Masculine",
            Gender::Feminine => "Feminine",
            Gender::Any => "Any",
        };
        let user_answer: &str = text_answer.as_str();
        let is_correct: bool = user_answer == correct_answer;
        self.user_plays_repo.update(current_play.id, is_correct)?;

        let message_text: &str = if is_correct { "Correct!" } else { "Wrong!" };
        self.telegram_client.send_message(chat_id, message_text)?;

        let new_noun: Noun = self.nouns_repo.get_random()?;
        let user = self.users_repo.get(telegram_user_id)?.unwrap();
        self.user_plays_repo.insert(user.id, new_noun.id)?;
        let question_text = format!(
            "What's the gender of '{}' ({})?",
            new_noun.spanish, new_noun.english
        );
        self.telegram_client
            .send_question(chat_id, &question_text)?;
        Ok(())
    }

    fn get_current_play(&mut self, telegram_user_id: u64) -> Result<UserPlay, Box<dyn Error>> {
        let user = self
            .users_repo
            .get(telegram_user_id)?
            .ok_or("User not found")?;
        let current_play = self
            .user_plays_repo
            .get_last(user.id)?
            .ok_or("No current play found")?;
        Ok(current_play)
    }
}
