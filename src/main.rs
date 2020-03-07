use teloxide::{prelude::*, utils::command::BotCommand};
use serde::{Deserialize};
use tokio::task::block_in_place;
//use serde_json::Result;

#[derive(Deserialize, Debug)]
struct Party {
    id: String,
    nameParty: String,
    dateStart: String,
    dateEnd: String,
    nameType: String,
    nameCountry: String,
    isoCountry: String,
    nameTown: String,
    geoLat: Option<String>,
    geoLon: Option<String>,
    nameOrganizer: String,
    urlOrganizer: Option<String>,
    urlImageSmall: Option<String>,
    urlImageMedium: Option<String>,
    urlImageFull: Option<String>,
    dateCreated: String,
    dateModified: String,
    nameStatus: String,
    urlPartyHtml: String,
    urlPartyJson: String,
}

#[derive(Deserialize, Debug)]
struct Parties {
    partylist: Vec<Party>,
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Check if the bot is alive.")]
    Ping,
    #[command(description = "Return a list with the upcoming raves.")]
    Raves,
}


async fn answer(
    cx: DispatcherHandlerCx<Message>,
    command: Command,
) -> ResponseResult<()> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Ping => cx.answer("Pong!").send().await?,
        Command::Raves => cx.answer(raves().await).send().await?,
    };

    Ok(())
}

async fn handle_commands(rx: DispatcherHandlerRx<Message>) {
    // Only iterate through commands in a proper format:
    rx.commands::<Command, String>(format!("Goabasebot"))
        // Execute all incoming commands concurrently:
        .for_each_concurrent(None, |(cx, command, _)| async move {
            answer(cx, command).await.log_on_error().await;
        })
        .await;
}

async fn raves() -> String {
    let data = block_in_place(|| get_parties());
    //TODO quitar unwrap
    let parsed: Parties = serde_json::from_str(&data.unwrap()).unwrap();
    //TODO Esto es una chapuza, mejorarlo
    let ref name = parsed.partylist[0].nameParty;
    return name.to_string();

}

#[tokio::main]
async fn main() {
    run().await;

}

async fn run() {
    teloxide::enable_logging!();
    //log::info!("Starting simple_commands_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot).messages_handler(handle_commands).dispatch().await;
}

fn get_parties() -> Result<String, reqwest::Error> {
    let resp = reqwest::blocking::get("https://www.goabase.net/api/party/json/?country=ES")?
        .text()?;

    Ok((resp))
}

