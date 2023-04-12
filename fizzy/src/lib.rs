use std::ops::Rem;
use std::fmt::Display;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
//pub struct Matcher<T>(std::marker::PhantomData<T>);
pub struct Matcher<T>(fn(T)->bool, &'static str);

impl<T:Copy> Matcher<T> {
    //pub fn new<F, S>(_matcher: F, _subs: S) -> Matcher<T> {
    pub fn new(_matcher: fn(T)->bool, _subs: &'static str) -> Matcher<T> {
        //unimplemented!()
        Matcher(_matcher, _subs)
    }
}


/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
//pub struct Fizzy<T>(std::marker::PhantomData<T>);
pub struct Fizzy<T>(Vec<Matcher<T>>);

impl<T: Display+Copy> Fizzy<T> {
    pub fn new() -> Self {
        //unimplemented!()
        Self(Vec::<Matcher<T>>::new())
    }

    // feel free to change the signature to `mut self` if you like
    //#[must_use]
    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        //unimplemented!()
        self.0.push(_matcher);
        Self(self.0)
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item = T>>(self, _iter: I) -> impl Iterator<Item = String> {
        // unimplemented!() doesn't actually work, here; () is not an Iterator
        // that said, this is probably not the actual implementation you desire
        //Vec::new().into_iter()
        FizzyResultIter(self, _iter)
    }
}

struct FizzyResultIter<T: Display+Copy, I: Iterator<Item = T>>(Fizzy<T>, I);

impl<T, I> Iterator for FizzyResultIter<T, I>
where T:Display+Copy, I:Iterator<Item=T> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let Self(fizzy, iter) = self;
        let Fizzy(matchers) = fizzy;
        let n = iter.next()?;
        let mut output = matchers
            .iter()
            .filter_map(|m| if m.0(n) {
                Some(m.1.to_string())
                } else {
                    None
                })
            .collect::<Vec<_>>()
            .join("");
        if output.is_empty() {
            output = n.to_string();
        }
        Some(output)
    }
}


/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where T:Display+Copy+Rem, <T as Rem>::Output:PartialEq<T>, u8:Into<T> {
    //unimplemented!()
    Fizzy::new()
        .add_matcher(Matcher::new(|n| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n| n % 5.into() == 0.into(), "buzz"))
}
