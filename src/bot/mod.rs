mod formatters;
mod main_menu;
mod onboarding;
mod todo_creation;

use core::fmt;

use teloxide::{
    dispatching::{dialogue::InMemStorage, HandlerExt},
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

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Start => write!(f, "/start"),
            Command::New => write!(f, "/new"),
            Command::View => write!(f, "/view"),
            Command::Help => write!(f, "/help"),
        }
    }
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
                    .filter(|msg: Message| match_commands(msg))
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

async fn help(bot: Bot, _: Update, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await
        .unwrap();
    Ok(())
}

fn match_commands(inp: Message) -> bool {
    if let Some(msg) = inp.text() {
        let msg = msg.trim().to_lowercase();
        for cmd in vec![Command::New, Command::View, Command::Start, Command::Help] {
            if msg.starts_with(&cmd.to_string()) || msg.starts_with(&cmd.to_string()[1..]) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_start() {
        start().await;
    }
}
