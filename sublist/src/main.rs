#![allow(unused)]

use sublist::{is_sublist, sublist, Comparison};

use std::fmt::Write;

fn join(a: &[i32]) -> String {
    a.iter().fold(String::new(), |mut s, &n| {
        write!(s, "{}", n).ok();
        s
    })
}

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

    let v1 = [2, 5];
    let v2 = [1, 2, 3, 4];
    println!("v1:{:?}", v1);
    println!("v2:{:?}", v2);

    println!("v1 is v2 sublist? {}", is_sublist(&v1, &v2));
    //println!("{}", join(&v1));
    //let str4v1 = join(&v1);
    //let str4v2 = join(&v2);

    //let c = &str4v1.matches(&str4v2).count();
    //println!("matches? {}", c);

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
