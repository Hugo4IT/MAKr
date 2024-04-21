use crate::value::HugExternalFunction;

#[derive(Debug, Clone, Copy)]
pub enum HugFunction {
    Hug {
        address: usize,
    },
    External {
        function_pointer: HugExternalFunction,
    },
}
