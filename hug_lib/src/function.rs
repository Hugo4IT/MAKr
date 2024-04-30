use crate::{
    value::{HugExternalFunction, HugValue},
    Ident,
};

#[derive(Debug, Clone)]
pub enum HugFunction {
    Hug {
        scope_id: usize,
        arguments: Vec<HugFunctionArgument>,
    },
    External {
        function_pointer: HugExternalFunction,
    },
}

#[derive(Debug, Clone)]
pub struct HugFunctionArgument {
    pub ident: Ident,
    pub default_value: Option<HugValue>,
}
