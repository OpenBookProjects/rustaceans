pub fn get_diamond(c: char) -> Vec<String> {
    let n = c as u8 - 'A' as u8 ;
    (0..=n).chain((0..n).rev()).map(|i| {
        let mut s = String::from(" ").repeat(2*(n as usize)+1);
        let s_bytes = unsafe { s.as_mut_vec() };
        let idx1 = n as usize - i as usize;
        let idx2 = n as usize + i as usize;
        s_bytes[idx1] = b'A' + i;
        s_bytes[idx2] = b'A' + i;
        s
    }).collect()
}

/* pub fn get_diamond(c: char) -> Vec<String> {
    let n = c as u8 - 'A' as u8 ;
    (0..=n).chain((0..n).rev()).map(|i| {
        let mut s = String::from(" ").repeat(2*(n as usize)+1);
        //let mut s = String::new();
        unsafe {
            s.as_bytes_mut()[(n - i) as usize] = 'A' as u8 + i;
            s.as_bytes_mut()[(n + i) as usize] = 'A' as u8 + i;
        }
        s
    }).collect()
}
pub fn get_diamond(c: char) -> Vec<String> {
    let n = c as u8 - 'A' as u8;
    (0..=n).chain((0..n).rev()).map(|i| {
        let mut s = String::from(" ").repeat(2*(n as usize)+1);
        let ch = char::from_u32(i as u32 + 'A' as u32).unwrap();
        let idx = n as usize - i as usize;
        s.insert(idx, ch);
        s.insert(n as usize + i as usize + 1, ch);
        s
    }).collect()
}

*/

