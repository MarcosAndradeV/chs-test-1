use std::marker::PhantomData;

use crate::Memory;

pub struct Stack<T> {
    _marker: PhantomData<T>,
    inner: Memory,
    len: usize,
    cap: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
            inner: Memory::new(size_of::<T>() * 16).unwrap(),
            len: 0,
            cap: 16,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            _marker: PhantomData,
            inner: Memory::new(size_of::<T>() * cap).unwrap(),
            len: 0,
            cap,
        }
    }

    pub fn drop(&mut self, n: usize) {
        if self.len >= n {
            self.len -= n;
            self.inner.move_rigth(n * size_of::<T>());
        }
    }

    fn resize(&mut self, new_size: usize) {
        self.inner.realloc(new_size).unwrap();
    }

    pub fn push(&mut self, val: T) {
        if self.len >= self.cap {
            self.resize(self.cap * 2);
        }
        if self.len > 0 {
            self.inner.move_left(size_of::<T>());
        }
        self.inner.wirte(val).unwrap();
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            let val = self.inner.read::<T>().unwrap();
            self.inner.move_rigth(size_of::<T>());
            self.len -= 1;
            Some(val)
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if self.len <= index {
            None
        } else {
            self.inner.as_slice().get(index)
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if self.len <= index {
            None
        } else {
            self.inner.as_slice_mut().get_mut(index)
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_1() {
        let mut stack: Stack<i64> = Stack::new();
        stack.push(88);
        assert!(stack.pop().is_some_and(|a| a == 88));
    }

    #[test]
    fn test_stack_2() {
        let mut stack: Stack<i64> = Stack::new();
        stack.push(80);
        stack.push(90);
        stack.push(100);
        assert!(stack.get(0).is_some_and(|a| *a == 80));
        assert!(stack.get(1).is_some_and(|a| *a == 90));
        assert!(stack.get(2).is_some_and(|a| *a == 100));
    }

    #[test]
    fn test_stack_3() {
        let mut stack: Stack<i64> = Stack::new();
        stack.push(10);
        stack.push(20);
        stack.push(30);
        assert!(stack.len() == 3);
        stack.drop(2);
        assert!(stack.len() == 1);
    }

    #[test]
    fn test_stack_4() {
        let mut stack: Stack<i64> = Stack::new();
        stack.push(10);
        stack.push(20);
        stack.push(30);
        assert!(stack.pop().is_some_and(|a| a == 30));
        stack.drop(1);
        assert!(stack.len() == 1);
        assert!(stack.pop().is_some_and(|a| a == 10));
    }
}
