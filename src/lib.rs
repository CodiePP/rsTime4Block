use wasm_bindgen::prelude::*;

use chrono::{NaiveDate, DateTime};

const K_VALUE: i64 = 21600; // network parameter
const S_VALUE: i64 = 20;    // time in seconds per block

fn launch_time_as_unix_time() -> i64 {
    let t0 = NaiveDate::from_ymd_opt(2017, 9, 23).unwrap().and_hms_opt(21, 44, 51).unwrap().and_utc();
    t0.timestamp()
}

fn convert_timestamp_to_unix_time(timestamp_str: &str) -> i64 {

    // Parse the timestamp string
    return match DateTime::parse_from_rfc3339(timestamp_str) {
        Err(_err) => 0,
        Ok(datetime0) => 
            datetime0.to_utc().timestamp(),
    }
    
}

fn format_timestamp(t: i64) -> String {
    return match DateTime::from_timestamp(t, 0) {
        None => String::from("not a timestamp"),
        Some(dt) => dt.to_rfc3339(),
    }
}

// #[wasm_bindgen]
// pub fn get_time(t: &str) -> i32 {
//     convert_timestamp_to_unix_time(t) as i32
// }

#[wasm_bindgen]
pub fn to_epoch(t: &str) -> i32 {
    let t0 = launch_time_as_unix_time();
    let t1 = convert_timestamp_to_unix_time(t);
    let dt = t1 - t0;
    (dt / K_VALUE / S_VALUE) as i32
}

#[wasm_bindgen]
pub fn from_epoch(e: i32) -> JsValue {
    let t0 = launch_time_as_unix_time();
    let t1 = ((e as i64) * K_VALUE * S_VALUE) + t0;
    JsValue::from_str(&format_timestamp(t1))
}
