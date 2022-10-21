pub fn reverse(input: &str) -> String {
    //unimplemented!("Write a function to reverse {}", input);
    //let mut aim = "";
    //for c in input.chars().rev() {
    //    println!("{:?}", c);
    //    //aim = aim + c;
    //}
    //aim.to_string()
    input.chars().rev().collect()
}
