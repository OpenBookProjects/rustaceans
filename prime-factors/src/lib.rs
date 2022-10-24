pub fn factors(n: u64) -> Vec<u64> {
    //unimplemented!("This should calculate the prime factors of {}", n)

    let mut primes: Vec<u64> = Vec::new();
    let mut number = n;
    let mut divider = 2;

    while number != 1 {
        if number % divider != 0 {
            divider += 1;
            continue;
        }

        primes.push(divider);
        number /= divider;
    }

    primes
}
