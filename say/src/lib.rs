pub fn encode(n: u64) -> String {
    //unimplemented!("Say {n} in English.");
    if n<20{
        let one2tens = vec!["zero", "one", "two", "three", "four", 
                            "five", "six", "seven", "eight", "nine",
                            "ten", "eleven", "twelve", "thirteen", "fourteen", 
                            "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
                        ];
        String::from(one2tens[n as usize])
    }else if n < 100{
        let tens = vec!["twenty", "thirty", "forty", "fifty", 
                        "sixty", "seventy", "eighty", "ninety",
                        ];
        let mut encoded_num = String::from(tens[(n/10-2) as usize]);
        if n % 10 != 0{
            encoded_num.push('-');
            encoded_num.push_str(encode(n%10).as_str());
        }
        encoded_num
    }else if n < 1000{
        let mut encoded_num = String::from(encode(n/100).as_str());
        encoded_num.push_str(" hundred");
        if n % 100 != 0{
            encoded_num.push(' ');
            encoded_num.push_str(encode(n%100).as_str());
        }
        encoded_num
    }else {
        let large_num = vec!["thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];
        let num_size = (n as f32).log(1000.0).trunc() as u32;
        if num_size as usize > large_num.len() {
            panic!("Can NOT parse number that too large!");
        }
        let mut encoded_num=String::from(encode(n/(1000_u64.pow(num_size))).as_str());
        encoded_num.push_str(" ");
        encoded_num.push_str(large_num[num_size as usize-1]);
        if n%(1000_u64.pow(num_size))!=0{
            encoded_num.push(' ');
            encoded_num.push_str(encode(n%(1000_u64.pow(num_size))).as_str());
        }
        encoded_num
    }

}
