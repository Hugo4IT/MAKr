#[derive(Debug, Clone)]
pub struct SymbolRef {
    pub mangled_name: String,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub mangled_name: String,
    pub name: String,
    pub kind: SymbolKind,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Class,
    Function,
    Variable,
}

#[derive(Debug)]
pub struct Compiler {}
