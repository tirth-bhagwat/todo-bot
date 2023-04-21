use teloxide::{requests::Requester, types::Message, Bot};

use super::{HandlerResult, MyDialogue, UserState};

use crate::models::users::{NewUser, User};

#[derive(Clone, Default, Debug, PartialEq)]
pub enum OnBoarding {
    #[default]
    Start,
    Name,
}

impl OnBoarding {
    pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        let user = User::get_by_id(&msg.chat.id.to_string())
            .await
            .unwrap_or_else(|_| None);

        if let Some(x) = user {
            bot.send_message(msg.chat.id, format!("Welcome {}!", x.name))
                .await?;
            dialogue.exit().await?;
        } else {
            dialogue
                .update(UserState::Welcome(OnBoarding::Start))
                .await?;
            OnBoarding::handle(bot, dialogue, msg).await?;
        }

        Ok(())
    }

    pub(super) async fn handle(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        if let Some(x) = dialogue.get().await? {
            match x {
                UserState::Welcome(OnBoarding::Start) => {
                    bot.send_message(msg.chat.id, "What is your name?").await?;
                    dialogue
                        .update(UserState::Welcome(OnBoarding::Name))
                        .await?;
                }
                UserState::Welcome(OnBoarding::Name) => {
                    if let Some(text) = msg.text() {
                        bot.send_message(
                            msg.chat.id,
                            format!("Hello, {}!\n Welcome aboard!", text),
                        )
                        .await?;

                        let user = NewUser {
                            id: msg.chat.id.to_string(),
                            name: text.to_string(),
                        };

                        user.save().await.unwrap();

                        dialogue.exit().await?;
                    } else {
                        bot.send_message(msg.chat.id, "Invalid name ...").await?;
                    }
                }
                _ => {
                    bot.send_message(msg.chat.id, "Invalid name ...").await?;
                }
            }
        } else {
        }
        Ok(())
    }
}
