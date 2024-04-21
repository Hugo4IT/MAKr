use std::fmt::Display;

use hug_lexer::parser::TokenPair;
use hug_lib::{value::HugValue, Ident};
use parser::HugTreeParser;

pub mod parser;

#[derive(Debug, Clone)]
pub enum HugTreeEntry {
    ModuleDefinition { module: Ident },
    ExternalTypeDefinition { _type: Ident },
    ExternalModuleDefinition { module: Ident, location: String },
    VariableDefinition { variable: Ident, value: Expression },
    Expression(Expression),
    Import { path: Vec<Ident> },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(HugValue),
    Call {
        function: Ident,
        args: Vec<Expression>,
    },
    Variable(Ident),
}

#[derive(Debug)]
pub struct HugTree {
    pub entries: Vec<HugTreeEntry>,
}

impl HugTree {
    pub fn new() -> HugTree {
        HugTree {
            entries: Vec::new(),
        }
    }

    pub fn merge_with(&mut self, other: HugTree) {
        self.entries.extend(other.entries.into_iter());
    }

    pub fn from_token_pairs(pairs: Vec<TokenPair>) -> HugTree {
        HugTreeParser::new(pairs).parse()
    }
}

impl Display for HugTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for (i, entry) in self.entries.iter().enumerate() {
            buffer.push_str(&format!("{:4}: {:?},\n", i, entry));
        }
        write!(f, "[\n{}]", buffer)
    }
}

pub struct HugType {}
