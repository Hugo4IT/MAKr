use hug_lib::{ident_table::IdentTable, variables::Variables};

use crate::HugTreeEntry;

#[derive(Debug, Clone)]
pub struct HugScope {
    pub members: Variables,
    pub idents: IdentTable,
    pub entries: Vec<HugTreeEntry>,
}

impl HugScope {
    pub fn new() -> Self {
        Self {
            members: Variables::new(),
            idents: IdentTable::new(),
            entries: Vec::new(),
        }
    }
}
