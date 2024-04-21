use std::collections::HashMap;

use crate::Ident;

#[derive(Debug, Clone)]
pub struct IdentTable {
    name_to_ident: HashMap<String, Ident>,
    ident_to_name: Vec<String>,
    next_ident: usize,
}

impl IdentTable {
    pub fn new() -> Self {
        Self {
            name_to_ident: HashMap::new(),
            ident_to_name: Vec::new(),
            next_ident: 0,
        }
    }

    fn new_ident(&mut self, name: &str) -> Ident {
        let ident = Ident(self.next_ident);
        self.next_ident += 1;

        self.name_to_ident.insert(name.to_string(), ident);
        self.ident_to_name.push(name.to_string());

        ident
    }

    pub fn ident(&mut self, name: &str) -> Ident {
        if let Some(&ident) = self.name_to_ident.get(name) {
            ident
        } else {
            self.new_ident(name)
        }
    }

    pub fn try_ident(&self, name: &str) -> Option<Ident> {
        self.name_to_ident.get(name).copied()
    }

    pub fn name(&self, ident: Ident) -> &str {
        &self.ident_to_name[ident.0]
    }
}
