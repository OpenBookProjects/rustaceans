#![allow(unused)]

const BRACKETS_OK: [char; 6] = ['[', '(', '{', ']', ')', '}'];
const BRACKETS_PARA: [&str; 3] = ["[]", "()", "{}"];

pub fn para_filter(string: &str) -> String {
    let mut chk_brackets = String::new();
    for i in string.chars() {
        if BRACKETS_OK.contains(&i) {
            chk_brackets.push(i);
            if chk_brackets.len() >= 2 {
                if BRACKETS_PARA.contains(&&chk_brackets[chk_brackets.len() - 2..]) {
                    chk_brackets.pop();
                    chk_brackets.pop();
                }
            }
        }
    }
    chk_brackets
}

pub fn brackets_are_balanced(string: &str) -> bool {
    /*     unimplemented!(
                   "Check if the string \"{}\" contains balanced brackets",
                   string
               );
    Rust Substring Examples - Dot Net Perls
    https://www.dotnetperls.com/substring-rust
    */

    //let mut chk_brackets = String::new();
    if string.len() == 0 {
        true
    } else {
        let chk_brackets = para_filter(string);

        if chk_brackets.len() == 0 {
            true
        } else {
            false
        }
    }
}
