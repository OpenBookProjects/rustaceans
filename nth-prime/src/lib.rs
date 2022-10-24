macro_rules! range {
    ($stop:expr) => {
        0..$stop
    };
    ($start:expr, $stop:expr) => {
        $start..$stop
    };
    ($start:expr, $stop:expr, -$step:expr) => {
        ($stop + 1..$start + 1).rev().step_by($step)
    };
    ($start:expr, $stop:expr, $step:expr) => {
        ($start..$stop).step_by($step)
    };
}

pub fn chk_prime(value: u32) -> bool {
    if value == 2 || value == 3 {
        return true;
    }

    if value % 2 == 0 || value % 3 == 0 {
        return false;
    }
    /*
    any int can make as any of:
        6n
        6n+1
        ..
        6n+5
    except 2,3, only 6n+1,6n+5 is prime
    if not, must include 6n+1||6n+5
    */

    if value % 6 != 1 && value % 6 != 5 {
        return false;
    }

    for i in range!(5, ((value as f64).sqrt() as u32) + 1, 6) {
        if value % i == 0 || value % (i + 2) == 0 {
            return false;
        }
    }

    true

    /*
       let mut divisor = 6;
       while divisor * divisor - 2 * divisor < value {
           if value % (divisor - 1) == 0 || value % (divisor + 1) == 0 {
               return false;
           }
           divisor += 6;
       }
    */
}

pub fn nth(n: u32) -> u32 {
    //unimplemented!("What is the 0-indexed {}th prime number?", n)

    let mut numb = 0;
    let mut as_prime = 2;

    while numb < n {
        as_prime += 1;
        if chk_prime(as_prime) {
            numb += 1;
        }
    }

    as_prime
}
