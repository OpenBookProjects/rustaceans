pub fn encrypt(input: &str) -> String {
    //unimplemented!("Encrypt {input:?} using a square code")
    if input.len() == 0 {
        //return String::from("");
        return String::new();
    }
    let plain = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<_>>();
    let row = (plain.len() as f64).sqrt().ceil() as usize;

    (0..row).map(|i|{
        plain
            .chunks(row)
            .filter_map(|v| v.get(i).or(Some(&' ')))
            .collect::<String>()
    }).collect::<Vec<_>>().join(" ")
}
