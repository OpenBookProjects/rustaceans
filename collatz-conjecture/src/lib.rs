use std::panic;

pub fn collatz(n: u64) -> Option<u64> {
    /* unimplemented!(
            "return Some(x) where x is the number of steps required to reach 1 starting with {}",
            n,
        )

        hread 'main' panicked at 'attempt to multiply with overflow', src/lib.rs:14:26

        catch_unwind in std::panic - Rust
    https://doc.rust-lang.org/std/panic/fn.catch_unwind.html

        */

    if n == 0 {
        return None;
    }

    let result = panic::catch_unwind(|| {
        let mut steps = 0;
        let mut cnumb = n;
        loop {
            match cnumb {
                1 => return steps,
                n if n % 2 == 0 => cnumb /= 2,
                _ => cnumb = cnumb * 3 + 1,
            }
            steps += 1;
        }
    });

    match result {
        Ok(steps) => Some(steps),
        Err(_) => None,
    }
}
