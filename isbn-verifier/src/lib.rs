/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    //unimplemented!("Is {:?} a valid ISBN number?", isbn);
    let (count, value) = isbn
        .chars()
        .filter(|&c| c !='-')
        .enumerate()
        .fold(Some((0,0)),|v,(i,c)| match (i,c){
            // normal code
            (_,'0'..='9') => Some((v?.0+1,v?.1+(10-i)*(c.to_digit(10)? as usize))),
            // last X as 10
            (9, 'X') => Some((v?.0+1,v?.1+(10-i)*10)),
            _ => None,
        })
        .unwrap_or((0,0));
    
    count == 10 && value%11==0
}


