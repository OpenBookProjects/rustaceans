/// "Encipher" with the Atbash cipher.

fn trans(text: &str) -> Vec<char>{
    text.to_lowercase()
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric())
        .map(|c| match c {
            'a'..='z' => (b'z' - (c as u8 - b'a')) as char,
            '0'..='9' => c,
            _ => ' ',
        })
        .collect()
}

pub fn encode(plain: &str) -> String {
    //unimplemented!("Encoding of {plain:?} in Atbash cipher.");
    trans(plain)
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    //unimplemented!("Decoding of {cipher:?} in Atbash cipher.");
    trans(cipher).iter().collect()
}
