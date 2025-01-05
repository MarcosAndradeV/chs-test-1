pub mod stack;
use std::{alloc::{alloc_zeroed, dealloc, realloc, Layout, LayoutError}, slice, usize};

#[derive(Debug)]
pub struct Memory {
    inner: *mut u8,
    // layout: Layout,
    size: usize,
    head: usize,
}

const MIN_SIZE: usize = 16;

impl Default for Memory {
    fn default() -> Self {
        Self::new(MIN_SIZE).unwrap()
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        let layout = Layout::array::<u8>(self.size).unwrap();
        unsafe { dealloc(self.inner, layout) };
    }
}

impl Memory {
    pub fn new(size: usize) -> Result<Self, LayoutError> {
        unsafe {
            let layout = Layout::array::<u8>(size)?;
            let inner = alloc_zeroed(layout);
            Ok(Self {
                inner,
                size,
                // layout,
                head: 0,
            })
        }
    }
    unsafe fn wirte_unchecked<T>(&self, val: T) {
        (self.inner.add(self.head) as *mut T).write(val);
    }
    pub fn wirte<T>(&self, val: T) -> Result<(), String> {
        if self.head + size_of::<T>() > self.size {
            return Err("Out Of Memory".to_string());
        }
        unsafe {
            self.wirte_unchecked(val);
        }
        Ok(())
    }
    pub fn read<T>(&self) -> Result<T, String> {
        if self.head + size_of::<T>() > self.size {
            return Err("Out Of Bounds".to_string());
        }
        let val = unsafe { self.read_unchecked::<T>() };
        Ok(val)
    }
    unsafe fn read_unchecked<T>(&self) -> T {
        (self.inner.add(self.head) as *mut T).read()
    }
    pub fn move_left(&mut self, n: usize) {
        if self.head + n < self.size {
            self.head += n;
        }
    }
    pub fn move_rigth(&mut self, n: usize) {
        self.head = self.head.saturating_sub(n);
    }
    pub fn as_slice<T>(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.inner as * const T, self.size) }
    }
    pub fn as_slice_mut<T>(&self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.inner as * mut T, self.size) }
    }
    pub fn realloc(&mut self, new_size: usize) -> Result<(), LayoutError> {
        unsafe {
            let layout = Layout::array::<u8>(new_size)?;
            self.size = new_size;
            self.inner = realloc(self.inner, layout, new_size);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_1() {
        assert!(Memory::default().size == MIN_SIZE);
    }

    #[test]
    fn test_memory_2() {
        let m = Memory::new(123);
        assert!(m.is_ok_and(|a| a.size == 123));
    }

    #[test]
    fn test_memory_3() {
        let m = Memory::new(8);
        assert!(m.is_ok());
        let m = m.unwrap();
        assert!(m.wirte(8u64).is_ok());
        assert!(m.read::<u64>().is_ok_and(|a| a == 8u64));
    }

    #[test]
    fn test_memory_4() {
        let m = Memory::new(16);
        assert!(m.is_ok());
        let mut m = m.unwrap();

        assert!(m.wirte(8u64).is_ok());
        assert!(m.read::<u64>().is_ok_and(|a| a == 8u64));
        m.move_left(8);
        assert!(m.wirte(16u64).is_ok());
        assert!(m.read::<u64>().is_ok_and(|a| a == 16u64));
    }

    #[derive(Clone, Copy, PartialEq)]
    struct Point(f32, f32);

    #[test]
    fn test_memory_5() {
        let m = Memory::new(8);
        assert!(m.is_ok());
        let m = m.unwrap();
        assert!(m.wirte(Point(10.0, 20.0)).is_ok());
        assert!(m.read::<Point>().is_ok_and(|a| a == Point(10.0, 20.0)));
    }
}
