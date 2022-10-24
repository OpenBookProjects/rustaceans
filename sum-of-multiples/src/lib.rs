pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    /*
    unimplemented!(
        "Sum the multiples of all of {:?} which are less than {}",
        factors,
        limit
    ) */

    let mut sum4m = 0;
    for numb in 0..limit {
        for f in factors {
            if *f > 0 && numb % *f == 0 {
                sum4m += numb;
                break;
            }
        }
    }
    sum4m
}
