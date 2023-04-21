use teloxide::{requests::Requester, types::Message, Bot};

use super::{
    onboarding::OnBoarding,
    todo_creation::{self, TodoReader},
    HandlerResult, MyDialogue,
};

use crate::models::users::{NewUser, User};

#[derive(Clone, Default, Debug, PartialEq)]
pub enum UserState {
    #[default]
    Idle,
    Welcome(OnBoarding),
    New(TodoReader),
    View,
    Help,
}
impl UserState {
    pub async fn init(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        let user = User::get_by_id(&msg.chat.id.to_string())
            .await
            .unwrap_or_else(|_| None);

        if let None = user {
            dialogue
                .update(UserState::Welcome(OnBoarding::Start))
                .await?;
            OnBoarding::start(bot, dialogue, msg).await?;
        } else {
            if match_command(&msg, "/start") {
                OnBoarding::start(bot, dialogue, msg).await?;
            } else if match_command(&msg, "/new") {
                TodoReader::start(bot, dialogue, msg).await?;
            }
        }

        Ok(())
    }
}

fn match_command(msg: &Message, command: &str) -> bool {
    if let Some(text) = msg.text() {
        if text.trim().starts_with(command) {
            return true;
        }
    }
    false
}
