#![allow(clippy::trivial_regex)]
#![allow(dead_code)]

mod dialogue;

use crate::dialogue::Dialogue;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env();

    teloxide::dialogues_repl(bot, |message, dialogue| async move {
        handle_message(message, dialogue)
            .await
            .expect("Something wrong with the bot!")
    })
    .await;
}

async fn handle_message(cx: UpdateWithCx<Message>, dialogue: Dialogue) -> TransitionOut<Dialogue> {
    let upd = cx.update.clone();

    match upd.text_owned() {
        None => match upd.location() {
            None => next(dialogue),
            Some(location) => {
                let lat = location.latitude;
                let long = location.longitude;
                let location = format!("{}, {}", lat.to_string(), long.to_string());
                log::info!("location is: {}", location);

                dialogue.react(cx, location).await
            }
        },
        Some(ans) => {
            log::info!("text is: {}", ans);

            dialogue.react(cx, ans).await
        }
    }
}
