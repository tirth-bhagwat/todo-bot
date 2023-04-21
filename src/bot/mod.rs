mod main_menu;
mod onboarding;
mod todo_creation;

use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        HandlerExt,
    },
    prelude::*,
    utils::command::BotCommands,
};

use self::{main_menu::UserState, onboarding::OnBoarding, todo_creation::TodoReader};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type MyDialogue = Dialogue<UserState, InMemStorage<UserState>>;

#[derive(BotCommands, Debug, Clone, Copy, PartialEq, Eq)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Create a new todo")]
    New,
    #[command(description = "View your todos")]
    View,
    #[command(description = "Get help")]
    Help,
}

pub async fn start() {
    dotenvy::dotenv().ok();
    let token = dotenvy::var("TELOXIDE_TOKEN").unwrap();
    let bot = Bot::new(token);

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<UserState>, UserState>()
            .branch(
                dptree::case![UserState::Idle]
                    .filter_command::<Command>()
                    .endpoint(UserState::init),
            )
            .branch(dptree::case![UserState::Welcome(x)].endpoint(OnBoarding::handle))
            .branch(dptree::case![UserState::New(x)].endpoint(TodoReader::handle))
            .chain(dptree::endpoint(help)),
    )
    .dependencies(dptree::deps![InMemStorage::<UserState>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn help(bot: Bot, upd: Update, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await
        .unwrap();
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_start() {
        start().await;
    }
}
