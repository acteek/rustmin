use teloxide::{prelude::*, utils::command::BotCommands};
use strum_macros::EnumIter;
use crate::store::Store;


#[derive(BotCommands, Clone, EnumIter)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Main menu.")]
    Start,
    #[command(description = "Check subscription status.")]
    Status,
    #[command(description = "Create subscription.")]
    Subscribe,
    #[command(description = "Delete subscription.")]
    Unsubscribe,
}


async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let name = msg.from().map(|user| user.first_name.clone()).expect("User");
    let chat_id = msg.chat.id;
    match cmd {
        Command::Start => {
            bot.send_message(chat_id, Command::descriptions().to_string()).await?
        }
        Command::Status => {
            let message = "status";
            bot.send_message(chat_id, format!("TODO {message}")).await?
        }
        Command::Subscribe => {
            let message = "subscribe";
            log::info!("{name} subscribed chatId[{chat_id}]");
            bot.send_message(chat_id, format!("TODO {message}"))
                .await?
        }
        Command::Unsubscribe => {
            let message = "unsubscribe";
            log::info!("{name} unsubscribed chatId[{chat_id}]");
            bot.send_message(chat_id, format!("TODO {message}")).await?
        }
    };

    Ok(())
}

pub async fn start_polling(token: &str) {
    let bot = Bot::new(token);
    let commands = Command::bot_commands();

    log::info!("Starting bot polling...");
    bot.set_my_commands(commands).await.expect("Error during set commands");
    Command::repl(bot, answer).await;
}
