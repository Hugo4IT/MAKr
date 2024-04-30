use std::fmt::Display;

use hug_lexer::parser::TokenPair;
use hug_lib::{function::HugFunctionArgument, value::HugValue, Ident};
use parser::HugTreeParser;
use scope::HugScope;

pub mod parser;
pub mod scope;

#[derive(Debug, Clone)]
pub enum HugTreeEntry {
    ModuleDefinition {
        module: Ident,
    },
    ExternalTypeDefinition {
        _type: Ident,
    },
    ExternalModuleDefinition {
        module: Ident,
        location: String,
    },
    VariableDefinition {
        variable: Ident,
        value: Expression,
    },
    FunctionDefinition {
        ident: Ident,
        arguments: Vec<HugFunctionArgument>,
    },
    Expression(Expression),
    Import {
        path: Vec<Ident>,
    },
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

impl Expression {
    pub fn is_constant(&self) -> bool {
        match self {
            Self::Literal(_) => true,
            _ => false,
        }
    }

    pub fn get_constant_value(self) -> Option<HugValue> {
        match self {
            Self::Literal(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct HugTree {
    pub root: HugScope,
}

impl HugTree {
    pub fn new() -> Self {
        Self {
            root: HugScope::new(),
        }
    }

    pub fn merge_with(&mut self, mut other: HugTree) {
        // self.entries.append(&mut other.entries);
    }

    pub fn from_token_pairs(pairs: Vec<TokenPair>) -> HugTree {
        HugTreeParser::new(pairs).parse()
    }
}

impl Display for HugTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let mut buffer = String::new();
        // for (i, entry) in self.entries.iter().enumerate() {
        //     buffer.push_str(&format!("{:4}: {:?},\n", i, entry));
        // }
        // write!(f, "[\n{}]", buffer)
        Ok(())
    }
}

pub struct HugType {}
