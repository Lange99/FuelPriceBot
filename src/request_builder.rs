use std::collections::HashMap;

use reqwest::header::HeaderMap;

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



