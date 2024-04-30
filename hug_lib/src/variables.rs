use crate::{value::HugValue, Ident};

#[derive(Debug, Clone)]
pub struct Variables {
    inner: Vec<Option<HugValue>>,
}

impl Variables {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    fn ensure_size(&mut self, size: usize) {
        if self.inner.len() < size + 1 {
            self.inner
                .extend((0..(size - self.inner.len() + 1)).map(|_| None));
        }
    }

    pub fn set(&mut self, ident: Ident, value: HugValue) {
        self.ensure_size(ident.0);

        let _ = self.inner[ident.0].insert(value);
    }

    pub fn get(&self, ident: Ident) -> Option<&HugValue> {
        self.inner.get(ident.0).and_then(|h| h.as_ref())
    }

    pub fn get_mut(&mut self, ident: Ident) -> Option<&mut HugValue> {
        self.ensure_size(ident.0);

        self.inner[ident.0].as_mut()
    }
}

impl Default for Variables {
    fn default() -> Self {
        Self::new()
    }
}
