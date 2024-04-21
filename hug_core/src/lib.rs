use core::fmt;

use hug_lib::{hug_export, value::HugValue};
use rt_format::{Format, FormatArgument, NoNamedArguments, ParsedFormat};

pub const HUG_CORE_SCRIPT: &str = include_str!("../hug/core.hug");

hug_export! {
    fn add(left: i32, right: i32) -> i32 {
        left + right
    }
}

hug_export! {
    fn print(fmt: &str, ...args: &HugValue) {
        if args.is_empty() {
            println!("{fmt}");

            return;
        }

        let args = unsafe {
            core::mem::transmute::<Vec<&HugValue>, Vec<&WrappedHugValue>>(args)
        };

        // TODO: Error handling
        let string = ParsedFormat::parse(fmt, args.as_slice(), &NoNamedArguments).unwrap();
        println!("{string}");
    }
}

// For printing
#[repr(transparent)]
struct WrappedHugValue(pub HugValue);

impl<'a> FormatArgument for &'a WrappedHugValue {
    fn supports_format(&self, specifier: &rt_format::Specifier) -> bool {
        match self.0 {
            HugValue::Int8(_)
            | HugValue::Int16(_)
            | HugValue::Int32(_)
            | HugValue::Int64(_)
            | HugValue::Int128(_)
            | HugValue::UInt8(_)
            | HugValue::UInt16(_)
            | HugValue::UInt32(_)
            | HugValue::UInt64(_)
            | HugValue::UInt128(_) => true,
            HugValue::Float32(_) | HugValue::Float64(_) => matches!(
                specifier.format,
                Format::Display | Format::Debug | Format::LowerExp | Format::UpperExp
            ),
            HugValue::String(_) | HugValue::Function(_) | HugValue::Module(_) => {
                matches!(specifier.format, Format::Display | Format::Debug)
            }
        }
    }

    fn fmt_display(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }

