#![allow(unused)]
use assembly_line::{production_rate_per_hour, working_items_per_minute};

//#[derive(Debug)]
fn main() {
    println!("production_rate_per_hour:{}", production_rate_per_hour(6));
    println!("working_items_per_minute:{}", working_items_per_minute(8));
}
