pub fn actions(n: u8) -> Vec<&'static str> {
    //unimplemented!("What is the secret handshake for {n}?")
    let mut result = Vec::new();
    if n & 0b0001 != 0{
        result.push("wink");
    }
    if n & 0b0010 != 0{
        result.push("double blink");
    }
    if n & 0b0100 != 0{
        result.push("close your eyes");
    }
    if n & 0b1000 != 0{
        result.push("jump");
    }
    if n & 0b10000 != 0{
        result.reverse();
    }
    result
}
