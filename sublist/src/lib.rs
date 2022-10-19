#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    //unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    use Comparison::*;
    if _first_list.len() == _second_list.len() {
        //println!("try every elements equ?")
        for x in _second_list {
            if _first_list.contains(&x) {
                continue;
            } else {
                //println!("Unequal?");
                return Unequal;
            }
        }
        return Equal;
    } else {
        //println!("try who sub who?")
        if _first_list.len() > _second_list.len() {
            for x in _second_list {
                if _first_list.contains(&x) {
                    continue;
                } else {
                    return Unequal;
                }
            }
            //println!("v1 Superlist v2");
            return Superlist;
        } else {
            for x in _first_list {
                if _second_list.contains(&x) {
                    continue;
                } else {
                    return Unequal;
                }
            }
            //println!("v1 Sublist v2");
            return Sublist;
        }
    }
    //return Unequal;
}
