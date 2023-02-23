use std::ops::Add;
//use std::ops::PartialOrd;
//use std::ops::Copy;

pub struct Triangle<T: Add<Output=T>+ PartialOrd + Copy>{
    sides:[T;3],
}

impl<T: Add<Output=T>+ PartialOrd + Copy> Triangle<T> {
    //pub fn build(sides: [u64; 3]) -> Option<Triangle> {
    pub fn build(sides: [T; 3]) -> Option<Self> {
        //unimplemented!("Construct new Triangle from following sides: {sides:?}. Return None if the sides are invalid.");
        let mut s3 = sides.clone();
        s3.sort_unstable_by(|a,b| a.partial_cmp(b).expect("Partial Compare Error."));
        println!("Address of sides: {:p}", std::ptr::addr_of!(sides));
        println!("Address of s3: {:p}", std::ptr::addr_of!(s3));
        (s3[0]+s3[1] > s3[2]).then_some(Self {sides: s3})
    }

    pub fn is_equilateral(&self) -> bool {
        //unimplemented!("Determine if the Triangle is equilateral.");
        self.sides[0]==self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        //unimplemented!("Determine if the Triangle is scalene.");
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        //unimplemented!("Determine if the Triangle is isosceles.");
        self.sides[0]==self.sides[1] || self.sides[1]==self.sides[2]
    }
}

