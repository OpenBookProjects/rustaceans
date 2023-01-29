/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
use std::cmp::{min,max};
use itertools::iproduct; // power tools must usage

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd,Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        /* unimplemented!(
            "if the value {} is a palindrome return Some, otherwise return None",
            value
        ); */
        let str = format!("{}",value);
        if str.chars().eq(str.chars().rev()){
            Some(Palindrome(value))
        }else{
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        //unimplemented!("return inner value of a Palindrome");
        self.0
    }
}

// 通用展开函式,完成自动化探索
fn extent<T>(range:Option<(T,T)>, x:T)-> Option<(T,T)> where T: Ord+Copy{
    match range{
        None => Some((x,x)),
        Some((a,b)) => Some((min(a,x),max(b,x)))
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    /* unimplemented!(
        "returns the minimum and maximum number of palindromes of the products of two factors in the range {} to {}",
        min, max
    ); */
    iproduct!(min..=max,min..=max)
        .flat_map(|(a,b)| Palindrome::new(a*b))
        .fold(None,extent)
}
