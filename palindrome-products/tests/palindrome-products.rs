//! This test suite was generated by the rust exercise tool, which can be found at
//! https://github.com/exercism/rust/tree/main/util/exercise

use palindrome_products::{palindrome_products, Palindrome};

/// Process a single test case for the property `smallest`
///
/// All cases for the `smallest` property are implemented
/// in terms of this function.
fn process_smallest_case((from, to): (u64, u64), expected: Option<u64>) {
    let min = palindrome_products(from, to).map(|(min, _)| min);
    assert_eq!(min.map(|newtype| newtype.into_inner()), expected);
}

/// Process a single test case for the property `largest`
///
/// All cases for the `largest` property are implemented
/// in terms of this function.
fn process_largest_case((from, to): (u64, u64), expected: Option<u64>) {
    let max = palindrome_products(from, to).map(|(_, max)| max);
    assert_eq!(max.map(|newtype| newtype.into_inner()), expected);
}


#[test]
/// test `Palindrome::new` with valid input
fn test_palindrome_new_return_some() {
    for v in [1, 11, 121, 12321, 1234321, 123454321, 543212345] {
        assert_eq!(Palindrome::new(v).expect("is a palindrome").into_inner(), v);
    }
}

#[test]
//#[ignore]
/// test `Palindrome::new` with invalid input
fn test_palindrome_new_return_none() {
    for v in [12, 2322, 23443, 1233211, 8932343] {
        assert_eq!(Palindrome::new(v), None);
    }
}

#[test]
//#[ignore]
/// finds the smallest palindrome from single digit factors
fn test_finds_the_smallest_palindrome_from_single_digit_factors() {
    process_smallest_case((1, 9), Some(1));
}

#[test]
//#[ignore]
/// finds the largest palindrome from single digit factors
fn test_finds_the_largest_palindrome_from_single_digit_factors() {
    process_largest_case((1, 9), Some(9));
}

#[test]
//#[ignore]
/// find the smallest palindrome from double digit factors
fn test_find_the_smallest_palindrome_from_double_digit_factors() {
    process_smallest_case((10, 99), Some(121));
}

#[test]
//#[ignore]
/// find the largest palindrome from double digit factors
fn test_find_the_largest_palindrome_from_double_digit_factors() {
    process_largest_case((10, 99), Some(9009));
}

#[test]
//#[ignore]
/// find smallest palindrome from triple digit factors
fn test_find_smallest_palindrome_from_triple_digit_factors() {
    process_smallest_case((100, 999), Some(10201));
}

#[test]
//#[ignore]
/// find the largest palindrome from triple digit factors
fn test_find_the_largest_palindrome_from_triple_digit_factors() {
    process_largest_case((100, 999), Some(906609));
}

#[test]
//#[ignore]
/// find smallest palindrome from four digit factors
fn test_find_smallest_palindrome_from_four_digit_factors() {
    process_smallest_case((1000, 9999), Some(1002001));
}

#[test]
//#[ignore]
/// find the largest palindrome from four digit factors
fn test_find_the_largest_palindrome_from_four_digit_factors() {
    process_largest_case((1000, 9999), Some(99000099));
}

#[test]
//#[ignore]
/// empty result for smallest if no palindrome in the range
fn test_empty_result_for_smallest_if_no_palindrome_in_the_range() {
    process_smallest_case((1002, 1003), None);
}

#[test]
//#[ignore]
/// empty result for largest if no palindrome in the range
fn test_empty_result_for_largest_if_no_palindrome_in_the_range() {
    process_largest_case((15, 15), None);
}

#[test]
//#[ignore]
/// error result for smallest if min is more than max
fn test_error_result_for_smallest_if_min_is_more_than_max() {
    process_smallest_case((10000, 1), None);
}

#[test]
//#[ignore]
/// error result for largest if min is more than max
fn test_error_result_for_largest_if_min_is_more_than_max() {
    process_largest_case((2, 1), None);
}
