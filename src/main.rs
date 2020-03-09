use teloxide::{prelude::*, utils::command::BotCommand};
use serde::{Deserialize};
use tokio::task::block_in_place;
use std::fmt::Write;
//use serde_json::Result;

use chrono::{DateTime, TimeZone, FixedOffset};

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct Party {
    id: String,
    name_party: String,
    date_start: String,
    date_end: String,
    name_type: String,
    name_country: String,
    iso_country: String,
    name_town: String,
    geo_lat: Option<String>,
    geo_lon: Option<String>,
    name_organizer: String,
    url_organizer: Option<String>,
    url_image_mall: Option<String>,
    url_image_medium: Option<String>,
    url_image_full: Option<String>,
    date_created: String,
    date_modified: String,
    name_status: String,
    url_party_html: String,
    url_party_json: String,
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
    #[command(description = "Return a list with the upcoming raves in the canary islands.")]
    Raves,
    #[command(description = "Return a list with the upcoming raves in Spain.")]
    TodasLasRaves,
}


async fn answer(
    cx: DispatcherHandlerCx<Message>,
    command: Command,
) -> ResponseResult<()> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Ping => cx.answer("Pong!").send().await?,
        Command::Raves => cx.answer(raves(false).await).send().await?,
        Command::TodasLasRaves => cx.answer(raves(true).await).send().await?,
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

async fn raves(all: bool) -> String {
    let data = block_in_place(|| get_parties());
    //TODO quitar unwrap
    let mut parsed: Parties = serde_json::from_str(&data.unwrap()).unwrap();
    if !all {
        parsed = filter_parties(parsed);
    }
    let message = format_parties(parsed); 
    //let ref name = parsed.partylist[0].nameParty;
    //return name.to_string();
    return message;

}

fn format_parties(data: Parties) -> String {
    let mut output: String = Default::default();
    for party in data.partylist {
        let date_start = DateTime::parse_from_rfc3339(party.date_start.as_str()).unwrap_or(FixedOffset::east(0).ymd(1970, 1, 1).and_hms(0, 0, 0));
        write!(&mut output, "{}\n{}\n{}\n",party.name_party, date_start.format("%-d %B %H:%M"), party.name_town);
    }
    return output;
}

// Return only the parties that are in the canary islands
fn filter_parties(data: Parties) -> Parties {
    let mut filtered: Parties = Default::default();
    for party in data.partylist {
        //TODO quitar unwrap
        let geo_lat = party.geo_lat.clone();
        let geo_lon = party.geo_lon.clone();
        if geo_lat.is_some() && geo_lon.is_some() {
            let geo_lat = geo_lat.unwrap().parse::<f64>().unwrap();
            let geo_lon = geo_lon.unwrap().parse::<f64>().unwrap();

            if geo_lat < 29.555 && geo_lat > 27.6145 {
                if geo_lon < -13.1956 && geo_lon > -18.540 {
                    filtered.partylist.push(party);
                    continue;
                }
            }
            // match using nameTown
            match party.name_town.to_lowercase().as_str() {
                "tenerife" => filtered.partylist.push(party),
                _ => continue,
            }
        }
        //29.555372, -13.265574 //arriba derecha
        //27.669400, -13.195601 //abajo derecha
        //27.614655, -18.239785// abajo izquierda
        //29.285276, -18.254011// arriba izquierda
        // Match using coordinates
    }
    return filtered;
}

#[test]
fn test_filter_parties(){
    let party1 = Party {
    id: String::from("00000"),
    name_party: String::from("nameParty1"),
    date_start: String::from("dateStart1"),
    date_end: String::from("dateEnd1"),
    name_type: String::from("nameType1"),
    name_country: String::from("nameCountry1"),
    iso_country: String::from("isoCountry1"),
    name_town: String::from("nameTown1"),
    geo_lat: Some(String::from("28.02")),
    geo_lon: Some(String::from("-16.01")),
    name_organizer: String::from("nameOrganizer1"),
    url_organizer: Some(String::from("urlOrganizer1")),
    url_image_small: Some(String::from("urlImageSmall1")),
    url_image_medium: Some(String::from("urlImageMedium1")),
    url_image_full: Some(String::from("urlImageFull1")),
    date_created: String::from("dateCreated1"),
    date_modified: String::from("dateModified1"),
    name_status: String::from("nameStatus1"),
    url_party_html: String::from("urlPartyHtml"),
    url_party_json: String::from("urlPartyJson1"),
    };
    
    let party2 = Party {
    id: String::from("00001"),
    name_party: String::from("nameParty2"),
    date_start: String::from("dateStart2"),
    date_end: String::from("dateEnd2"),
    name_type: String::from("nameType2"),
    name_country: String::from("nameCountry2"),
    iso_country: String::from("isoCountry2"),
    name_town: String::from("TENERIFE"),
    geo_lat: Some(String::from("-15.00")),
    geo_lon: Some(String::from("28.0")),
    name_organizer: String::from("nameOrganizer2"),
    url_organizer: Some(String::from("urlOrganizer1")),
    url_image_small: Some(String::from("urlImageSmall2")),
    url_image_medium: Some(String::from("urlImageMedium2")),
    url_image_full: Some(String::from("urlImageFull1")),
    date_created: String::from("dateCreated2"),
    date_modified: String::from("dateModified2"),
    name_status: String::from("nameStatus2"),
    url_party_html: String::from("urlPartyHtml2"),
    url_party_json: String::from("urlPartyJson"),
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

