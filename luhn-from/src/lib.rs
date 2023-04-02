pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        //unimplemented!("Determine if the current Luhn struct contains a valid credit card number.");
        self.0.chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .try_fold((0u32, 0u32), |(cnt, sum), c| {
                c.to_digit(10)
                    .map(|d| d* if cnt &1 == 1{2} else{1})
                    .map(|d| if d > 9 {d-9} else {d})
                    .map(|d| (cnt+1, sum+d))
            })
            .map_or(false, |(count, sum)| count > 1 && sum % 10 == 0)
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
//impl<'a> From<&'a str> for Luhn {
impl<T: ToString> From<T> for Luhn {
    //fn from(input: &'a str) -> Self {
    fn from(input: T) -> Self {
        //unimplemented!("From the given input '{input}' create a new Luhn struct.");
        Self(input.to_string())
    }
}