    fn fmt_debug(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            HugValue::Int8(v) => fmt::Debug::fmt(&v, f),
            HugValue::Int16(v) => fmt::Debug::fmt(&v, f),
            HugValue::Int32(v) => fmt::Debug::fmt(&v, f),
            HugValue::Int64(v) => fmt::Debug::fmt(&v, f),
            HugValue::Int128(v) => fmt::Debug::fmt(&v, f),
            HugValue::UInt8(v) => fmt::Debug::fmt(&v, f),
            HugValue::UInt16(v) => fmt::Debug::fmt(&v, f),
            HugValue::UInt32(v) => fmt::Debug::fmt(&v, f),
            HugValue::UInt64(v) => fmt::Debug::fmt(&v, f),
            HugValue::UInt128(v) => fmt::Debug::fmt(&v, f),
            HugValue::Float32(v) => fmt::Debug::fmt(&v, f),
            HugValue::Float64(v) => fmt::Debug::fmt(&v, f),
            HugValue::String(ref v) => fmt::Debug::fmt(v, f),
            HugValue::Function(ref v) => write!(f, "<Function {v:?}>"),
            HugValue::Module(ref v) => write!(f, "<Module {v:?}>"),
        }
    }

    fn fmt_octal(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            HugValue::Int8(v) => fmt::Octal::fmt(&v, f),
            HugValue::Int16(v) => fmt::Octal::fmt(&v, f),
            HugValue::Int32(v) => fmt::Octal::fmt(&v, f),
            HugValue::Int64(v) => fmt::Octal::fmt(&v, f),
            HugValue::Int128(v) => fmt::Octal::fmt(&v, f),
            HugValue::UInt8(v) => fmt::Octal::fmt(&v, f),
            HugValue::UInt16(v) => fmt::Octal::fmt(&v, f),
            HugValue::UInt32(v) => fmt::Octal::fmt(&v, f),
            HugValue::UInt64(v) => fmt::Octal::fmt(&v, f),
            HugValue::UInt128(v) => fmt::Octal::fmt(&v, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_lower_hex(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            HugValue::Int8(v) => fmt::LowerHex::fmt(&v, f),
            HugValue::Int16(v) => fmt::LowerHex::fmt(&v, f),
            HugValue::Int32(v) => fmt::LowerHex::fmt(&v, f),
            HugValue::Int64(v) => fmt::LowerHex::fmt(&v, f),
            HugValue::Int128(v) => fmt::LowerHex::fmt(&v, f),
            HugValue::UInt8(v) => fmt::LowerHex::fmt(&v, f),
            HugValue::UInt16(v) => fmt::LowerHex::fmt(&v, f),
            HugValue::UInt32(v) => fmt::LowerHex::fmt(&v, f),
            HugValue::UInt64(v) => fmt::LowerHex::fmt(&v, f),
            HugValue::UInt128(v) => fmt::LowerHex::fmt(&v, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_upper_hex(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            HugValue::Int8(v) => fmt::UpperHex::fmt(&v, f),
            HugValue::Int16(v) => fmt::UpperHex::fmt(&v, f),
            HugValue::Int32(v) => fmt::UpperHex::fmt(&v, f),
            HugValue::Int64(v) => fmt::UpperHex::fmt(&v, f),
            HugValue::Int128(v) => fmt::UpperHex::fmt(&v, f),
            HugValue::UInt8(v) => fmt::UpperHex::fmt(&v, f),
            HugValue::UInt16(v) => fmt::UpperHex::fmt(&v, f),
            HugValue::UInt32(v) => fmt::UpperHex::fmt(&v, f),
            HugValue::UInt64(v) => fmt::UpperHex::fmt(&v, f),
            HugValue::UInt128(v) => fmt::UpperHex::fmt(&v, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_binary(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            HugValue::Int8(v) => fmt::Binary::fmt(&v, f),
            HugValue::Int16(v) => fmt::Binary::fmt(&v, f),
            HugValue::Int32(v) => fmt::Binary::fmt(&v, f),
            HugValue::Int64(v) => fmt::Binary::fmt(&v, f),
            HugValue::Int128(v) => fmt::Binary::fmt(&v, f),
            HugValue::UInt8(v) => fmt::Binary::fmt(&v, f),
            HugValue::UInt16(v) => fmt::Binary::fmt(&v, f),
            HugValue::UInt32(v) => fmt::Binary::fmt(&v, f),
            HugValue::UInt64(v) => fmt::Binary::fmt(&v, f),
            HugValue::UInt128(v) => fmt::Binary::fmt(&v, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_lower_exp(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            HugValue::Int8(v) => fmt::LowerExp::fmt(&v, f),
            HugValue::Int16(v) => fmt::LowerExp::fmt(&v, f),
            HugValue::Int32(v) => fmt::LowerExp::fmt(&v, f),
            HugValue::Int64(v) => fmt::LowerExp::fmt(&v, f),
            HugValue::Int128(v) => fmt::LowerExp::fmt(&v, f),
            HugValue::UInt8(v) => fmt::LowerExp::fmt(&v, f),
            HugValue::UInt16(v) => fmt::LowerExp::fmt(&v, f),
            HugValue::UInt32(v) => fmt::LowerExp::fmt(&v, f),
            HugValue::UInt64(v) => fmt::LowerExp::fmt(&v, f),
            HugValue::UInt128(v) => fmt::LowerExp::fmt(&v, f),
            HugValue::Float32(v) => fmt::LowerExp::fmt(&v, f),
            HugValue::Float64(v) => fmt::LowerExp::fmt(&v, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_upper_exp(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            HugValue::Int8(v) => fmt::UpperExp::fmt(&v, f),
            HugValue::Int16(v) => fmt::UpperExp::fmt(&v, f),
            HugValue::Int32(v) => fmt::UpperExp::fmt(&v, f),
            HugValue::Int64(v) => fmt::UpperExp::fmt(&v, f),
            HugValue::Int128(v) => fmt::UpperExp::fmt(&v, f),
            HugValue::UInt8(v) => fmt::UpperExp::fmt(&v, f),
            HugValue::UInt16(v) => fmt::UpperExp::fmt(&v, f),
            HugValue::UInt32(v) => fmt::UpperExp::fmt(&v, f),
            HugValue::UInt64(v) => fmt::UpperExp::fmt(&v, f),
            HugValue::UInt128(v) => fmt::UpperExp::fmt(&v, f),
            HugValue::Float32(v) => fmt::UpperExp::fmt(&v, f),
            HugValue::Float64(v) => fmt::UpperExp::fmt(&v, f),
            _ => Err(fmt::Error),
        }
    }

    fn to_usize(&self) -> Result<usize, ()> {
        match self.0 {
            HugValue::Int8(v) => (v).try_into().map_err(|_| ()),
            HugValue::Int16(v) => (v).try_into().map_err(|_| ()),
            HugValue::Int32(v) => (v).try_into().map_err(|_| ()),
            HugValue::Int64(v) => (v).try_into().map_err(|_| ()),
            HugValue::Int128(v) => (v).try_into().map_err(|_| ()),
            HugValue::UInt8(v) => Ok(v as usize),
            HugValue::UInt16(v) => Ok(v as usize),
            HugValue::UInt32(v) => Ok(v as usize),
            HugValue::UInt64(v) => (v).try_into().map_err(|_| ()),
            HugValue::UInt128(v) => (v).try_into().map_err(|_| ()),
            _ => Err(()),
        }
    }
}
