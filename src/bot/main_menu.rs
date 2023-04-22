use teloxide::{
    requests::{Requester, RequesterExt},
    types::{Message, ParseMode::MarkdownV2},
    Bot,
};

use super::{
    formatters::format_todos,
    onboarding::OnBoarding,
    todo_creation::{self, TodoReader},
    HandlerResult, MyDialogue,
};

use crate::models::{
    todos::Todo,
    users::{NewUser, User},
};

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
            } else if match_command(&msg, "/view") {
                let todos = match Todo::get_for_user(&msg.chat.id.to_string()).await {
                    Ok(u) => u
                        .iter()
                        .map(|usr: &(Todo, User)| usr.0.clone())
                        .collect::<Vec<Todo>>(),
                    Err(_) => panic!(),
                };
                let mut x = bot.send_message(msg.chat.id, format_todos(todos));
                x.parse_mode = Some(MarkdownV2);
                x.await?;
            }
        }

        Ok(())
    }
}

fn match_command(msg: &Message, command: &str) -> bool {
    if let Some(text) = msg.text() {
        if text.trim().to_lowercase().starts_with(command) || text.trim().to_lowercase().starts_with(&command[1..].to_string()) {
            return true;
        }
    }
    false
}
