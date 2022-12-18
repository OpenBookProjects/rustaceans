use std::collections::HashMap;
use itertools::Itertools;


const NUMS: [char;10]=['0',
                    '1',
                    '2',
                    '3',
                    '4',
                    '5',
                    '6',
                    '7',
                    '8',
                    '9',
];

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    //unimplemented!("Solve the alphametic {:?}", input)
    let subject = input.split("==").collect::<Vec<_>>();

    let left = subject[0].split('+').map(|x| x.trim()).collect::<Vec<_>>();
    let right = subject[1].trim();

    let mut asciis = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_uppercase()).collect::<Vec<_>>();

    asciis.sort();
    asciis.dedup();

    if left.iter().any(|lc| lc.len() > right.len()) || asciis.len()>10{
        return None;
    }

    let perms = NUMS.iter().permutations(asciis.len());
    
    for perm in perms{
        let dic: HashMap<_,_> = asciis.iter().zip(perm).collect();

        if left.iter().any(|lw| dic[&lw.chars().next().unwrap()]==&'0') 
            || dic[&right.chars().next().unwrap()]==&'0'{
            continue;
        };

        let left:i64 = left
            .iter()
            .map(|lw| lw.chars().map(|cc| dic[&cc])
            .join("")
            .parse::<i64>()
            .unwrap())
            .sum();
        let right = right
            .chars()
            .map(|rc| dic[&rc])
            .join("")
            .parse::<i64>()
            .unwrap();

        if left == right{
            let result = dic.iter().map(|(k,v)|(**k,v.to_digit(10).unwrap() as u8)).collect::<HashMap<_,_>>();
            return Some(result);
        }
    }
    None

}

