pub fn number(user_number: &str) -> Option<String> {
    /* unimplemented!(
        "Given the number entered by user '{user_number}', convert it into SMS-friendly format. If the entered number is not a valid NANP number, return None."
    ); */
    let mut fmt = user_number
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    if fmt.len() == 11 && fmt[0] == 1 {
        fmt.remove(0);
    }
    (fmt.len() == 10 && fmt[0] > 1 && fmt[3] >1 )
        .then_some(fmt
                    .iter()
                    .map(|c| c.to_string())
                    .collect())
}
