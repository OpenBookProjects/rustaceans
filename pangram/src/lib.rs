/// Determine whether a sentence is a pangram.
// The quick brown fox jumps over the lazy dog.
pub fn is_pangram(sentence: &str) -> bool {
    //unimplemented!("Is {} a pangram?", sentence);
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .all(|w| sentence.to_lowercase().contains(w))
    // all word invluded sentence
}
