use num_bigint::BigInt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::str::FromStr;

const FP_DEC_SHIFT: u32 = 50;
/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Decimal(BigInt);

impl Add for Decimal{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Decimal(self.0 + other.0)
    }
}

impl Sub for Decimal{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Decimal(self.0 - other.0)
    }
}

impl Mul for Decimal{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let v = self.0 * other.0/BigInt::from(10).pow(FP_DEC_SHIFT);Decimal(v)
    }
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        //unimplemented!("Create a new decimal with a value of {input}")
        let is_negetive = input.starts_with('-');

        let mut it = input[(is_negetive as usize)..].split('.');
        let (int_part, dec_part) = (it.next().unwrap(), it.next().unwrap_or("0"));

        let mut int = BigInt::from_str(int_part).ok()?;
        int *= BigInt::from(10).pow(FP_DEC_SHIFT);

        let mut dec = BigInt::from_str(dec_part).ok()?;
        let dec_shift = FP_DEC_SHIFT - dec_part.len() as u32;
        dec *= BigInt::from(10).pow(dec_shift);

        let v = if is_negetive{-int - dec} else {int + dec};
        Some(Decimal(v))
    }
}
