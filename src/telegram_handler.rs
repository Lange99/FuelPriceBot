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
    },
}

impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}

pub async fn start(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "Welcome to Fuel Bot!\n send me your position ")
        .await?;
    dialogue.update(State::ReceiveLocation).await?;
    Ok(())
}

pub async fn receive_location(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    match msg.location() {
        Some(location) => {
            bot.send_message(msg.chat.id, "distance?").await?;
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
            bot.send_message(msg.chat.id, "select carburant's id")
                .await?;
            dialogue
                .update(State::ReceiveFuelType {
                    location: user_location,
                    distance: distance,
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
    (user_location, distance): (teloxide::types::Location, f64),
) -> HandlerResult {
    match msg.text().map(|text| text.parse::<f64>()) {
        Some(Ok(age)) => {
            bot.send_message(msg.chat.id, "Your location is:").await?;
            bot.send_location(msg.chat.id, user_location.latitude, user_location.longitude)
                .await?;
            bot.send_message(msg.chat.id, "Your fuel type is:").await?;
            bot.send_message(msg.chat.id, distance.to_string()).await?;
            bot.send_message(msg.chat.id, "Your max distance is:")
                .await?;
            bot.send_message(msg.chat.id, age.to_string()).await?;
            dialogue.update(State::Start).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me the id ")
                .await?;
        }
    }

    Ok(())
}


pub async fn conversation_handler(urlrequest:&str, header_map:reqwest::header::HeaderMap){
    
    let token = env::var("TELOXIDE_FUEL_TOKEN").expect("TELOXIDE_FUEL_TOKEN");
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
                dptree::case![State::ReceiveFuelType { location, distance }]
                    .endpoint(receive_id_fuel),
            ),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}