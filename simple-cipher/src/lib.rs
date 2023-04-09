use rand::Rng;
use rand::thread_rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    //unimplemented!("Use {key} to encode {s} using shift cipher")
    if key.is_empty() || key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }
    let bytes = s
        .bytes()
        .zip(key.bytes().cycle())
        .map(|(c, k)| b'a' + ((c-b'a') + (k-b'a')).rem_euclid(26));
    Some(String::from_utf8(bytes.collect()).unwrap())
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    //unimplemented!("Use {key} to decode {s} using shift cipher")
    if key.is_empty() || key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }
    let bytes = s
        .bytes()
        .zip(key.bytes().cycle())
        .map(|(c, k)| b'a' + ((c-b'a') + 26 - (k-b'a')).rem_euclid(26));
    Some(String::from_utf8(bytes.collect()).unwrap())
}

pub fn encode_random(s: &str) -> (String, String) {
    /* unimplemented!(
        "Generate random key with only a-z chars and encode {s}. Return tuple (key, encoded s)"
    ) */
    let mut rng = thread_rng();
    let key: String = (0..100)
            .map(|_| rng.gen_range(b'a'..=b'z') as char)
            .collect();
    let ciphertxt = encode(&key, s).unwrap();
    (key, ciphertxt)
}
