#![allow(unused)]
use lucians_luscious_lasagna::{
    elapsed_time_in_minutes, expected_minutes_in_oven, preparation_time_in_minutes,
    remaining_minutes_in_oven,
};

//#[derive(Debug)]
fn main() {
    println!("expected_minutes_in_oven:{}", expected_minutes_in_oven());
    println!(
        "remaining_minutes_in_oven:{}",
        remaining_minutes_in_oven(25)
    );
    println!(
        "preparation_time_in_minutes:{}",
        preparation_time_in_minutes(4)
    );
    println!("elapsed_time_in_minutes:{}", elapsed_time_in_minutes(3, 20));
}
