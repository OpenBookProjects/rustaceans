/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    //unimplemented!("Out of {:?}, which hand wins?", hands)
    let mut hold: BinaryHeap<(PokerHand, &str)> =
        hands.iter().map(|&s| (PokerHand::parse(s), s)).collect();

    let (winning, s) = hold.pop().unwrap();

    let mut results = vec![s];
    // while Option
    while let Some((value, s)) = hold.pop() {
        if value < winning {
            break;
        }
        results.push(s);
    }
    results
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PokerHand {
    counts: Vec<u8>,
    values: Vec<u8>,
}

impl PokerHand {
    fn parse(s: &str) -> Self {
        let (values, suits): (Vec<u8>, Vec<u8>) = s
            .split_whitespace()
            .map(|s| -> (u8, u8) {
                {
                    let (value, suit) = s.split_at(s.len() - 1);
                    (
                        match value.parse::<u8>() {
                            Ok(v) => v,
                            Err(_) => match value {
                                "J" => 11,
                                "Q" => 12,
                                "K" => 13,
                                _ => 14, //"A"
                            },
                        },
                        suit.as_bytes()[0],
                    )
                }
            })
            .unzip();

        let mut value_count_map: HashMap<u8, u8> = HashMap::new();

        for &value in values.iter() {
            *value_count_map.entry(value).or_default() += 1;
        }

        let mut count_value_vec: Vec<(u8, u8)> =
            value_count_map.into_iter().map(|(v, c)| (c, v)).collect();

        count_value_vec.sort_by_cached_key(|&x| Reverse(x));

        let (mut counts, mut values): (Vec<u8>, Vec<u8>) = count_value_vec.iter().copied().unzip();

        if counts.len() == 5 {
            // A as 1
            if values == [14, 5, 4, 3, 2] {
                values = vec![5, 4, 3, 2, 1];
            }
            // 无对时 NILL pares
            let straight = values[0] - values[4] == 4;
            let flush = suits[1..].iter().all(|&x| x == suits[0]);
            counts = match (straight, flush) {
                // Straight flush > Four <- kind([4, 1])
                (true, true) => vec![5],
                // Full House([3, 2]) > Flush > straight > Three <- ([3, 1, 1])
                (false, true) => vec![3, 1, 3],
                (true, false) => vec![3, 1, 2],
                (false, false) => counts,
            }
        }
        Self { counts, values }
    }
}
