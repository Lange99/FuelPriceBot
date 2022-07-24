use std::collections::HashMap;

use reqwest::header::HeaderMap;


pub use response::response_struct;


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

pub async fn request(url: &str, headers: HeaderMap,points: Vec<HashMap<String, f64>>,
    fuelType: String,
    priceOrder: String) -> response_struct {
    let client = reqwest::Client::new();
    let payload = json_to_pass::new(points, fuelType, priceOrder);
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



