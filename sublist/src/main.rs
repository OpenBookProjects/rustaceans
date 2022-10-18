#![allow(unused)]
//#[derive(Debug, PartialEq, Eq)]
use sublist::{sublist, Comparison};

//#[derive(Debug, PartialEq, Eq)]
fn main() {
    use Comparison::*;

    let l0: [char; 0] = [];
    let l1: [char; 4] = ['a', 's', 'd', 'f'];

    println!("{:?}", l1);

    let chk = 'a';
    if l1.contains(&chk) {
        println!("{:?} contains {}", l1, chk);
    } else {
        println!("{:?} doesn't contain {}", l1, chk);
    }
    //info("Timezone changed");
}
