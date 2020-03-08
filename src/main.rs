use teloxide::{prelude::*, utils::command::BotCommand};
use serde::{Deserialize};
use tokio::task::block_in_place;
use std::fmt::Write;
//use serde_json::Result;

#[derive(Deserialize, PartialEq, Clone, Debug)]
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

#[derive(Deserialize, Default, PartialEq, Clone, Debug)]
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
    let parsed = filter_parties(parsed);
    let message = format_parties(parsed); 
    //let ref name = parsed.partylist[0].nameParty;
    //return name.to_string();
    return message;

}

fn format_parties(data: Parties) -> String {
    let mut output: String = Default::default();
    for party in data.partylist {
        write!(&mut output, "{} {} {} {}\n",party.nameParty, party.dateStart, party.dateEnd, party.nameTown);
    }
    return output;
}

// Return only the parties that are in the canary islands
fn filter_parties(data: Parties) -> Parties {
    let mut filtered: Parties = Default::default();
    for party in data.partylist {
        //TODO quitar unwrap
        let geoLat = party.geoLat.clone().unwrap().parse::<f64>().unwrap();
        let geoLon = party.geoLon.clone().unwrap().parse::<f64>().unwrap();
        //29.555372, -13.265574 //arriba derecha
        //27.669400, -13.195601 //abajo derecha
        //27.614655, -18.239785// abajo izquierda
        //29.285276, -18.254011// arriba izquierda
        // Match using coordinates
        if geoLat < 29.555 && geoLat > 27.6145 {
            println!("1");
            if geoLon < -13.1956 && geoLon > -18.540 {
                println!("2");
                filtered.partylist.push(party);
                continue;
            }    
        }
        // match using nameTown
        match party.nameTown.to_lowercase().as_str() {
            "tenerife" => filtered.partylist.push(party),
            _ => continue,
        }
    }
    return filtered;
}

#[test]
fn test_filter_parties(){
    let party1 = Party {
    id: String::from("00000"),
    nameParty: String::from("nameParty1"),
    dateStart: String::from("dateStart1"),
    dateEnd: String::from("dateEnd1"),
    nameType: String::from("nameType1"),
    nameCountry: String::from("nameCountry1"),
    isoCountry: String::from("isoCountry1"),
    nameTown: String::from("nameTown1"),
    geoLat: Some(String::from("28.02")),
    geoLon: Some(String::from("-16.01")),
    nameOrganizer: String::from("nameOrganizer1"),
    urlOrganizer: Some(String::from("urlOrganizer1")),
    urlImageSmall: Some(String::from("urlImageSmall1")),
    urlImageMedium: Some(String::from("urlImageMedium1")),
    urlImageFull: Some(String::from("urlImageFull1")),
    dateCreated: String::from("dateCreated1"),
    dateModified: String::from("dateModified1"),
    nameStatus: String::from("nameStatus1"),
    urlPartyHtml: String::from("urlPartyHtml"),
    urlPartyJson: String::from("urlPartyJson1"),
    };
    
    let party2 = Party {
    id: String::from("00001"),
    nameParty: String::from("nameParty2"),
    dateStart: String::from("dateStart2"),
    dateEnd: String::from("dateEnd2"),
    nameType: String::from("nameType2"),
    nameCountry: String::from("nameCountry2"),
    isoCountry: String::from("isoCountry2"),
    nameTown: String::from("TENERIFE"),
    geoLat: Some(String::from("-15.00")),
    geoLon: Some(String::from("28.0")),
    nameOrganizer: String::from("nameOrganizer2"),
    urlOrganizer: Some(String::from("urlOrganizer1")),
    urlImageSmall: Some(String::from("urlImageSmall2")),
    urlImageMedium: Some(String::from("urlImageMedium2")),
    urlImageFull: Some(String::from("urlImageFull1")),
    dateCreated: String::from("dateCreated2"),
    dateModified: String::from("dateModified2"),
    nameStatus: String::from("nameStatus2"),
    urlPartyHtml: String::from("urlPartyHtml2"),
    urlPartyJson: String::from("urlPartyJson"),
    };
    let mut all_parties: Parties = Default::default();
    all_parties.partylist.push(party1);
    all_parties.partylist.push(party2);
    let all_parties_clone = all_parties.clone();

    let result = filter_parties(all_parties);
    assert_eq!(all_parties_clone, result);
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

    Ok(resp)
}

