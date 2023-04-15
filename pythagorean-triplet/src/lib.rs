use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    //unimplemented!("Given the sum {sum}, return all possible Pythagorean triplets, which produce the said sum, or an empty HashSet if there are no such triplets. Note that you are expected to return triplets in [a, b, c] order, where a < b < c");
    (1..sum/3)
        .flat_map(|a| ((a+1)..(sum-a+1)/2).map(move |b| [a, b, sum-a-b]))
        .filter(|[a, b, c]| a*a + b*b == c*c)
        .collect()
}
