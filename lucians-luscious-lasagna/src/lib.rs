// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn expected_minutes_in_oven() -> i32 {
    //unimplemented!("return expected minutes in the oven")
    let m40 = 40;
    m40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    /*
    unimplemented!(
        "calculate remaining minutes in oven given actual minutes in oven: {}",
        actual_minutes_in_oven
    );
    */

    let book_time: i32 = expected_minutes_in_oven();
    //println!("{}", book_time);
    let remaining: i32 = book_time - actual_minutes_in_oven;
    remaining
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    //    unimplemented!(
    //        "calculate preparation time in minutes for number of layers: {}",
    //        number_of_layers
    //    )

    let need_preparation: i32 = number_of_layers * 2;
    need_preparation
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    /*     unimplemented!(
        "calculate elapsed time in minutes for number of layers {} and actual minutes in oven {}",
        number_of_layers,
        actual_minutes_in_oven
    ) */
    let need_preparation: i32 = preparation_time_in_minutes(number_of_layers);
    //let remaining: i32 = expected_minutes_in_oven() - actual_minutes_in_oven;
    let elapsed_time: i32 = need_preparation + actual_minutes_in_oven;
    elapsed_time
}
