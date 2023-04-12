use teloxide::{requests::Requester, types::Message, Bot};

use super::{HandlerResult, MainMenu, MyDialogue};

#[derive(Clone, Default, Debug)]
pub enum OnBoarding {
    #[default]
    Start,
    Name,
}

impl OnBoarding {
    pub(super) async fn handle(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        if let Some(x) = dialogue.get().await? {
            match x {
                MainMenu::OnBoarding(OnBoarding::Start) => {
                    bot.send_message(msg.chat.id, "What is your name?").await?;
                    dialogue
                        .update(MainMenu::OnBoarding(OnBoarding::Name))
                        .await?;
                }
                MainMenu::OnBoarding(OnBoarding::Name) => {
                    if let Some(text) = msg.text() {
                        bot.send_message(
                            msg.chat.id,
                            format!("Hello, {}!\n Welcome aboard!", text),
                        )
                        .await?;
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
