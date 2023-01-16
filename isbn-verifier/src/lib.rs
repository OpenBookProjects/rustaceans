/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    //unimplemented!("Is {:?} a valid ISBN number?", isbn);
    let (count, value) = isbn
        .chars()
        .filter(|&c| c !='-')
        .enumerate()
        .fold(Some((0,0)),|v,(i,c)| match (i,c){
            // normal code 
            // v?.0 中的 ? 是用来处理 Option 类型的。在这里 v 是一个Option<(usize, usize)> 类型的变量, 如果 ? 在Option 值上使用，则只有在值为Some时才会继续执行代码，否则它会返回None并停止当前函数的执行。所以这里如果 v 是 None，那么整个fold函数将会返回 None， 否则就可以访问元组的 0 和 1 两个元素。
            // 如果没有 ? 则无法通过编译,因为如果有意外值时,将无法确认如何执行
            (_,'0'..='9') => Some((v?.0+1,v?.1+(10-i)*(c.to_digit(10)? as usize))),
            // last X as 10
            (9, 'X') => Some((v?.0+1,v?.1+(10-i)*10)),
            _ => None,
        })
        .unwrap_or((0,0));
    
    count == 10 && value%11==0
}


