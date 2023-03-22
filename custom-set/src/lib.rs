use std::collections::HashSet;
use std::hash::Hash;


#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T: PartialEq+Eq+Hash> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    //phantom: std::marker::PhantomData<T>,
    data: HashSet<T>,
}

impl<T> CustomSet<T> 
where T: 
    PartialEq+Eq+Hash+Clone,
{
    pub fn new(_input: &[T]) -> Self {
        //unimplemented!();
        Self { data: HashSet::from_iter(_input.iter().cloned()) }
    }

    pub fn contains(&self, _element: &T) -> bool {
        //unimplemented!();
        self.data.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        //unimplemented!();
        self.data.insert(_element);
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        //unimplemented!();
        self.data.is_subset(&_other.data)
    }

    pub fn is_empty(&self) -> bool {
        //unimplemented!();
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        //unimplemented!();
        self.data.is_disjoint(&_other.data)
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        //unimplemented!();
        Self { data: self.data.intersection(&_other.data).cloned().collect() }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        //unimplemented!();
        Self { data: self.data.difference(&_other.data).cloned().collect() }
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        //unimplemented!();
        Self { data: self.data.union(&_other.data).cloned().collect() }
    }
}
