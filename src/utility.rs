use crate::response::{response_struct, station};
use chrono::Utc;
use std::collections::HashMap;

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

fn calculate_distance(firstlocation: &Vec<HashMap<String, f64>>, station: station) -> f64 {
    let lat1: f64 = firstlocation[0]["lat"].clone();
    let lng1: f64 = firstlocation[0]["lng"].clone();
    let lat2: f64 = station.location.lat.clone();
    let lng2: f64 = station.location.lng.clone();
    calculate_distance_between_points(lat1, lng1, lat2, lng2)
}
fn calculate_distance_between_points(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let earth_radius: f64 = 6371.0;
    let dlat1 = lat1.to_radians();
    let dlng1 = lng1.to_radians();
    let dlat2 = lat2.to_radians();
    let dlng2 = lng2.to_radians();

    let dlat = dlat2 - dlat1;
    let dlng = dlng2 - dlng1;

    let a: f64 =
        (dlat / 2.0).sin().powi(2) + (dlng / 2.0).sin().powi(2) * dlat1.cos() * dlat2.cos();
    let c: f64 = 2.0 * a.sqrt().asin();

    earth_radius * c
}

/// this function will return a vector of station_utility objects
/// first the function will calculate the distance between the fist location and all the stations
/// then it will calculate the price for the fuel type and return a vector of station_utility objects
fn setup_data(
    response: response_struct,
    max_distance: f64,
    id_fuel: i16,
    userlocation: Vec<HashMap<String, f64>>,
) -> Vec<station_utility> {
    let mut stations: Vec<station_utility> = Vec::new();
    print!("{}", response.results.len().to_string());
    let updated_station = delete_not_updated_stations(response);
    for station in &updated_station {
        let distance: f64 = calculate_distance(&userlocation, station.clone());

        let price: f64 = station.get_price_for_fuel(id_fuel);
        //println!("{}", distance);
        stations.push(station_utility::new(station.clone(), distance, price));
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
/// * 'String' - a formatted string with the stations near the fist location
pub fn get_best_stations(
    response: response_struct,
    max_distance: f64,
    id_fuel: i16,
    userlocation: Vec<HashMap<String, f64>>,
) -> String {
    let mut stations = setup_data(response, max_distance, id_fuel, userlocation);
    stations.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap()); //sort by price
    let mut sum_price: f64 = 0.0;
    let average_price: f64;
    let mut i = 0;
    //remove the stations with a price of 0.0
    while i < stations.len() {
        if stations[i].price == 0.0 {
            stations.remove(i);
        } else {
            sum_price = sum_price + stations[i].price;
            i = i + 1;
        }
    }
    average_price = sum_price / stations.len() as f64;
    return print_best_stations_info(stations, average_price);
}

fn delete_not_updated_stations(response: response_struct) -> Vec<station> {
    let mut stations: Vec<station> = Vec::new();
    let today = Utc::now();
    let today_string = today.format("%Y-%m-%d").to_string();
    let yesterday = today - chrono::Duration::days(1);
    let yesterday_string = yesterday.format("%Y-%m-%d").to_string();
    println!("today_string: {}", today_string);
    println!("yesterday_string: {}", yesterday_string);
    for station in &response.results {
        if station.parse_date() == today_string || station.parse_date() == yesterday_string {
            stations.push(station.clone());
        }
    }
    stations
}
/// This method will return a string with the avaible fuel types for the stations 
/// # Arguments
/// * 'response' - the response from the api
///
/// # Returns
/// * 'String' - a formatted string with the avaible fuel types for the stations near the fist location
///
pub fn get_fuel_type(
    response: response_struct,
) -> String {
    let mut type_fuel: HashMap<i16, String> = HashMap::new();
    println!("{} stations", response.results.len());
    for station in &response.results {
        for fuel in &station.fuels {
            let id_fuel: i16 = fuel.fuelId.clone();
            let fuel_type: String = fuel.name.clone();
            type_fuel.insert(id_fuel, fuel_type);
        }
    }
    string_of_type_fuel(type_fuel)
}

fn string_of_type_fuel(type_fuel: HashMap<i16, String>) -> String {
    let mut string_type_fuel: String = String::new();
    string_type_fuel.push_str(&format!("{} - exit | \n", 0));
    let mut vec: Vec<(&i16, &String)> = type_fuel.iter().collect();
    vec.sort_by_key(|k| *k.0);

    for (id, name) in vec {
        string_type_fuel.push_str(&format!("{} - {} | \n", id, name));
    }
    string_type_fuel
}

fn print_best_stations_info(best_stations: Vec<station_utility>, avg: f64) -> String {
    let mut final_string = String::new();
    let mut counter = 0;
    final_string.push_str(&format!(" | AVERAGE PRICE: {:.3} | ", avg));
    for station in &best_stations {
        let mut temp = String::new();
        counter += 1;
        temp.push_str(&format!("{}", counter));
        temp.push_str("-> || ");
        temp.push_str("NAME: ");
        temp.push_str(&format!("{}", station.station.name.to_lowercase()));
        temp.push_str("; || ADDRESS: ");
        temp.push_str(&format!("{}", station.station.address.to_lowercase()));
        temp.push_str("; || MAPS URL: ");
        temp.push_str(&format!(
            "https://www.google.com/maps/search/?api=1&query={}%2C{}",
            station.station.location.lat, station.station.location.lng
        ));
        temp.push_str(" ; || DISTANCE: ");
        temp.push_str(&format!("{:.3} km", station.distance));
        temp.push_str("; || PRICE: ");
        temp.push_str(&format!("{}€", station.price));
        temp.push_str(" | ");
        final_string.push_str(&temp);
    }

    final_string
}
