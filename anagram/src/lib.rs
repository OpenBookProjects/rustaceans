use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    /*
    unimplemented!(
        "For the '{}' word find anagrams among the following words: {:?}",
        word,
        possible_anagrams
    ); */
    let word_lower = word.to_lowercase();
    let word_sorted = get_sorted(&word_lower);
    possible_anagrams
        .iter()
        //.cloned() // converts &a to a
        .filter(|anagram| {
            let anagram_lower = anagram.to_lowercase();
            anagram_lower.len() == word_lower.len()
                && anagram_lower != word_lower
                && get_sorted(&anagram_lower) == word_sorted
        })
        .copied() // &a?
        .collect()
}

fn get_sorted(word: &str) -> Vec<char> {
    //let lower = word.to_lowercase();
    let mut sorted: Vec<char> = word.chars().collect();
    sorted.sort_unstable();
    //(lower, chars)
    sorted
}
