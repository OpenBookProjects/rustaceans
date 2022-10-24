pub fn raindrops(n: u32) -> String {
    //unimplemented!("what sound does Raindrop #{} make?", n)

    let mut dropping = "".to_string();
    if n % 3 == 0 {
        dropping.push_str("Pling")
    }
    if n % 5 == 0 {
        dropping.push_str("Plang")
    }
    if n % 7 == 0 {
        dropping.push_str("Plong")
    }
    if dropping.is_empty() {
        dropping = n.to_string()
    }
    dropping
}
