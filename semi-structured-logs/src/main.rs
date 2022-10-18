#![allow(unused)]
use semi_structured_logs::{error, info, log, warn, LogLevel};

//#[derive(Debug)]
fn main() {
    use LogLevel::*;

    let log_mod: LogLevel = Info;

    match log_mod {
        Info => println!("LogLevel::Info"),
        Warning => println!("LogLevel::Warning"),
        Error => println!("LogLevel::Error"),
        _ => println!("UnKnow LogLevel"),
    }
    //println!("LogLevel:{}", Info);
    println!("{}", log(Info, "Timezone changed"));
    //info("Timezone changed");
}
