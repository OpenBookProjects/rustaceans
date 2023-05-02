const PLANTS:[(char, &str); 4] = [
    ('G', "grass"), 
    ('C', "clover"), 
    ('R', "radishes"), 
    ('V', "violets")];

pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    //unimplemented!("Solve kindergarten-garden exercise");
    let idx:usize = _student
            .chars()
            .next()
            .unwrap() as usize - 'A' as usize;
    _diagram
        .split('\n')
        .flat_map(|row| row
            .chars()
            .skip(idx*2)
            .take(2)
            .map(char_to_plant))
        .collect()
}

fn char_to_plant(c: char) -> &'static str {
    (PLANTS
        .iter()
        .find(|p| p.0 == c)
        .unwrap())
        .1
}