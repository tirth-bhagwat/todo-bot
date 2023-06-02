mod formatters;
mod main_menu;
mod onboarding;
mod todo_creation;
mod utlities;

use core::fmt;
use futures::future::BoxFuture;

use teloxide::{
    dispatching::{dialogue::InMemStorage, HandlerExt},
    prelude::*,
    utils::command::BotCommands,
};

use self::{main_menu::UserState, onboarding::OnBoarding, todo_creation::TodoReader};
use utlities::is_valid_command;

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

impl Command {
    pub fn get_handler(
        self,
    ) -> Option<
        Box<
            dyn Fn(
                Bot,
                MyDialogue,
                Message,
            )
                -> BoxFuture<'static, Result<(), Box<dyn std::error::Error + Send + Sync>>>,
        >,
    > {
        match self {
            Command::Start => Some(Box::new(|bot, dialogue, msg| {
                Box::pin(OnBoarding::handle(bot, dialogue, msg))
            })),

            Command::New => Some(Box::new(|bot, dialogue, msg| {
                Box::pin(TodoReader::handle(bot, dialogue, msg))
            })),

            _ => None,
        }
    }
}

pub async fn start() {
    dotenvy::dotenv().ok();
    let token = dotenvy::var("TELOXIDE_TOKEN").unwrap();
    let bot = Bot::new(token);

    // let x = OnBoarding::handle;

    // bot.send_message("90", "Bot started")
    //     .parse_mode(MarkdownV2)
    //     .send()
    //     .await
    //     .unwrap();

    // bot.send_message(user_id, text)
    // .parse_mode(MarkdownV2)
    // .disable_web_page_preview(true)
    // .disable_notification(silent)
    // .send()
    // .await
    // .map(|_| ())

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<UserState>, UserState>()
            .branch(
                dptree::case![UserState::Idle]
                    .filter(|msg: Message| is_valid_command(&msg).is_some())
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
        .send()
        .await?;
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
