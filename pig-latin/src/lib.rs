pub fn translate(input: &str) -> String {
    /* unimplemented!(
        "Using the Pig Latin text transformation rules, convert the given input '{}'",
        input
    ); */
    let words = input
        .split_ascii_whitespace()
        .map(pig2latin)
        .collect::<Vec<_>>();
    words.join(" ")
}

fn pig2latin(s: &str) -> String{
    let mut vowel = s
        .find(|c| ['a','e','i','o','u'].contains(&c))
        .unwrap_or_else(||s.find(|c| c=='y').unwrap());
    if s.starts_with("xr") || s.starts_with("yt"){
        vowel = 0;
    }
    if vowel>0 && &s[vowel - 1..=vowel] == "qu"{
        vowel+=1;
    }
    s[vowel..].to_owned()+&s[..vowel]+"ay"
}
