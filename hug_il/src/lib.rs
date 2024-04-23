//! The intermediate language for the Hug programming language

pub type Word = u16;
pub type Dword = u32;
pub type Qword = u64;

#[derive(Debug, Clone, Copy)]
pub enum Width {
    Byte,
    Word,
    Dword,
    Qword,
    Ptr,
}

#[derive(Debug, Clone, Copy)]
pub enum Precision {
    Half,
    Single,
    Double,
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Address(pub usize);

#[derive(Debug, Clone)]
pub struct Header {
    pub link_libraries: Vec<ExternalLibrary>,
}

#[derive(Debug, Clone)]
pub struct ExternalLibrary {
    /// Name of the library
    pub name: String,
    pub symbols: Vec<ExternalSymbol>,
}

#[derive(Debug, Clone)]
pub struct ExternalSymbol {
    /// The identifier associated with the symbol
    pub symbol: String,
    /// The address in the data section to store the function pointer in
    pub address: Address,
}

#[derive(Debug, Clone, Copy)]
pub struct LoadInfo<T> {
    pub address: usize,
    pub inner: T,
}

#[derive(Debug, Clone, Copy)]
pub enum HugOp {
    //
    // Stack manipulation
    //
    /// Push to the stack
    Push(Push, Width),
    /// Pop from the stack
    Pop(Pop, Width),
    /// Call a function
    Call(Call),
    /// Return from a function, specifying the width of the return value
    Return(Width),
    /// Return from a function, without a return value
    ReturnVoid,

    //
    // Arithmetic
    //
    /// Add two numbers
    Add(OpSpec),
    /// Subtract two numbers
    Sub(OpSpec),
    /// Multiply two numbers
    Mul(OpSpec),
    /// Divide two numbers
    Div(OpSpec),
    /// Negate a number
    Neg(OpSpec),

    //
    // Boolean logic
    //
    /// True if two booleans equal true
    And,
    /// True if one of two booleans equal true
    Or,
    /// Invert a boolean value
    Not,

    //
    // Comparisons
    //
    /// Compare two values, true if equal
    Eq(Width),
    /// Compare two values, true if left is greater than right
    Gt(Width),
    /// Compare two values, true if left is less than right
    Lt(Width),

    //
    // Bitwise logic
    //
    /// Shift bits right, discarding overflow
    Shr(Width, u8),
    /// Shift bits left, discarding overflow
    Shl(Width, u8),
    /// Shift bits right, wrapping overflow
    ShrW(Width, u8),
    /// Shift bits left, wrapping overflow
    ShlW(Width, u8),
    /// Flip bits
    Flip(Width),
    /// Bitwise AND two values
    BitAnd(Width),
    /// Bitwise OR two values
    BitOr(Width),
    /// Bitwise XOR two values
    BitXor(Width),
}

#[derive(Debug, Clone, Copy)]
pub enum Push {
    /// A constant value
    Constant(Qword),
    /// A value located in the data section of the binary at `Address`
    Data(Address),
    /// A function argument (indexed with `usize`)
    Argument(usize),
    /// A local variable (indexed with `usize`)
    Local(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum Pop {
    /// A function argument (indexed with `usize`)
    Argument(usize),
    /// A local variable (indexed with `usize`)
    Local(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum Call {
    /// Call a hug function located at `Address`
    Hug(Address),
    /// Call a native function from an external library using the function pointer
    /// located at `Address` in the data section.
    Native(Address),
}

#[derive(Debug, Clone, Copy)]
pub enum OpSpec {
    Integer { signed: bool, width: Width },
    Float { precision: Precision },
}
