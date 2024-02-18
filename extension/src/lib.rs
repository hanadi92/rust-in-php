use std::str::FromStr;
use ext_php_rs::prelude::*;

use chrono::NaiveTime;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Train {
    #[serde(rename = "Day")]
    day: u32,
    #[serde(rename = "ObsID")]
    obs_id: u128,
    #[serde(rename = "DepTime")]
    dep_time: String,
    #[serde(rename = "ArrTime")]
    arr_time: String,
    #[serde(rename = "DwellTime")]
    dwell_time: u32,
    #[serde(rename = "StopTime")]
    stop_time: u32,
    #[serde(rename = "Boarding")]
    boarding: u32,
    #[serde(rename = "Alighting")]
    a_lighting: u32,
    #[serde(rename = "CurrLoad")]
    curr_load: u32,
    #[serde(rename = "Speed")]
    speed: u32,
    #[serde(rename = "CoveredDistance")]
    covered_distance: u32,
    #[serde(rename = "RunTime")]
    run_time: u32,
    #[serde(rename = "RuntimeWithStopTime")]
    runtime_with_stop_time: u32,
    #[serde(rename = "SpeedWithStopTime")]
    speed_with_stop_time: u32,
    #[serde(rename = "ArrLoad")]
    arr_load: u32,
    #[serde(rename = "PassingTravellers")]
    passing_travellers: u32,
    #[serde(rename = "SeatUsage")]
    seat_usage: f32,
    #[serde(rename = "NumberOfVehicles")]
    number_of_vehicles: u32,
    #[serde(rename = "DepartureLineNumber")]
    departure_line_number: u32,
    #[serde(rename = "VehicleOwnerID")]
    vehicle_owner_id: String,
    #[serde(rename = "CarOrderPos")]
    car_order_pos: String,
    #[serde(rename = "StopName")]
    stop_name: String,
    #[serde(rename = "StopNumber")]
    stop_number: u32,
    #[serde(rename = "DepTerminal")]
    dep_terminal: String,
    #[serde(rename = "ArrTerminal")]
    arr_terminal: String,
}

impl Train {
    fn is_departing_soon(&self, current_time: &NaiveTime) -> bool {
        let train_departure_time = NaiveTime::from_str(&self.dep_time).unwrap();
        let time_difference = train_departure_time.signed_duration_since(*current_time);

        // Check if the train departure time is greater than the current time
        // and the time difference is less than or equal to 10 minutes (600 seconds).
        train_departure_time > *current_time && time_difference.num_seconds() <= 600
    }
    fn is_departing_from(&self, current_place: &String) -> bool {
        let train_departure_place = &self.dep_terminal;
        train_departure_place == current_place
    }
}

#[php_function]
fn find_departing_trains_php(
    current_time: &str,
    current_terminal: &str,
) -> String {
    let current_time = NaiveTime::from_str(current_time).unwrap();

    // Read CSV file in parallel
    let trains: Vec<Train> = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .flexible(true)
        .from_path("../../mini_data.csv")
        .expect("Error reading CSV file")
        .deserialize()
        .filter_map(|result| result.ok())
        .collect();

    // Filter departing trains based on time
    let departing_trains: Vec<&Train> = trains
        .iter()
        .filter(|train| train.is_departing_soon(&current_time) && train.is_departing_from(&current_terminal.to_string()))
        .collect();

    // Convert departing_trains to a string
    return serde_json::to_string(&departing_trains).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_php() {
        // Call the function and get the result
        let result = find_departing_trains_php("14:54:20", "Kungsängen");

        // Deserialize the JSON string into a vector of Train structs
        let departing_trains: Vec<Train> = serde_json::from_str(&result).unwrap();

        // Assert that the length of departing_trains is as expected
        assert_eq!(departing_trains.len(), 6);
    }
}
