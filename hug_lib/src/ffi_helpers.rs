use std::mem::ManuallyDrop;

use crate::value::{FromHugValue, HugValue};

#[derive(Debug)]
#[repr(C)]
pub struct ExportDescriptor {
    pub function_args: usize,
    pub function_varargs: bool,
    /// All function argument names seperated by spaces ( )
    pub function_arg_names: *const libc::c_char,
    /// All function argument types seperated by spaces ( )
    pub function_arg_types: *const libc::c_char,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct PackedArgs {
    pub args_len: usize,
    pub args: *const Option<HugValue>,
}

impl PackedArgs {
    pub fn pack(args: &[Option<HugValue>]) -> Self {
        Self {
            args_len: args.len(),
            args: args.as_ptr(),
        }
    }

    pub fn unpack<'a>(self) -> Args<'a> {
        Args {
            cursor: 0,
            values: ManuallyDrop::new(unsafe {
                core::slice::from_raw_parts(self.args, self.args_len)
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Args<'a> {
    cursor: usize,
    values: ManuallyDrop<&'a [Option<HugValue>]>,
}

impl<'a> Args<'a> {
    fn next<T: FromHugValue<'a>>(&mut self) -> Option<T> {
        if self.cursor >= self.values.len() {
            return None;
        }

        let output = self.values[self.cursor]
            .as_ref()
            .map(|value| T::from_hug_value(value).expect("Type error"));

        self.cursor += 1;

        output
    }

    pub fn arg<T: FromHugValue<'a>>(&mut self, name: &'static str) -> Option<T> {
        self.next()
    }

    pub fn collect_remaining<T: FromHugValue<'a>>(mut self) -> Vec<T> {
        Vec::from_iter(core::iter::from_fn(|| self.next()))
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ReturnValue {
    pub values_len: usize,
    pub values: *mut HugValue,
}

impl ReturnValue {
    pub fn pack<T>(value: T) -> Self
    where
        T: Into<HugValue>,
    {
        Self {
            values_len: 1,
            values: Box::into_raw(Box::new(value.into())),
        }
    }

    pub fn unpack(self) -> HugValue {
        unsafe { *Box::from_raw(self.values) }
    }
}
