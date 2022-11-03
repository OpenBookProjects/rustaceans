pub fn series(digits: &str, len: usize) -> Vec<String> {
    /*  unimplemented!(
        "What are the series of length {} in string {:?}",
        len,
        digits
    ) */
    let mut chips = Vec::new();
    if len > digits.len() {
        return chips;
    }
    for index in 0..digits.len() + 1 - len {
        chips.push(digits[index..index + len].to_string());
    }
    chips
}
