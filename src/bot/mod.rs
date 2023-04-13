mod main_menu;
mod onboarding;

use teloxide::{
    dispatching::{dialogue::InMemStorage, HandlerExt},
    macros::BotCommands,
    prelude::*,
};

use self::{main_menu::MainMenu, onboarding::OnBoarding};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type MyDialogue = Dialogue<MainMenu, InMemStorage<MainMenu>>;

// create commands
#[derive(BotCommands, Debug, Clone, Copy, PartialEq, Eq)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
enum Command {
    Start,
    New,
    View,
    Help,
}

pub async fn start() {
    dotenvy::dotenv().ok();
    let token = dotenvy::var("TELOXIDE_TOKEN").unwrap();
    let bot = Bot::new(token);

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<MainMenu>, MainMenu>()
            .branch(
                dptree::case![MainMenu::Start]
                    .filter_command::<Command>()
                    .endpoint(MainMenu::start),
            )
            .branch(dptree::case![MainMenu::OnBoarding(x)].endpoint(OnBoarding::handle)),
    )
    // import database dependencies
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