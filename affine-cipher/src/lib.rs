//#![feature(iter_intersperse)]

use modinverse::egcd;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const M: i32 =26;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    //unimplemented!("Encode {plaintext} with the key ({a}, {b})");
    let (g,_,_) = egcd(a,M);
    if g != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let letters: Vec<u8> = plaintext
        .bytes()
        //.filter(|&c| c.is_ascii_alphabetic())
        .filter(|&c| c.is_ascii_alphanumeric())
        .map(|c| match c {
            b'A'..=b'Z' => b'a'+((c-b'A') as i32*a+b).rem_euclid(M) as u8,
            b'a'..=b'z' => b'a'+((c-b'a') as i32*a+b).rem_euclid(M) as u8,
            b'0'..=b'9' => c,
            b => b,
            //_ => b' ',
            //_ => continue,
        })
        .collect();
    let chunks: Vec<&[u8]> = letters.chunks(5).collect();
    let ciphertext = String::from_utf8(chunks.join(&b' ')).unwrap();
    Ok(ciphertext)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    //unimplemented!("Decode {ciphertext} with the key ({a}, {b})");
    let (g, inv_a,_) = egcd(a,M);
    if g !=1{
        return Err(AffineCipherError::NotCoprime(a));
    }
    let bytes = ciphertext
        .bytes()
        .filter(|&c| c !=b' ')
        .map(|c| match c {
            b'A'..=b'Z' => b'a'+(((c-b'A') as i32 - b)* inv_a)
                    .rem_euclid(M) as u8,
            b'a'..=b'z' => b'a'+(((c-b'a') as i32 - b)* inv_a)
                    .rem_euclid(M) as u8,
            b => b,
        });
        let plaintext = String::from_utf8(bytes.collect()).unwrap();
        Ok(plaintext)
}

















