#[derive(Debug, Clone)]
pub struct Stream<T> {
    data: Vec<T>,
    position: usize,
}

impl<T> Stream<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data, position: 0 }
    }

    pub fn read_exact(&mut self, len: usize) -> &[T] {
        if (self.position + len) > self.data.len() {
            panic!("Not enough elements in Stream");
        }
        let out_slice = self.data[self.position..self.position + len].as_ref();
        self.position += len;

        out_slice
    }

    /// Read all remaining elements
    pub fn read_all(&self) -> &[T] {
        let length = self.data.len() - self.position;
        &self.data[self.position..self.position + length]
    }

    /// Drain the stream and return the underlying data (including data already read)
    pub fn drain(self) -> Vec<T> {
        self.data
    }

    pub fn write_slice(&mut self, slice: &[T])
    where
        T: Clone,
    {
        self.data.extend_from_slice(slice);
    }
}
