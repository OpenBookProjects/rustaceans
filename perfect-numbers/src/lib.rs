#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    //unimplemented!("classify {}", num);
    match (1..num).filter(|n| num%n==0).sum::<u64>(){
        n if n < num || num==1 => Some(Classification::Deficient),
        0 => None,
        n if n == num => Some(Classification::Perfect),
        n if n > num => Some(Classification::Abundant),
        _ => None,
    }
}
