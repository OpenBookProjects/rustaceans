pub fn verse(n: u32) -> String {
    //unimplemented!("emit verse {}", n)

    let msg_first: String = match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.".to_owned(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.".to_owned(),
        _ => format!(
            "{n} bottles of beer on the wall, {n} bottles of beer.",
            n = n
        ),
    };
    let i32n: i32 = n.try_into().unwrap();
    let msg_second: String = match i32n - 1 {
        -1 => "Go to the store and buy some more, 99 bottles of beer on the wall.".to_owned(),
        0 => "Take it down and pass it around, no more bottles of beer on the wall.".to_owned(),
        1 => "Take one down and pass it around, 1 bottle of beer on the wall.".to_owned(),
        _ => format!(
            "Take one down and pass it around, {} bottles of beer on the wall.",
            n - 1
        ),
    };
    return format!("{}\n{}\n", msg_first, msg_second);
}

pub fn sing(start: u32, end: u32) -> String {
    //unimplemented!("sing verses {} to {}, inclusive", start, end)

    let mut msgs = String::new();
    for n in (end..start + 1).rev() {
        msgs.push_str(&format!("{}\n", verse(n)));
    }
    return format!("{}\n", msgs.trim_end());
}
