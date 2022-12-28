// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[allow(clippy::new_without_default)]
pub struct School {
    pupil: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        //unimplemented!()
        Self {
            pupil:BTreeMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        //unimplemented!("Add {} to the roster for {}", student, grade)
        self.pupil
            .entry(grade)
            .or_insert(BTreeSet::new())
            .insert(String::from(student))
            ;
    }

    pub fn grades(&self) -> Vec<u32> {
        //unimplemented!()
        self.pupil
            .keys() 
            .cloned() //<--
            .collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        //unimplemented!("Return the list of students in {}", grade)
        match self.pupil.get(&grade){
            Some(x) => {
                x.iter()
                .cloned() //<--
                .collect()
                },
            None => {Vec::new()}
        }
    }
}
