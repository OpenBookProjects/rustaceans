pub fn answer(command: &str) -> Option<i32> {
    let tokens = command
                .split([' ','?'])
                .skip(2)
                .filter(|s| !s.is_empty());

    let mut tokens = tokens
                .map(|w| w.parse::<i32>().map_err(|_| w));

    //println!("{:?}",tokens);
    for token in tokens.clone() {
        match token {
            Ok(num) => println!("Parsed integer: {}", num),
            Err(word) => println!("Invalid word: {}", word),
        }
    }

    let mut left = tokens.next()?.ok()?;
    loop{
        let op = match (tokens.next(),left){
            (Some(Err(op)),_) => op,
            (None,x)=> return Some(x),
            _ => return None,
        };
        match op {
            "plus" => left += tokens.next()?.ok()?,
            "minus" => left -= tokens.next()?.ok()?,
            "multiplied" => left *= tokens.nth(1)?.ok()?,
            "divided" => {
                let div = tokens.nth(1)?.ok()?;
                left = left.checked_div(div)?;
            },
            _ => return None,
        }
    }
}
