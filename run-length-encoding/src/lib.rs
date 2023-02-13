pub fn encode(source: &str) -> String {
    //unimplemented!("Return the run-length encoding of {source}.");
    source
        .chars()
        .fold(Vec::new(),|mut v,curr| match v.pop() {
            None => Vec::from([(1,curr)]),
            Some((count, prev)) if prev == curr => [v, Vec::from([(count+1,curr)])].concat(),
            Some((count, prev)) => [v, Vec::from([(count,prev),(1,curr)])].concat(),
        })
        .iter()
        .map(|(count,c)|match count{
            0..=1 => c.to_string(),
            _=> count.to_string()+&c.to_string(),
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    //unimplemented!("Return the run-length decoding of {source}.");
    source
        .chars()
        .fold(("".to_string(),0),|(res,len),c|{
            match(c.is_ascii_digit(),len){
                (true,_)=>(res,len*10+c.to_string().parse::<usize>().unwrap()),
                (_,0)=> (res+&c.to_string(),0),
                (_,count)=>(res+&c.to_string().repeat(count),0),
            }
        })
        .0
}
