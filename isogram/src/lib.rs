use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    //unimplemented!("Is {} an isogram?", candidate);
    let mut hs: HashSet<char> = HashSet::new();

    candidate
        //.bytes()
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        //.map(|c| c.to_ascii_lowercase())
        .all(|c| hs.insert(c))
}
