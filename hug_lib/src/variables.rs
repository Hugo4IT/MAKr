use std::collections::HashMap;

use crate::{value::HugValue, Ident};

#[derive(Debug, Clone)]
pub struct Variables {
    inner: HashMap<usize, HugValue>,
}

impl Variables {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn contains(&self, ident: Ident) -> bool {
        self.inner.contains_key(&ident.0)
    }

    pub fn set(&mut self, ident: Ident, value: HugValue) {
        let _ = self.inner.insert(ident.0, value);
    }

    pub fn get(&self, ident: Ident) -> Option<&HugValue> {
        self.inner.get(&ident.0)
    }

    pub fn get_mut(&mut self, ident: Ident) -> Option<&mut HugValue> {
        self.inner.get_mut(&ident.0)
    }
}
