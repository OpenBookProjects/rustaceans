/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    //unimplemented!()
    return vec![];
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    //unimplemented!("create a zeroized buffer of {} bytes", count)
    return vec![0; count];
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    //unimplemented!()
    let mut buffer = create_buffer(5);
    for i in 0..5 {
        if i < 2 {
            buffer[i] = 1;
        } else {
            buffer[i] = buffer[i - 1] + buffer[i - 2];
        }
    }
    return buffer;
}
