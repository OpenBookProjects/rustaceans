#![allow(unused)]
use matching_brackets::brackets_are_balanced;

//#[derive(Debug)]
fn main() {
    let mut chaos = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
                 \\end{array}\\right";
    println!("brackets_are_balanced:{}", brackets_are_balanced(chaos));
}
