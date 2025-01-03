
#[derive(Debug)]
pub struct MainStack {
    inner: Vec<i64>,
}

impl MainStack {
    pub fn new(size: usize) -> Self {
        Self {
            inner: Vec::with_capacity(size),
        }
    }
    pub fn push(&mut self, value: i64) {
        self.inner.push(value);
    }
    pub fn pop(&mut self) -> Option<i64> {
        self.inner.pop()
    }
    pub fn drop(&mut self, n: usize) {
        let n = self.inner.len() - n;
        self.inner.truncate(n);
    }

    pub fn get(&self, index: usize) -> Option<&i64> {
        self.inner.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut i64> {
        self.inner.get_mut(index)
    }

}
