pub const SIZE: usize = 1024;

pub struct Ringbuffer<T> {
    read: usize,
    filled: usize,
    buffer: [T; SIZE],
}

impl<T: Copy> Ringbuffer<T> {
    pub const fn new(buffer: [T; SIZE]) -> Self {
        Ringbuffer::<T> {
            read: 0,
            filled: 0,
            buffer: buffer,
        }
    }

    pub fn capacity(&self)     -> usize { self.buffer.len() }
    pub fn filled_slots(&self) -> usize { self.filled }
    pub fn free_slots(&self)   -> usize { self.capacity() - self.filled_slots() }

    fn masked(&self, index: usize) -> usize { index & (self.capacity() - 1) }

    pub fn push(&mut self, value: T) {
        self.buffer[self.masked(self.read + self.filled)] = value;
        self.filled += 1;
    }

    pub fn pop(&mut self) -> T {
        let result = self.peek();
        self.read += 1;
        self.filled -= 1;
        return result;
    }

    pub fn peek_absolute(&self, index: usize) -> T {
        return self.buffer[self.masked(index)];
    }

    pub fn peek_relative(&self, offset: usize) -> T {
        return self.peek_absolute(self.read + offset);
    }

    pub fn peek(&self) -> T {
        return self.peek_relative(0);
    }

    fn copy<A: Copy>(to: &mut[A], from: &[A]) {
        for i in 0..from.len() {
            to[i] = from[i];
        }
    }

    pub fn push_multiple(&mut self, values: &[T]) {
        let len = values.len();
        let write = self.masked(self.read + self.filled);
        if write + len <= self.capacity() {
            Self::copy(&mut self.buffer[write..write + len], values);
        } else {
            let len1 = self.capacity() - write;
            let len2 = len - len1;
            Self::copy(&mut self.buffer[write..write + len1], &values[0..len1]);
            Self::copy(&mut self.buffer[0..len2], &values[len1..len2]);
        }
        self.filled += len;
    }

    pub fn pop_multiple(&mut self, values: &mut [T]) {
        self.peek_multiple(values);
        let len = values.len();
        self.read += len;
        self.filled -= len;
    }

    pub fn peek_multiple_absolute(&self, index: usize, values: &mut [T]) {
        let len = values.len();
        let read = self.masked(index);
        if read + len <= self.capacity() {
            Self::copy(values, &self.buffer[read..read + len]);
        } else {
            let len1 = self.capacity() - read;
            let len2 = len - len1;
            Self::copy(&mut values[0..len1], &self.buffer[read..read + len1]);
            Self::copy(&mut values[len1..len2], &self.buffer[0..len2]);
        }
    }

    pub fn peek_multiple_relative(&self, offset: usize, values: &mut [T]) {
        self.peek_multiple_absolute(self.read + offset, values);
    }

    pub fn peek_multiple(&self, values: &mut [T]) {
        self.peek_multiple_relative(0, values);
    }
}
