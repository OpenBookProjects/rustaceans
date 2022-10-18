// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const PROD4H: f64 = 221.0;

const fn chk_success_rate(speed: u8) -> f64 {
    /*
    - `1` to `4`: 100% success rate.
    - `5` to `8`: 90% success rate.
    - `9` and `10`: 77% success rate.
        */
    match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    //unimplemented!("calculate hourly production rate at speed: {}", speed)
    PROD4H * (speed as f64) * chk_success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    //unimplemented!("calculate the amount of working items at speed: {}", speed)
    (production_rate_per_hour(speed) / 60.0) as u32
}
