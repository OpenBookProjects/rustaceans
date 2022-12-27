use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    //unimplemented!("How will you transform the tree {:?}?", h)
    let mut res = BTreeMap::<char,i32>::new();

    h.iter().for_each(|(&k,v)|{
        v.iter()
            .for_each(|s|{
                res.insert(s.to_ascii_lowercase(),k);
            });
    });
    res
}
