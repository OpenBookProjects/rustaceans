//use std::error::Error;
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

fn vlq_encoder(values: u32, out: &mut Vec<u8>) {
    (0..4)
        .map(|i| (values >> (7 * (4 - i))) & 0x7f)
        .skip_while(|&x| x == 0)
        .for_each(|x| out.push((x | 0x80) as u8));

    out.push((values & 0x7f) as u8)
}

fn vlq_decoder(bytes: &[u8]) -> Result<u32, Error> {
    match bytes.last() {
        None => Err(Error::IncompleteNumber),
        Some(&x) if x >= 0x80 => Err(Error::IncompleteNumber),
        Some(_) => bytes
            .iter()
            .map(|x| (x & 0x7f) as u32)
            .try_fold(0u32, |acc, x| Some(acc.checked_mul(128)? + x))
            .ok_or(Error::Overflow),
    }
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    //unimplemented!("Convert the values {values:?} to a list of bytes")
    let mut res = Vec::new();
    values.iter().for_each(|&v| vlq_encoder(v, &mut res));
    res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    //unimplemented!("Convert the list of bytes {bytes:?} to a list of numbers")
    let mut res = Vec::new();
    for chunk in bytes.split_inclusive(|&c| c < 0x80) {
        res.push(vlq_decoder(chunk)?);
    }
    Ok(res)
}
