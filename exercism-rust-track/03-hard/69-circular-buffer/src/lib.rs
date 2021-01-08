fn modulus(x: i32, m: usize) -> usize {
    let m_i32 = m as i32;

    (((x % m_i32) + m_i32) % m_i32) as usize
}

pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    empty_pointer: usize,
    count: usize,
    capacity: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            data.push(None);
        }

        Self {
            data,
            empty_pointer: 0,
            count: 0,
            capacity,
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.count == self.capacity {
            Err(Error::FullBuffer)
        } else {
            self.overwrite(_element);

            Ok(())  
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.count == 0 {
            Err(Error::EmptyBuffer)
        } else {
            let index = modulus(self.empty_pointer as i32 - self.count as i32, self.capacity);

            self.count -= 1;

            Ok(self.data.get_mut(index).unwrap().take().unwrap())
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.capacity {
            self.data[i].take();
        }

        self.empty_pointer = 0;
        self.count = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        self.data[self.empty_pointer].replace(_element);

        self.empty_pointer = (self.empty_pointer + 1) % self.capacity;

        self.count = std::cmp::min(self.count + 1, self.capacity);
    }
}
