/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    //unimplemented!("Is the Luhn checksum for {} valid?", code);
    let (mut total, mut i) = (0, 0);

    for c in code.chars().rev() {
        if c == ' ' {
            continue;
        } else if !c.is_digit(10) {
            return false;
        }

        let d = c.to_digit(10).unwrap() * ((i % 2) + 1);
        total += d;
        if d > 9 {
            total -= 9
        }
        i += 1;
    }
    (i > 1) && (total % 10 == 0)
}
