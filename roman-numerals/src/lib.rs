use std::fmt::{Display, Formatter, Result};

pub struct Roman(usize);

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        //unimplemented!("Return a roman-numeral string representation of the Roman object");
        let converted: String = [
            (1000,["","M","MM","MMM","","","","","",""]),
            (100,["","C","CC","CCC","CD","D","DC","DCC","DCCC","CM"]),
            (10,["","X","XX","XXX","XL","L","LX","LXX","LXXX","XC"]),
            (1,["","I","II","III","IV","V","VI","VII","VIII","IX"])
        ]
        .iter()
        .map(|(base,num)| num[self.0/base%10])
        .collect();
        write!(_f, "{}", converted)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        //unimplemented!("Construct a Roman object from the '{num}' number");
        Self(num as usize)
    }
}
