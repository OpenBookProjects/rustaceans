pub fn is_armstrong_number(num: u32) -> bool {
    //unimplemented!("true if {} is an armstrong number", num)
    println!("{}'s len:{}", num, num.to_string().len());
    let p = num.to_string().len() as u32;
    println!("pow base {}", p);
    let mut an = 0;
    for n in num.to_string().chars() {
        println!("one by one {}", n);
        println!(
            "{} pow {} got {}",
            n.to_digit(10).unwrap_or(0),
            p,
            n.to_digit(10).unwrap_or(0).pow(p)
        );
        an += n.to_digit(10).unwrap_or(0).pow(p);
    }

    an == num
}
