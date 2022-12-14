#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
///
///

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    /*     unimplemented!(
        "Convert {:?} from base {} to base {}",
        number,
        from_base,
        to_base
    ) */
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    // fv: from value, merge number vec base from_base as one number
    // [Iterator in std::iter - Rust](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
    // [迭代与并发 - Rust 中的异步编程](https://huangjj27.github.io/async-book/05_streams/02_iteration_and_concurrency.html)
    let mut fv = number
        .iter()
        .rev()
        .enumerate()
        .try_fold(0, |cr, (i, &fv)| {
            if fv >= from_base {
                Err(Error::InvalidDigit(fv))
            } else {
                Ok(cr + fv * from_base.pow(i as u32))
            }
        })?;

    // base to_base, split fv as new base number
    let mut vec = vec![];
    loop {
        vec.push(fv % to_base);
        fv /= to_base;
        if fv == 0 {
            vec.reverse();
            return Ok(vec);
        }
    }
}
