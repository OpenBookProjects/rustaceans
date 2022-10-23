pub fn square(s: u32) -> u64 {
    //unimplemented!("grains of rice on square {}", s);
    let su64 = s as u64;
    //su64.pow(s - 1)

    let alert = "Square must be between 1 and 64";
    match su64 {
        0 => panic!("{}", alert),
        1..=64 => 2_u64.pow(s - 1),
        _ => panic!("{}", alert),
    }
}

pub fn total() -> u64 {
    //(1..64).fold(0, |sum, item| sum + square(item - 1))
    square(64) + (square(64) - 1)
}
