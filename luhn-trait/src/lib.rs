pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
/* impl<'a> Luhn for &'a str {
    fn valid_luhn(&self) -> bool {
        unimplemented!("Determine if '{self}' is a valid credit card number.");
    }
}
 */

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        self.to_string()
            .chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .try_fold((0u32, 0u32), |(cnt, sum), c| {
                c.to_digit(10)
                    .map(|d| d*if cnt &1 == 1 {2} else {1})
                    .map(|d| if d>9{d-9} else{d})
                    .map(|d| (cnt+1, sum+d))
                })
            .map_or(false,|(cnt, sum)| cnt>1 && sum%10==0)
    }
}