use teloxide::{requests::Requester, types::Message, utils::markdown::escape, Bot};

use super::{HandlerResult, MyDialogue, UserState};

use crate::models::todos::{NewTodo, Todo};

#[derive(Clone, Default, Debug, PartialEq)]
pub enum TodoReader {
    #[default]
    Start,
    Title,
    Description(String),
}
impl TodoReader {
    pub(super) async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        dialogue.update(UserState::New(TodoReader::Start)).await?;
        TodoReader::handle(bot, dialogue, msg).await?;
        Ok(())
    }
    pub(super) async fn handle(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        if let Ok(Some(state)) = dialogue.get().await {
            match state {
                UserState::New(TodoReader::Start) => {
                    bot.send_message(msg.chat.id, "Enter title").await?;
                    dialogue.update(UserState::New(TodoReader::Title)).await?;
                }
                UserState::New(TodoReader::Title) => {
                    if let Some(t) = msg.text() {
                        bot.send_message(msg.chat.id, "Enter description").await?;
                        dialogue
                            .update(UserState::New(TodoReader::Description(escape(t))))
                            .await?;
                    } else {
                        bot.send_message(msg.chat.id, "Enter title").await?;
                    }
                }
                UserState::New(TodoReader::Description(t)) => {
                    if let Some(d) = msg.text() {
                        bot.send_message(
                            msg.chat.id,
                            format!("Todo saving  {}", msg.chat.id.to_string()),
                        )
                        .await?;
                        let todo = NewTodo {
                            title: t,
                            description: Some(escape(d)),
                            user_id: msg.chat.id.to_string(),
                            status: 0,
                        };

                        todo.save().await.unwrap();

                        bot.send_message(msg.chat.id, format!("Todo saved")).await?;

                        dialogue.exit().await?;
                    } else {
                        bot.send_message(msg.chat.id, "Enter description").await?;
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
