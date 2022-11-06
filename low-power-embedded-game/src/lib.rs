// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    //unimplemented!("implement `fn divmod`");
    //let result: (i16, i16) = (dividend / divisor, dividend % divisor);
    //result
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    /* unimplemented!("implement `fn evens`");
        // TODO: remove this; it's only necessary to allow this function to compile
        // before the student has done any work.
        std::iter::empty()
        StepBy in std::iter - Rust
    https://doc.rust-lang.org/std/iter/struct.StepBy.html
        */
    iter.step_by(2)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        /*曼哈顿距离 - 维基百科，自由的百科全书 https://zh.wikipedia.org/wiki/%E6%9B%BC%E5%93%88%E9%A0%93%E8%B7%9D%E9%9B%A2
        d(A,B)=∣6−2∣+∣5−2∣=4+3=7
        */
        //unimplemented!("implement `fn manhattan`")
        //println!("self.0:{}\nself.1:{}", self.0, self.1);

        self.0.abs() + self.1.abs()
    }
}
