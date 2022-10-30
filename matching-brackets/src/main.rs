#![allow(unused)]
use matching_brackets::brackets_are_balanced;

const BRACKETS_OK: [char; 6] = ['[', '(', '{', ']', ')', '}'];
const BRACKETS_PARA: [&str; 3] = ["[]", "()", "{}"];

//#[derive(Debug)]
pub fn chkp(string: &str) -> String {
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

fn main() {
    let mut chaos = "[({]})";
    //println!("brackets_are_balanced:{}", brackets_are_balanced(chaos));

    println!("chkp: {}", chkp(chaos));
}
