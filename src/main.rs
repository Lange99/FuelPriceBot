use std::collections::HashMap;
use std::fmt::format;
use std::hash::Hash;
use std::{env, future};

use futures::executor::block_on;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Error, Response};

pub mod request_builder;
pub mod response;
pub use utility::get_best_stations;
pub mod utility;
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use serde_json::json;

use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::Location};
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
pub mod telegram_handler;
pub use telegram_handler::conversation_handler;

/* TODO:
* INTERFACCIA TELEGRAM
 */

#[tokio::main]
async fn main() {
    /// URL
    const REQUEST_URL: &str = "https://carburanti.mise.gov.it/ospzApi/search/zone";

    ///HEADER
    const REQUEST_HOST: &str = "carburanti.mise.gov.it";
    const REQUEST_ACCEPT: &str = "application/json";
    const REQUEST_CONTENT_TYPE: &str = "application/json";
    const REQUEST_ORIGIN: &str = "https://carburanti.mise.gov.it";
    const REQUEST_REFERER: &str = "https://carburanti.mise.gov.it/ospzSearch/zona";

    let mut headers = HeaderMap::new();
    headers.insert("host", REQUEST_HOST.parse().unwrap());
    headers.insert("accept", REQUEST_ACCEPT.parse().unwrap());
    headers.insert("content-type", REQUEST_CONTENT_TYPE.parse().unwrap());
    headers.insert("origin", REQUEST_ORIGIN.parse().unwrap());
    headers.insert("referer", REQUEST_REFERER.parse().unwrap());

    //INITIALIZE TELEGRAM BOT
    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    //telegram handler
    loop {
        telegram_handler::conversation_handler().await;
    }
}

/*
let mut hashCoo = HashMap::new();
hashCoo.insert("lat".into(), lat);
hashCoo.insert("lng".into(), lng);

let position_vec = vec![hashCoo];
let position_vec_clone = position_vec.clone();


let payload = json_to_pass::new(position_vec, fuelType, priceOrder);

//need tokio runtime
let rt = tokio::runtime::Builder::new_current_thread()
    .enable_io()
    .enable_time()
    .build()
    .unwrap();
let response = rt.block_on(request(REQUEST_URL, headers, payload)); */
