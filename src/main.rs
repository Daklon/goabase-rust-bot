use teloxide::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("Starting!");
    let data = get_parties().await;
    
    println!("{:?}", data);

    teloxide::enable_logging!();
    log::info!("Starting ping_pong_bot!");

    let data = get_parties().await;

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each(|message| async move {
                message.answer("pong").send().await.log_on_error().await;
            })
        })
        .dispatch()
        .await;
    
}

async fn get_parties() -> Result<String, reqwest::Error> {
    let resp = reqwest::get("https://www.rust-lang.org").await?.text().await?;

    Ok((resp))
}

