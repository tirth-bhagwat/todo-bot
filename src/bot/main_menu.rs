use teloxide::{requests::Requester, types::Message, Bot};

use super::{onboarding::OnBoarding, HandlerResult, MyDialogue};

use crate::models::users::{NewUser, User};

#[derive(Clone, Default, Debug)]
pub enum MainMenu {
    #[default]
    Start,
    OnBoarding(OnBoarding),
    New,
    View,
    Help,
}
impl MainMenu {
    pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        let user = User::get_by_tele_id(&msg.chat.id.to_string())
            .await
            .unwrap_or_else(|_| None);

        if let Some(x) = user {
            bot.send_message(msg.chat.id, format!("Welcome {}!", x.name))
                .await?;
            dialogue.exit().await?;
        } else {
            bot.send_message(msg.chat.id, "What is your name?").await?;
            dialogue
                .update(MainMenu::OnBoarding(OnBoarding::Name))
                .await?;
        }

        Ok(())
    }
}
