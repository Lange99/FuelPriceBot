
use crate::response::{location, response_struct, station};
use chrono::{DateTime, Utc};
use std::{borrow::BorrowMut, collections::HashMap, vec};

// this struct is a "wrapper" struct around station struct.
// i saved in this struct the distance between the station and the user and the price of the fuel for the user.
pub struct station_utility {
    station: station,
    distance: f64,
    price: f64,
}

impl station_utility {
    pub fn new(station: station, distance: f64, price: f64) -> station_utility {
        station_utility {
            station,
            distance,
            price,
        }
    }
}

fn calculate_distance(fistlocation: &Vec<HashMap<String, f64>>, station: station) -> f64 {
    let mut distance: f64 = 0.0;
    for point in fistlocation {
        let lat1: f64 = point["lat"].clone();
        let lng1: f64 = point["lng"].clone();
        let lat2: f64 = station.location.lat.clone();
        let lng2: f64 = station.location.lng.clone();
        distance += calculate_distance_between_points(lat1, lng1, lat2, lng2);
    }
    distance
}
fn calculate_distance_between_points(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let earth_radius: f64 = 6371.0;
    let dlat: f64 = (lat2 - lat1).to_radians();
    let dlng: f64 = (lng2 - lng1).to_radians();
    let a: f64 = (dlat / 2.0).sin().powi(2) + (dlng / 2.0).sin().powi(2) * lat1.cos() * lat2.cos();
    let c: f64 = 2.0 * a.sqrt().atan2(1.0);
    earth_radius * c
}

/// this function will return a vector of station_utility objects
/// first the function will calculate the distance between the fist location and all the stations
/// then it will calculate the price for the fuel type and return a vector of station_utility objects
pub fn setup_data(
    response: response_struct,
    max_distance: f64,
    id_fuel: i16,
    userlocation: Vec<HashMap<String, f64>>,
) -> Vec<station_utility> {
    let mut stations: Vec<station_utility> = Vec::new();
    let updated_station = delete_not_updated_stations(response);
    for station in &updated_station {
        let distance: f64 = calculate_distance(&userlocation, station.clone());
        if distance <= max_distance {
            let price: f64 = station.get_price_for_fuel(id_fuel);
            stations.push(station_utility::new(station.clone(), distance, price));
        }
    }
    stations
}

/// this function will return a vector of station objects
/// first i setup the vector of station objects with the updated stations near the fist location
/// next I sort the vector by the price of the fuel type
/// # Arguments
/// * `response` - the response from the api
/// * `max_distance` - the max distance from the fist location
/// * `id_fuel` - the id of the fuel type
/// * `userlocation` - the location of the user
/// # Returns
/// * `Vec<station>` - the vector of station objects sorted by the price of the fuel type
pub fn get_best_stations(
    response: response_struct,
    max_distance: f64,
    id_fuel: i16,
    userlocation: Vec<HashMap<String, f64>>,
) -> Vec<station> {
    let mut stations = setup_data(response, max_distance, id_fuel, userlocation);
    stations.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap()); //sort by price
    let mut best_stations: Vec<station> = Vec::new();
    for station in &stations {
        best_stations.push(station.station.clone());
    }
    best_stations
}

fn delete_not_updated_stations(response: response_struct) -> Vec<station> {
    let mut stations: Vec<station> = Vec::new();
    let today = Utc::now();
    let today_string = today.format("%Y-%m-%d").to_string();
    println!("today_string: {}", today_string);
    for station in &response.results {
        if station.parse_date() == today_string {
            stations.push(station.clone());
        }
    }
    stations
}

pub fn get_type_fuel_inside_distance(stationList:Vec<station_utility>) -> HashMap<i64, String> {
    let mut type_fuel: HashMap<i64, String> = HashMap::new();
    for station in &stationList {
        for fuel in &station.station.fuels {
            if type_fuel.contains_key(&fuel.id) {
                continue;
            } else {
                type_fuel.insert(fuel.id, fuel.name.clone());
            }
        }
    }
    type_fuel
}
