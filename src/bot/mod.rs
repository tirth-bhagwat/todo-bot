mod main_menu;
mod onboarding;

use teloxide::{
    dispatching::{dialogue::InMemStorage, HandlerExt},
    prelude::*,
};

use self::{main_menu::MainMenu, onboarding::OnBoarding};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type MyDialogue = Dialogue<MainMenu, InMemStorage<MainMenu>>;

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
