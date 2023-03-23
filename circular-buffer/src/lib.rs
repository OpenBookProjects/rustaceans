pub struct CircularBuffer<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    //phantom: std::marker::PhantomData<T>,
    capacity:usize,
    //marker: usize,
    buffer: Vec<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {/* 
        unimplemented!(
            "Construct a new CircularBuffer with the capacity to hold {}.",
            match capacity {
                1 => "1 element".to_string(),
                _ => format!("{capacity} elements"),
            }
        ); */
        Self { 
            capacity, 
            //marker: 0, 
            buffer: Vec::with_capacity(capacity) }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        //unimplemented!("Write the passed element to the CircularBuffer or return FullBuffer error if CircularBuffer is full.");
        if self.buffer.len() == self.capacity {
            return Err(Error::FullBuffer);

        }
        
        self.buffer.push(_element);
        Ok(())

    }

    pub fn read(&mut self) -> Result<T, Error> {
        //unimplemented!("Read the oldest element from the CircularBuffer or return EmptyBuffer error if CircularBuffer is empty.");
        if self.buffer.len() == 0 {
            return Err(Error::EmptyBuffer);
        }
        let out = self.buffer[0].clone();
        self.buffer.remove(0);
        Ok(out)
    }

    pub fn clear(&mut self) {
        //unimplemented!("Clear the CircularBuffer.");
        if self.buffer.len() > 0 {
            self.buffer.remove(0);
        }
    }

    pub fn overwrite(&mut self, _element: T) {
        //unimplemented!("Write the passed element to the CircularBuffer, overwriting the existing elements if CircularBuffer is full.");
        self.buffer.push(_element);
        if self.buffer.len() > self.capacity {
            self.buffer.remove(0);
        }
    }
}


