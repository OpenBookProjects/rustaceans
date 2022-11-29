use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    /*
    pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str>
    unimplemented!(
        "For the '{}' word find anagrams among the following words: {:?}",
        word,
        possible_anagrams
    ); */
    let words = get_sorted(word);
    //let word_sorted = get_sorted(&word_lower);
    possible_anagrams
        .iter()
        //.cloned() // converts &a to a
        .filter(|anagram| {
            //let anagram_lower = anagram.to_lowercase();
            let candidate = get_sorted(anagram);
            //candidate.0.len() == words.0.len() &&
            candidate.0 != words.0 && candidate.1 == words.1
        })
        .copied() // &a?
        .collect()
}

//fn get_sorted(word: &str) -> Vec<char> {
fn get_sorted(word: &str) -> (String, Vec<char>) {
    let lower = word.to_lowercase();
    let mut sorted: Vec<char> = lower.chars().collect();
    sorted.sort_unstable();
    (lower, sorted)
    //sorted
}
