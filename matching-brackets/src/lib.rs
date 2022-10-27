#![allow(unused)]
//const BRACKETS_OPEN: [char; 3] = ['[', '(', '{'];
//const BRACKETS_CLOSE: [char; 3] = [']', ')', '}'];
const BRACKETS_OK: [char; 6] = ['[', '(', '{', ']', ')', '}'];
const BRACKETS_PARA: [&str; 3] = ["[]", "()", "{}"];

pub fn brackets_are_balanced(string: &str) -> bool {
    /*     unimplemented!(
                   "Check if the string \"{}\" contains balanced brackets",
                   string
               );
    Rust Substring Examples - Dot Net Perls
    https://www.dotnetperls.com/substring-rust
    */

    let mut chk_brackets = String::new();
    //println!("{}", BRACKETS_PARA[0]);
    //println!("string 's len={}", string.len());
    if string.len() == 0 {
        true
    } else {
        for i in string.chars() {
            //println!("i in str:{}", &i);

            if BRACKETS_OK.contains(&i) {
                chk_brackets.push(i);

                if chk_brackets.len() >= 2 {
                    /*  println!("now as >{}<", chk_brackets);
                    println!(
                        "i in str[-2:] ~> {}",
                        &chk_brackets[chk_brackets.len() - 2..]
                    );
                    println!(
                        "BRACKETS_PARA include? {}",
                        BRACKETS_PARA.contains(&&chk_brackets[chk_brackets.len() - 2..])
                    )
                     *///let tail2c = &chk_brackets[chk_brackets.len() - 2..];
                    if BRACKETS_PARA.contains(&&chk_brackets[chk_brackets.len() - 2..]) {
                        chk_brackets.pop();
                        chk_brackets.pop();
                    }
                }
            }
        }

        //println!("after checked->{}<-", chk_brackets);
        if chk_brackets.len() == 0 {
            true
        } else {
            false
        }
    }
}
