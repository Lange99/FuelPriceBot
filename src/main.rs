use std::collections::HashMap;
use std::fmt::format;
use std::future;
use std::hash::Hash;

use futures::executor::block_on;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Error, Response};

pub mod request_builder;
pub mod response;
pub mod utility;
pub use request_builder::json_to_pass;
pub use response::response_struct;
pub use utility::get_best_stations;

use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use serde_json::json;


/* TODO:
* INTERFACCIA TELEGRAM
 */

 
fn main() {
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

    //PAYLOAD
    // this part is temporary, it will be replaced by a real payload provided by the user
    let lat = 45.5332742;
    let lng = 10.2121261;
    let fuelType: String = "1-x".to_string();
    let priceOrder: String = "asc".to_string();

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
    let response = rt.block_on(request(REQUEST_URL, headers, payload));
    //println!("{:?}", response);

    let best_station = utility::get_best_stations(response, 10.0, 1, position_vec_clone);
    println!("{:?}", best_station);
}

async fn request(url: &str, headers: HeaderMap, payload: json_to_pass) -> response_struct {
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await;
    match res {
        Ok(res) => {
            let finres = res
                .json::<response_struct>()
                .await
                .expect("error in parsing");
            return finres;
        }
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
