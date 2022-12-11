extern crate regex;
use regex::Regex;

const CYM: [char; 6] = ['\'', '_', '+', ':', ',', '.'];
const CONN: [char; 1] = ['-'];

pub fn abbreviate(phrase: &str) -> String {
    //unimplemented!("Given the phrase '{}', return its acronym", phrase);

    let mut arms = String::new();
    for i in phrase.chars() {
        if CYM.contains(&i) {
            continue; // ignore cymbloes
        }
        if CONN.contains(&i) {
            arms.push(' '); // replace connect as space
            continue;
        }
        arms.push(i);
    }

    let rex = Regex::new(r"\b\w|[A-Z][a-z]").unwrap();
    let mut result: Vec<String> = Vec::new();
    for cap in rex.captures_iter(&arms) {
        result.push(cap[0][..1].into());
    }
    result.join("").to_uppercase()
}
