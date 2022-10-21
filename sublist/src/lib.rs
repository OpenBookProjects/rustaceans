#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_sublist<T: PartialEq>(first: &[T], second: &[T]) -> bool {
    let flen = first.len();
    flen == 0 || second.windows(flen).any(|slice| first == slice)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    //unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    use Comparison::*;

    if _first_list == _second_list {
        return Equal;
    } else if is_sublist(_first_list, _second_list) {
        Comparison::Sublist
    } else if is_sublist(_second_list, _first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
