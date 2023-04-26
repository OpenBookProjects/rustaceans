pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    //unimplemented!("From the given input '{input:?}' construct a proper dominoes chain or return None if it is not possible.");
    dom(&mut input.iter().cloned().collect(), 0)
}

fn dom(dominoes:&mut Vec<(u8,u8)>, pos:usize) -> Option<Vec<(u8,u8)>> {
    if pos==dominoes.len() {
        return if dominoes
            .first()
            .map(|d| d.0)==dominoes
                .last()
                .map(|d| d.1) {
            Some(dominoes.clone())
        } else {
            None
        };
    }
    for i in pos..dominoes.len() {
        for _ in 0..2{
            dominoes[i] = (dominoes[i].1, dominoes[i].0);
            if pos==0 || dominoes[pos-1].1==dominoes[i].0 {
                dominoes.swap(pos, i);
                let res = dom(dominoes, pos+1);
                dominoes.swap(pos, i);
                if res.is_some() {
                    return res;
                }
            }
        }
    }
    None
}
