/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    //unimplemented!("What is the Hamming Distance between {} and {}", s1, s2);
    match s1.len().eq(&s2.len()){
        true => Some(s1
                        .chars()
                        .zip(s2.chars())
                        .filter(|(l,r)| l!=r)
                        .count()
                    ),
        _ => None,
    }
}
