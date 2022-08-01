use std::collections::HashMap;
use serde; 
use reqwest::header::HeaderMap;

use crate::response::{response_struct};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
/// This struct is used to create the payload of the request.
pub struct json_to_pass {
    points: Vec<HashMap<String, f64>>,
    fuelType: String,
    priceOrder: String,
}

impl json_to_pass {
    pub fn new(
        points: Vec<HashMap<String, f64>>,
        fuelType: String,
        priceOrder: String,
    ) -> json_to_pass {
        json_to_pass {
            points,
            fuelType,
            priceOrder,
        }
    }
}

pub async fn request(
    points: Vec<HashMap<String, f64>>,
) -> response_struct {

    let client = reqwest::Client::new();

    //setup the request
    const REQUEST_URL: &str = "https://carburanti.mise.gov.it/ospzApi/search/zone";
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
   
    //setup the payload
    // the fuel type is default 1 because the api return me 
    // always all the the types of fuel available in the station
    let fuelType: String = 1.to_string();
    let priceOrder: String = "asc".to_string();
    let payload = json_to_pass::new(points,fuelType, priceOrder);
    let res = client
        .post(REQUEST_URL)
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
