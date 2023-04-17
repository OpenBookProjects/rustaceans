/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    // This field is just to suppress compiler complaints;
    // feel free to delete it at any point.
    //_phantom: std::marker::PhantomData<&'a u8>,
    key: &'a [u8],
    i: usize,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    //pub fn new<Key>(key: &Key) -> Xorcism<'a> {
    pub fn new<Key:AsRef<[u8]>+'a+?Sized>(key: &'a Key) -> Xorcism<'a> {
        //unimplemented!()
        let key = key.as_ref();
        Xorcism {
            key:key,
            i: 0,
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        //unimplemented!()
        for d in data{
            *d ^= self.key[self.i];
            self.i = (self.i+1)%self.key.len();
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    //pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8> {
    pub fn munge<'b>(
        &'b mut self, 
        data: impl IntoIterator<Item = impl std::borrow::Borrow<u8>>+'b,
    ) -> impl Iterator<Item = u8>+'b {
        //unimplemented!();
        // this empty iterator silences a compiler complaint that
        // () doesn't implement ExactSizeIterator
        //std::iter::empty()
        data
            .into_iter()
            .map(|x| x.borrow().clone())
            .map(|mut d| {
                d ^= self.key[self.i];
                self.i = (self.i+1)%self.key.len();
                d
            })
    }
}
