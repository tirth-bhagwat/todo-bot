use diesel::helper_types::On;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type MyDialogue = Dialogue<MainMenu, InMemStorage<MainMenu>>;

#[derive(Clone, Default, Debug)]
pub enum OnBoarding {
    #[default]
    Start,
    Name,
}

impl OnBoarding {
    async fn handle(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
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
    async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        bot.send_message(msg.chat.id, "Hello!").await?;
        bot.send_message(msg.chat.id, "What is your name?").await?;
        dialogue
            .update(MainMenu::OnBoarding(OnBoarding::Name))
            .await?;
        Ok(())
    }
}

pub async fn start() {
    dotenvy::dotenv().ok();
    let token = dotenvy::var("TELOXIDE_TOKEN").unwrap();
    println!("token: {}", token);
    let bot = Bot::new(token);
    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<MainMenu>, MainMenu>()
            .branch(dptree::case![MainMenu::Start].endpoint(MainMenu::start))
            .branch(dptree::case![MainMenu::OnBoarding(x)].endpoint(OnBoarding::handle)),
    )
    .dependencies(dptree::deps![InMemStorage::<MainMenu>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_start() {
        start().await;
    }
}
