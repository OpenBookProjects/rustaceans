#![allow(unused)]
use low_power_embedded_game::{divmod, evens, Position};

//#[derive(Debug)]
fn main() {
    let out: Vec<i32> = evens((0..=10).rev()).collect();
    //let mut even_ints = println!("{}", evens((0..=10).rev()).next());
    println!("{:#?}", &out);
    //assert_eq!(even_ints.next(), Some(0));
    //info("Timezone changed");
    println!("manhattan(30,70) ~> {}", Position(30, 70).manhattan());
}
