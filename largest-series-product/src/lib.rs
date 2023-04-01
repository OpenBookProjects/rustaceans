#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}


pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    //unimplemented!("largest series product of a span of {span} digits in {string_digits}");
    match span {
        0 => Ok(1),
        _ => string_digits
            .chars()
            .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)).map(u64::from))
            .collect::<Result<Vec<_>, _>>()?
            .windows(span)
            .map(|w| w.iter().product())
            .max()
            .ok_or(Error::SpanTooLong),
    }
}


