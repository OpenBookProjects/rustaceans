use beer_song as beer;
fn main() {
    println!("{}", beer::verse(0));
    println!("{}", "~".repeat(42));
    println!("{}", beer::verse(1));
    println!("{}", "~".repeat(42));
    println!("{}", beer::sing(8, 6));
}
