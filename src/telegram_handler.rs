use crate::request_builder::{self, request};
use crate::response::{self, response_struct};
use crate::utility::{self, get_best_stations, get_type_fuel_inside_distance};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::env;
/**
* {
  "327":"Supreme Diesel",
  "323":"L-GNC",
  "324":"GNL",
  "7":"Benzina WR 100",
  "12":"Benzina Plus 98",
  "1":"Benzina",
  "27":"Gasolio speciale",
  "28":"HiQ Perform+",
  "20":"Blue Diesel",
  "13":"Gasolio Oro Diesel",
  "308":"S-Diesel",
  "341":"Excellium Diesel",
  "3":"Metano",
  "328":"E-DIESEL",
  "10":"Gasolio Premium",
  "5":"Blue Super",
  "2":"Gasolio",
  "4":"GPL",

*
*/
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::Location};
type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
pub enum State {
    Start,
    ReceiveLocation,
    ReceiveMaxDistance {
        location: teloxide::types::Location,
    },
    ReceiveFuelType {
        location: teloxide::types::Location,
        distance: f64,
        request: response::response_struct,
    },
}

impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}

pub async fn start(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "Welcome to Fuel Bot!\nSend me your position ")
        .await?;
    dialogue.update(State::ReceiveLocation).await?;
    Ok(())
}

pub async fn receive_location(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
) -> HandlerResult {
    match msg.location() {
        Some(location) => {
            bot.send_message(
                msg.chat.id,
                "how far do you want to search for distributors? (distance in km)?",
            )
            .await?;
            dialogue
                .update(State::ReceiveMaxDistance {
                    location: *location,
                })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please send me your location")
                .await?;
        }
    }

    Ok(())
}

pub async fn receive_max_distance(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    user_location: Location,
) -> HandlerResult {
    match msg.text().map(|text| text.parse::<f64>()) {
        Some(Ok(distance)) => {
            let mut points: HashMap<String, f64> = HashMap::new();
            points.insert("lat".to_string(), user_location.latitude);
            points.insert("lng".to_string(), user_location.longitude);
            let points = vec![points];

            // request:
            let request = request(points.clone()).await;
            let response = request.clone();
            println!("{:?}", points);
            let fuel_to_print = get_type_fuel_inside_distance(request, distance, points);
            let question = format!("Select the id of the fuel: | \n{}", fuel_to_print);
            let messages = split_message(&question);
            for message in messages {
                bot.send_message(msg.chat.id, message).await?;
            }
            dialogue
                .update(State::ReceiveFuelType {
                    location: user_location,
                    distance: distance,
                    request: response,
                })
                .await
                .expect("Error updating state");
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me the distance")
                .await?;
        }
    }

    Ok(())
}

pub async fn receive_id_fuel(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    (user_location, distance, request): (teloxide::types::Location, f64, response_struct),
) -> HandlerResult {
    match msg.text().map(|text| text.parse::<i16>()) {
        Some(Ok(id_fuel)) => {
            let mut points: HashMap<String, f64> = HashMap::new();
            points.insert("lat".to_string(), user_location.latitude);
            points.insert("lng".to_string(), user_location.longitude);
            let points = vec![points];
            let request = request.clone();
            let best_stations = get_best_stations(request, distance, id_fuel, points);
            let final_response = format!("The best station are: | \n{}", best_stations);
            let messages_to_send = split_message(&final_response);
            for message in messages_to_send {
                bot.send_message(msg.chat.id, message).await?;
            }

            dialogue.update(State::Start).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me the id ").await?;
        }
    }

    Ok(())
}

fn split_message(message: &str) -> Vec<String> {
    //max caracters per message
    let max_caracters = 4096;
    let mut messages = Vec::new();
    let mut message_to_send = String::new();
    let mut counter = 0;
    for word in message.split_whitespace() {
        if counter + word.len() > max_caracters {
            messages.push(message_to_send);
            message_to_send = String::new();
            counter = 0;
        }
        if word == "|" {
            message_to_send.push_str("\n\n");
        } else if word == "||" {
            message_to_send.push_str("\n");
        } else {
            message_to_send.push_str(word);
        }
        message_to_send.push(' ');

        counter += word.len() + 1;
    }
    messages.push(message_to_send);
    messages
}

pub async fn conversation_handler() {
    let token = env::var("TELOXIDE_FUEL_TOKEN").expect("missing: TELOXIDE_FUEL_TOKEN");
    let bot = Bot::new(token).auto_send();
    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start))
            .branch(dptree::case![State::ReceiveLocation].endpoint(receive_location))
            .branch(
                dptree::case![State::ReceiveMaxDistance { location }]
                    .endpoint(receive_max_distance),
            )
            .branch(
                dptree::case![State::ReceiveFuelType {
                    location,
                    distance,
                    request
                }]
                .endpoint(receive_id_fuel),
            ),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
