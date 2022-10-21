use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    //unimplemented!("What time is a gigasecond later than {}", start);
    /*     Duration in std::time - Rust
    https://doc.rust-lang.org/std/time/struct.Duration.html
    */
    start + Duration::seconds(1_000_000_000)
}
