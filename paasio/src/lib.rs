use std::io::{Read, Result, Write};

//pub struct ReadStats<R>(::std::marker::PhantomData<R>);
pub struct ReadStats<R>{
    bytes:usize,
    reads:usize,
    wrapped:R,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        //unimplemented!()
        Self {
            bytes:0,
            reads:0,
            wrapped,
        }
    }

    pub fn get_ref(&self) -> &R {
        //unimplemented!()
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        //unimplemented!()
        self.bytes
    }

    pub fn reads(&self) -> usize {
        //unimplemented!()
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        //unimplemented!("Collect statistics about this call reading {:?}", buf)
        let bytes_read = self.wrapped.read(buf)?;
        self.bytes += bytes_read;
        self.reads += 1;
        Ok(bytes_read)
    }
}

//pub struct WriteStats<W>(::std::marker::PhantomData<W>);
pub struct WriteStats<W> {
    bytes: usize,
    writes: usize,
    wrapped:W,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        //unimplemented!()
        Self {
            bytes:0,
            writes: 0,
            wrapped,
        }
    }

    pub fn get_ref(&self) -> &W {
        //unimplemented!()
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        //unimplemented!()
        self.bytes
    }

    pub fn writes(&self) -> usize {
        //unimplemented!()
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        //unimplemented!("Collect statistics about this call writing {:?}", buf)
        let bytes_writed = self.wrapped.write(buf)?;
        self.bytes += bytes_writed;
        self.writes +=1;
        Ok(bytes_writed)
    }

    fn flush(&mut self) -> Result<()> {
        //unimplemented!()
        self.wrapped.flush()
    }
}
