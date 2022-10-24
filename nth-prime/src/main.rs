use nth_prime as np;

macro_rules! range {
    ($stop:expr) => {
        0..$stop
    };
    ($start:expr, $stop:expr) => {
        $start..$stop
    };
    ($start:expr, $stop:expr, -$step:expr) => {
        ($stop + 1..$start + 1).rev().step_by($step)
    };
    ($start:expr, $stop:expr, $step:expr) => {
        ($start..$stop).step_by($step)
    };
}

pub fn main() {
    println!("chk_prime:0=>{}", np::chk_prime(0));

    for i in range!(5, 60, 6) {
        println!("{}", i);
    }

    println!("{}", 5_f64.sqrt() as i32)
}
