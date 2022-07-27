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


#[tokio::main]
async fn main() {

    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    //telegram handler
    loop {
        telegram_handler::conversation_handler().await;
    }
}
