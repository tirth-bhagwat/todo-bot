use teloxide::{
    payloads::SendMessageSetters,
    requests::{Request, Requester},
    types::{Message, ParseMode::MarkdownV2},
    utils::command::BotCommands,
    Bot,
};

use super::{
    formatters::format_todos, onboarding::OnBoarding, todo_creation::TodoReader,
    utlities::is_valid_command, Command, HandlerResult, MyDialogue,
};

use crate::models::{todos::Todo, users::User};

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
            if let Some(cmd) = is_valid_command(&msg) {
                match cmd {
                    Command::Start => OnBoarding::start(bot, dialogue, msg).await?,
                    Command::New => TodoReader::start(bot, dialogue, msg).await?,
                    Command::View => {
                        let todos = match Todo::get_for_user(&msg.chat.id.to_string()).await {
                            Ok(u) => u
                                .iter()
                                .map(|usr: &(Todo, User)| usr.0.clone())
                                .collect::<Vec<Todo>>(),
                            Err(_) => panic!(),
                        };

                        bot.send_message(msg.chat.id, format_todos(todos))
                            .parse_mode(MarkdownV2)
                            .send()
                            .await?;
                    }
                    Command::Help => {
                        bot.send_message(msg.chat.id, Command::descriptions().to_string())
                            .await?;
                    }
                }
            }
        }

        Ok(())
    }
}

// fn match_command(msg: &Message, command: &str) -> bool {
//     if let Some(text) = msg.text() {
//         if text.trim().to_lowercase() == command || text.trim().to_lowercase() == command[1..] {
//             return true;
//         }
//     }
//     false
// }
