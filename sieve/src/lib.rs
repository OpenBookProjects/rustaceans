pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    //unimplemented!("Construct a vector of all primes up to {upper_bound}");
    let mut primes = vec![];
    for numb in 2..=upper_bound{
        if ! primes.iter().any(|x| numb%x==0){
            primes.push(numb)
        }else{
            continue
        }
    }
    primes
}
