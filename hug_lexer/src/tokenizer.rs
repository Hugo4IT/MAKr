use std::str::Chars;

use hug_lib::{ident_table::IdentTable, Ident};

type TokenList = Vec<Token>;

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    // Comments
    LineComment,  //  //
    BlockComment, //  /* .. */

    Whitespace, //  \s,\n,\n\r, etc.

    Literal(LiteralKind),       //  420, "nice", 6.9, 'F'
    Keyword(KeywordKind),       //  var, function, type, module
    Identifier(Ident),          //  var [this] = 10
    Annotation(AnnotationKind), //  @
    BuiltInType(TypeKind),      //  Int32, Float64, String

    // Not specific to any usage
    Comma,            //  ,
    Dot,              //  .
    OpenParenthesis,  //  (
    CloseParenthesis, //  )
    OpenBrace,        //  {
    CloseBrace,       //  }
    OpenBracket,      //  [
    CloseBracket,     //  ]
    Colon,            //  :

    // Operators
    Assign,         //  =
    Add,            //  +
    Subtract,       //  -
    Multiply,       //  *
    Divide,         //  /
    Modulus,        //  %
    AddAssign,      //  +=
    SubtractAssign, //  -=
    MultiplyAssign, //  *=
    DivideAssign,   //  /=
    ModulusAssign,  //  %=

    // Conditionals
    Not,                 //  !
    And,                 //  &&
    Or,                  //  ||
    IsEqualTo,           //  ==
    IsNotEqualTo,        //  !=
    LessThan,            //  <
    GreaterThan,         //  >
    LessThanOrEquals,    //  <=
    GreaterThanOrEquals, //  >=

    // Binary operators
    BinaryAnd,          //  &
    BinaryOr,           //  |
    BinaryNot,          //  ~
    BinaryXOr,          //  ^
    BinaryAndAssign,    //  &=
    BinaryOrAssign,     //  |=
    BinaryNotAssign,    //  ~=
    BinaryXOrAssign,    //  ^=
    ShiftLeft,          //  <<
    ShiftRight,         //  >>
    ShiftLeftOverflow,  //  <<<
    ShiftRightOverflow, //  >>>

    Unknown, // Error
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeKind {
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    Float32,
    Float64,
    String,
    Other(Ident),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnotationKind {
    Extern,
    Other(Ident),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeywordKind {
    Enum,
    Function,
    Let,
    Module,
    Private,
    Public,
    Type,
    Use,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    Integer(Base),
    Float(Base),
    Char,
    String,
    RawString,
    FormatString,
    Boolean,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    Binary,
    Octal,
    Hexadecimal,
    Decimal,
}

impl TokenKind {
    pub fn expect_literal(self) -> Option<LiteralKind> {
        if let Self::Literal(k) = self {
            Some(k)
        } else {
            None
        }
    }

    pub fn expect_keyword(self) -> Option<KeywordKind> {
        if let Self::Keyword(k) = self {
            Some(k)
        } else {
            None
        }
    }

    pub fn expect_ident(self) -> Option<Ident> {
        if let Self::Identifier(id) = self {
            Some(id)
        } else {
            None
        }
    }

    pub fn expect_kind(self, kind: TokenKind) -> Option<Self> {
        if self == kind {
            Some(self)
        } else {
            None
        }
    }

    pub fn expect_type(self) -> Option<TypeKind> {
        if let TokenKind::BuiltInType(k) = self {
            Some(k)
        } else {
            None
        }
    }
}

pub struct Tokenizer<'a> {
    pub len: usize,
    pub chars: Chars<'a>,
    pub idents: &'a mut IdentTable,
}

impl<'a> Tokenizer<'a> {
    pub fn new(idents: &'a mut IdentTable, program: &'a str) -> Self {
        Self {
            len: program.len(),
            chars: program.chars(),
            idents,
        }
    }

    pub fn consumed_len(&self) -> usize {
        self.len - self.chars.as_str().len()
    }

    pub fn reset_consumed_len(&mut self) {
        self.len = self.chars.as_str().len();
    }

    pub fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn peek_next(&self) -> char {
        self.chars.clone().next().unwrap_or('\0')
    }

    pub fn peek_next_next(&self) -> char {
        let mut chars = self.chars.clone();
        chars.next().expect("Unexpected end of file!");
        chars.next().unwrap_or('\0')
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn ignore_until(&mut self, condition: impl Fn(char) -> bool) {
        while !condition(self.peek_next()) && !self.is_eof() {
            self.next().unwrap();
        }
    }

    pub fn line_comment(&mut self) -> TokenKind {
        self.next().unwrap(); // Skip /[/]
        self.ignore_until(|c| c == '\n');
        self.next();
        TokenKind::LineComment
    }

    pub fn block_comment(&mut self) -> TokenKind {
        self.next().unwrap(); // Skip /[*]
        let mut can_end = false;
        while let Some(c) = self.next() {
            match c {
                '*' => can_end = true,
                '/' if can_end => break,
                _ => (),
            }
        }
        self.next();
        TokenKind::BlockComment
    }

    pub fn operator(&mut self, operator: TokenKind) -> TokenKind {
        if self.peek_next() == '=' {
            self.next().unwrap(); // Skip <operator>[=]
            match operator {
                TokenKind::Add => TokenKind::AddAssign,
                TokenKind::Subtract => TokenKind::SubtractAssign,
                TokenKind::Multiply => TokenKind::MultiplyAssign,
                TokenKind::Divide => TokenKind::DivideAssign,
                TokenKind::Modulus => TokenKind::ModulusAssign,
                TokenKind::BinaryNot => TokenKind::BinaryNotAssign,
                TokenKind::BinaryXOr => TokenKind::BinaryXOrAssign,
                TokenKind::BinaryAnd => TokenKind::BinaryAndAssign,
                TokenKind::BinaryOr => TokenKind::BinaryOrAssign,
                other => panic!("Unrecognized operator: {:?}", other),
            }
        } else {
            operator
        }
    }

    pub fn whitespace(&mut self) -> TokenKind {
        self.ignore_until(|c| !c.is_whitespace());
        TokenKind::Whitespace
    }

    pub fn string(&mut self) -> TokenKind {
        let mut is_escaped = false;
        while let Some(c) = self.next() {
            match c {
                '\\' => is_escaped = true,
                '"' if !is_escaped => break,
                _ if is_escaped => is_escaped = false,
                _ => (),
            }
        }
        TokenKind::Literal(LiteralKind::String)
    }

    pub fn format_string(&mut self) -> TokenKind {
        self.next().unwrap(); // Ignore f["]
        self.string();
        TokenKind::Literal(LiteralKind::FormatString)
    }

    pub fn char(&mut self) -> TokenKind {
        self.next().unwrap(); // Skip '[<char>]'
        self.next().unwrap(); // Skip '<char>[']
        TokenKind::Literal(LiteralKind::Char)
    }

    pub fn number(&mut self, starts_with_zero: bool) -> TokenKind {
        let mut kind = None;
        let base = if starts_with_zero {
            match self.peek_next() {
                'b' => Base::Binary,
                'o' => Base::Octal,
                'x' => Base::Hexadecimal,
                _ => Base::Decimal,
            }
        } else {
            Base::Decimal
        };

        while !self.is_eof() {
            let c = self.peek_next();
            if c == '.' || c == 'f' {
                if kind.is_none() {
                    kind = Some(LiteralKind::Float(base));
                } else {
                    break;
                }
            } else if !c.is_numeric() && c != '_' {
                break;
            }

            self.next();
        }

        TokenKind::Literal(kind.unwrap_or(LiteralKind::Integer(base)))
    }

    pub fn annotation(&mut self) -> TokenKind {
        let mut buffer = String::new();
        while self.peek_next().is_alphanumeric() && !self.is_eof() {
            let c = self.next().unwrap();
            buffer.push(c);
        }

        let kind = match buffer.as_ref() {
            "extern" => AnnotationKind::Extern,
            other => {
                if other.len() == 0 {
                    return TokenKind::Unknown;
                }

                for (i, ch) in other.chars().enumerate() {
                    // If not a valid identifier name
                    if !((ch.is_alphabetic() && i == 0) || (ch.is_alphanumeric() && i != 0))
                        && ch != '_'
                    {
                        return TokenKind::Unknown;
                    }
                }

                AnnotationKind::Other(self.idents.ident(other))
            }
        };

        TokenKind::Annotation(kind)
    }

    pub fn condition(&mut self, kind: TokenKind) -> TokenKind {
        let next_char = self.peek_next();
        let new_kind = match kind {
            TokenKind::Not if next_char == '=' => TokenKind::IsNotEqualTo,
            TokenKind::BinaryAnd => {
                if next_char == '&' {
                    TokenKind::And
                } else {
                    return self.operator(kind);
                }
            }
            TokenKind::BinaryOr => {
                if next_char == '|' {
                    TokenKind::Or
                } else {
                    return self.operator(kind);
                }
            }
            TokenKind::Assign if next_char == '=' => TokenKind::IsEqualTo,
            TokenKind::LessThan if next_char == '=' => TokenKind::LessThanOrEquals,
            TokenKind::LessThan if next_char == '<' => {
                if self.peek_next_next() == '<' {
                    self.next();
                    TokenKind::ShiftLeftOverflow
                } else {
                    TokenKind::ShiftLeft
                }
            }
            TokenKind::GreaterThan if next_char == '=' => TokenKind::GreaterThanOrEquals,
            TokenKind::GreaterThan if next_char == '>' => {
                if self.peek_next_next() == '>' {
                    self.next();
                    TokenKind::ShiftRightOverflow
                } else {
                    TokenKind::ShiftRight
                }
            }
            _ => kind,
        };

        if kind != new_kind {
            self.next();
        }

        new_kind
    }

    pub fn try_keyword(&mut self, first_char: char) -> TokenKind {
        let mut buffer = String::new();
        buffer.push(first_char);

        while {
            let c = self.peek_next();
            c.is_alphanumeric() || c == '_'
        } && !self.is_eof()
        {
            buffer.push(self.next().unwrap());
        }

        match buffer.as_str() {
            "enum" => TokenKind::Keyword(KeywordKind::Enum),
            "function" => TokenKind::Keyword(KeywordKind::Function),
            "let" => TokenKind::Keyword(KeywordKind::Let),
            "module" => TokenKind::Keyword(KeywordKind::Module),
            "private" => TokenKind::Keyword(KeywordKind::Private),
            "public" => TokenKind::Keyword(KeywordKind::Public),
            "type" => TokenKind::Keyword(KeywordKind::Type),
            "use" => TokenKind::Keyword(KeywordKind::Use),
            "true" => TokenKind::Literal(LiteralKind::Boolean),
            "false" => TokenKind::Literal(LiteralKind::Boolean),
            "Int8" => TokenKind::BuiltInType(TypeKind::Int8),
            "Int16" => TokenKind::BuiltInType(TypeKind::Int16),
            "Int32" => TokenKind::BuiltInType(TypeKind::Int32),
            "Int64" => TokenKind::BuiltInType(TypeKind::Int64),
            "Int128" => TokenKind::BuiltInType(TypeKind::Int128),
            "UInt8" => TokenKind::BuiltInType(TypeKind::UInt8),
            "UInt16" => TokenKind::BuiltInType(TypeKind::UInt16),
            "UInt32" => TokenKind::BuiltInType(TypeKind::UInt32),
            "UInt64" => TokenKind::BuiltInType(TypeKind::UInt64),
            "UInt128" => TokenKind::BuiltInType(TypeKind::UInt128),
            "Float32" => TokenKind::BuiltInType(TypeKind::Float32),
            "Float64" => TokenKind::BuiltInType(TypeKind::Float64),
            "String" => TokenKind::BuiltInType(TypeKind::String),
            other => {
                if other.len() == 0 {
                    return TokenKind::Unknown;
                }

                for (i, ch) in other.chars().enumerate() {
                    // If not a valid identifier name
                    if !((ch.is_alphabetic() && i == 0) || (ch.is_alphanumeric() && i != 0))
                        && ch != '_'
                    {
                        return TokenKind::Unknown;
                    }
                }

                TokenKind::Identifier(self.idents.ident(other))
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        let ch = self.next().unwrap();
        let token_kind = match ch {
            // Comments/division
            '/' => match self.peek_next() {
                '/' => self.line_comment(),
                '*' => self.block_comment(),
                _ => self.operator(TokenKind::Divide),
            },

            // Whitespace
            c if c.is_whitespace() => self.whitespace(),

            // Format string
            'f' if self.peek_next() == '"' => self.format_string(),

            // Regular string
            '"' => self.string(),

            // Char
            '\'' => self.char(),

            // Numbers
            c @ '0'..='9' => self.number(c == '0'),

            '@' => self.annotation(),

            // Others
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '(' => TokenKind::OpenParenthesis,
            ')' => TokenKind::CloseParenthesis,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            ':' => TokenKind::Colon,

            // Common operators
            // +, +=
            '+' => self.operator(TokenKind::Add),
            // -, -=
            '-' => self.operator(TokenKind::Subtract),
            // *, *=
            '*' => self.operator(TokenKind::Multiply),
            // Divide already parsed at the top

            // Uncommon operators
            // %, %=
            '%' => self.operator(TokenKind::Modulus),
            // ~, ~=
            '~' => self.operator(TokenKind::BinaryNot),
            // ^, ^=
            '^' => self.operator(TokenKind::BinaryXOr),

            // Conditions or operators
            // =, ==
            '=' => self.condition(TokenKind::Assign),
            // !, !=
            '!' => self.condition(TokenKind::Not),
            // &, &&
            '&' => self.condition(TokenKind::BinaryAnd),
            // |, ||
            '|' => self.condition(TokenKind::BinaryOr),
            // <, <<, <<<, <=
            '<' => self.condition(TokenKind::LessThan),
            // >, >>, >>>, >=
            '>' => self.condition(TokenKind::GreaterThan),

            emoji if !emoji.is_ascii() && unic_emoji_char::is_emoji(emoji) => {
                panic!("Dont use emojis in your script!")
            }

            // Try keywords otherwise return TokenKind::Unknown
            other => self.try_keyword(other),
        };

        Token {
            len: self.consumed_len(),
            kind: token_kind,
        }
    }

    pub fn tokenize(&mut self) -> TokenList {
        let mut tokens = TokenList::new();
        while !self.is_eof() {
            self.reset_consumed_len();
            tokens.push(self.next_token());
        }
        tokens
    }
}
