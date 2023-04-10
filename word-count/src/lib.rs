use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    //unimplemented!("Count of occurrences of words in {words:?}");
    words
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_matches('\'').to_lowercase())
        .fold(HashMap::new(), |mut acc,w|{
            let pt = acc.entry(w).or_insert(0);
            *pt += 1;
            acc
        })
}
