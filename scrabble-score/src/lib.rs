const SCOREMAP:[u64;26] = [
                        1,  //a
                        3,  //b
                        3,  //c
                        2,  //d
                        1,  //e
                        4,  //f
                        2,  //g
                        4,  //h
                        1,  //i
                        8,  //j
                        5,  //k
                        1,  //l
                        3,  //m
                        1,  //n
                        1,  //o
                        3,  //p
                        10, //q
                        1,  //r
                        1,  //s
                        1,  //t
                        1,  //u
                        4,  //v
                        4,  //w
                        8,  //x
                        4,  //y
                        10  //z
                        ];

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    //unimplemented!("Score {word} in Scrabble.");
    word
        .to_lowercase()
        .chars()
        .filter(|x| 'a'<=*x&&*x<='z')
        .map(|c| SCOREMAP[(c as u8 - b'a') as usize])
        .sum()
}
