use teloxide::{requests::Requester, types::Message, Bot};

use super::{onboarding::OnBoarding, MyDialogue, HandlerResult};

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
    pub(super) async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        bot.send_message(msg.chat.id, "Hello!").await?;
        bot.send_message(msg.chat.id, "What is your name?").await?;
        dialogue
            .update(MainMenu::OnBoarding(OnBoarding::Name))
            .await?;
        Ok(())
    }
}
