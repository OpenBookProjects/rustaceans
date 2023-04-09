pub fn rotate(input: &str, key: i8) -> String {
    /* unimplemented!(
        "How would input text '{input}' transform when every letter is shifted using key '{key}'?"
    ); */
    let key = key.rem_euclid(26) as u8;
    input
        .bytes()
        .map(|c| match c {
                b'A'..=b'Z' => b'A' + (c - b'A' + key).rem_euclid(26),
                b'a'..=b'z' => b'a' + (c - b'a' + key).rem_euclid(26),
                _ => c,
            } as char)
        .collect()

}
