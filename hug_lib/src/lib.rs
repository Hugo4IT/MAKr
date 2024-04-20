pub use libc;
pub use paste;

use std::collections::HashMap;
use value::HugExternalFunction;

pub mod ffi_helpers;
pub mod value;

pub type ModuleExportsFn = extern "C" fn() -> *mut libc::c_char;
pub type ModuleDeallocStringFn = unsafe extern "C" fn(string: *mut libc::c_char);

#[macro_export]
macro_rules! hug_module {
    ($($export:ident),*) => {
        $crate::paste::paste! {
            #[no_mangle]
            extern "C" fn __HUG_MODULE_EXPORTS() -> *const $crate::libc::c_char {
                std::ffi::CString::new(
                    stringify!($($export),*)
                ).unwrap().into_raw()
            }

            #[no_mangle]
            unsafe extern "C" fn __HUG_MODULE_DEALLOC_STRING(string: *mut $crate::libc::c_char) {
                let _ = std::ffi::CString::from_raw(string);
            }
        }
    };
}

#[macro_export]
macro_rules! unwrap_args {
    ($input:ident, $($args:ty),+) => {
        (
            $($input.next().expect(&format!("Not enought arguments for function {}!", stringify!($input))).assert::<$args>().unwrap()),+
        );
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident(pub usize);

#[macro_export]
macro_rules! hug_export {
    {
        $(#[$meta:meta])*
        $vis:vis unsafe fn $fn_name:ident ( $($arg_name:ident : $arg_type:ty $(= $arg_default:expr)?),* $(, ...$varargs_name:ident: $varargs_type:ty)?) $(-> $return_type:ty)? $fn_block:block
    } => {
        $crate::paste::paste! {
            mod $fn_name {
            }

            #[allow(non_snake_case)]
            $(#[$meta])*
            unsafe fn [<$fn_name _IMPL>] ( $($arg_name : $arg_type ),* $(, $varargs_name: Vec<$varargs_type>)? ) $(-> $return_type)? $fn_block

            #[no_mangle]
            $vis unsafe extern "C" fn [<_HUG_EXPORT_ $fn_name>] (args: $crate::ffi_helpers::PackedArgs) -> $crate::ffi_helpers::ReturnValue {
                let mut args = args.unpack();

                let result = [<$fn_name _IMPL>] (
                    $(args.arg::<$arg_type>(stringify!($arg_name)) $(.or(Some($arg_default)))? .unwrap()),*
                    $(, args.collect_remaining::<$varargs_type>())?
                );

                $crate::ffi_helpers::ReturnValue::pack(result)
            }
        }
    };
    {
        $(#[$meta:meta])*
        $vis:vis fn $fn_name:ident ( $($arg_name:ident : $arg_type:ty $(= $arg_default:expr)?),* $(, ...$varargs_name:ident: $varargs_type:ty)?) $(-> $return_type:ty)? $fn_block:block
    } => {
        $crate::paste::paste! {
            mod $fn_name {
            }

            #[allow(non_snake_case)]
            $(#[$meta])*
            fn [<$fn_name _IMPL>] ( $($arg_name : $arg_type ),* $(, $varargs_name: Vec<$varargs_type>)? ) $(-> $return_type)? $fn_block

            #[no_mangle]
            $vis unsafe extern "C" fn [<_HUG_EXPORT_ $fn_name>] (args: $crate::ffi_helpers::PackedArgs) -> $crate::ffi_helpers::ReturnValue {
                let mut args = args.unpack();

                let result = [<$fn_name _IMPL>] (
                    $(args.arg::<$arg_type>(stringify!($arg_name)) $(.or(Some($arg_default)))? .unwrap()),*
                    $(, args.collect_remaining::<$varargs_type>())?
                );

                $crate::ffi_helpers::ReturnValue::pack(result)
            }
        }
    };
}
