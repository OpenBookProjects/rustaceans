extern crate rand;
use rand::prelude::*;

/*
Modular exponentiation - Wikipedia
https://en.wikipedia.org/wiki/Modular_exponentiation
Pseudocode
function modular_pow(base, exponent, modulus) is
    if modulus = 1 then
        return 0
    Assert :: (modulus - 1) * (modulus - 1) does not overflow base
    result := 1
    base := base mod modulus
    while exponent > 0 do
        if (exponent mod 2 == 1) then
            result := (result * base) mod modulus
        exponent := exponent >> 1
        base := (base * base) mod modulus
    return result
*/
fn modular_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    base = base % modulus;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus
        }
        exponent >>= 1;
        base = base.pow(2) % modulus;
    }
    result
}

pub fn private_key(p: u64) -> u64 {
    //unimplemented!("Pick a private key greater than 1 and less than {}", p)
    thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    /* unimplemented!(
        "Calculate public key using prime numbers {} and {}, and private key {}",
        p,
        g,
        a
    ) */
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    /* unimplemented!(
        "Calculate secret key using prime number {}, public key {}, and private key {}",
        p,
        b_pub,
        a
    ) */
    modular_pow(b_pub, a, p)
}
