pub fn find(array: &[i32], key: i32) -> Option<usize> {
    /* unimplemented!(
        "Using the binary search algorithm, find the element '{}' in the array '{:?}' and return its index.",
        key,
        array
    ); */

    let mut l = 0;
    let mut r = array.len();

    while l < r{
        let guess = (l+r)/2;

        if key < array[guess]{
            r = guess
        }else if key>array[guess]{
            l = guess+1
        }else{ //key==array[guess]
            return Some(guess)
        }
    }

    None

}
