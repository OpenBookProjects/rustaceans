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

    println!("l0's len:{}", l0.len());
    println!("l1's len:{}", l1.len());

    /*     let v1: Vec<u64> = (10..1_000_001).collect();
       let v2: Vec<u64> = (1..1_000_000).collect();
       println!("v1's len:{}", v1.len());
       println!("v2's len:{}", v2.len());
    */
    let v1: [u8; 3] = [1, 2, 3];
    let v2: [u8; 2] = [1, 3];
    println!("v1:{:?}", v1);
    println!("v2:{:?}", v2);

    if v1.len() == v2.len() {
        println!("try every elements equ?");
        for x in v1 {
            if v2.contains(&x) {
                continue;
            } else {
                println!("Unequal?");
            }
        }
        println!("v1 equal v2");
    } else {
        if v1.len() > v2.len() {
            println!("try v1 Superlist v2?");
            for x in v2 {
                if v1.contains(&x) {
                    continue;
                } else {
                    println!("Unequal?");
                }
            }
            println!("v1 Superlist v2");
        } else {
            println!("try v1 Sublist v2?");
            for x in v1 {
                if v2.contains(&x) {
                    continue;
                } else {
                    println!("Unequal?");
                }
            }
            println!("v1 Sublist v2");
        }
    }
    println!("Unequal?");

    let s = "123";
    let t = "13";
    let c = s.matches(t).count();
    println!("matches? {}", c);
}
