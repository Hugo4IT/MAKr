#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind<'a> {
    // Comments
    LineComment,  //  //
    BlockComment, //  /* .. */

    Whitespace, //  \s,\n,\n\r, etc.

    Literal(LiteralKind),           //  420, "nice", 6.9, 'F'
    Keyword(KeywordKind),           //  var, function, type, module
    Identifier(&'a str),            //  var [this] = 10
    Annotation(AnnotationKind<'a>), //  @
    BuiltInType(TypeKind<'a>),      //  Int32, Float64, String

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
    Arrow,            //  ->

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
pub enum TypeKind<'a> {
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
    Other(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnotationKind<'a> {
    Extern,
    Other(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeywordKind {
    Enum,
    Fn,
    Return,
    Let,
    Module,
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

impl<'a> TokenKind<'a> {
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

    pub fn expect_ident(self) -> Option<&'a str> {
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

    pub fn expect_type(self) -> Option<TypeKind<'a>> {
        if let TokenKind::BuiltInType(k) = self {
            Some(k)
        } else {
            None
        }
    }
}

pub struct Tokenizer<'a> {
    cursor_checkpoint: usize,
    cursor: usize,
    source: &'a str,
    // pub chars: Chars<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            cursor_checkpoint: 0,
            cursor: 0,
            source,
        }
    }

    pub fn consumed_len(&self) -> usize {
        self.cursor - self.cursor_checkpoint
    }

    pub fn reset_consumed_len(&mut self) {
        self.cursor_checkpoint = self.cursor;
    }

    pub fn next(&mut self) -> char {
        if self.cursor < self.source.len() {
            let ch = self.source.as_bytes()[self.cursor] as char;

            self.cursor += 1;

            ch
        } else {
            '\0'
        }
    }

    pub fn peek(&self, steps: usize) -> char {
        if self.cursor + steps < self.source.len() {
            self.source.as_bytes()[self.cursor + steps] as char
        } else {
            '\0'
        }
    }

    pub fn is_eof(&self) -> bool {
        self.cursor >= self.source.len()
    }

    pub fn ignore_until(&mut self, condition: impl Fn(char) -> bool) {
        while !condition(self.peek(1)) && !self.is_eof() {
            self.next();
        }
    }

    pub fn line_comment(&mut self) -> TokenKind<'a> {
        self.next(); // Skip /[/]
        self.ignore_until(|c| c == '\n');
        self.next();
        TokenKind::LineComment
    }

    pub fn block_comment(&mut self) -> TokenKind<'a> {
        self.next(); // Skip /[*]
        let mut can_end = false;
        loop {
            match self.next() {
                '*' => can_end = true,
                '/' if can_end => break,
                '\0' => {
                    eprintln!("Comment not closed");
                    return TokenKind::BlockComment;
                }
                _ => (),
            }
        }
        self.next();
        TokenKind::BlockComment
    }

    pub fn operator(&mut self, operator: TokenKind<'a>) -> TokenKind<'a> {
        if self.peek(1) == '=' {
            self.next(); // Skip <operator>[=]
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

    pub fn whitespace(&mut self) -> TokenKind<'a> {
        self.ignore_until(|c| !c.is_whitespace());
        TokenKind::Whitespace
    }

    pub fn string(&mut self) -> TokenKind<'a> {
        let mut is_escaped = false;
        loop {
            match self.next() {
                '\\' => is_escaped = true,
                '"' if !is_escaped => break,
                '\0' => panic!("Unterminated string"),
                _ if is_escaped => is_escaped = false,
                _ => (),
            }
        }
        TokenKind::Literal(LiteralKind::String)
    }

    pub fn format_string(&mut self) -> TokenKind<'a> {
        self.next(); // Ignore f["]
        self.string();
        TokenKind::Literal(LiteralKind::FormatString)
    }

    pub fn char(&mut self) -> TokenKind<'a> {
        self.next(); // Skip '[<char>]'
        self.next(); // Skip '<char>[']
        TokenKind::Literal(LiteralKind::Char)
    }

    pub fn number(&mut self, starts_with_zero: bool) -> TokenKind<'a> {
        let mut kind = None;
        let base = if starts_with_zero {
            match self.peek(1) {
                'b' => Base::Binary,
                'o' => Base::Octal,
                'x' => Base::Hexadecimal,
                _ => Base::Decimal,
            }
        } else {
            Base::Decimal
        };

        while !self.is_eof() {
            let c = self.peek(1);
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

    pub fn annotation(&mut self) -> TokenKind<'a> {
        let start_index = self.cursor;

        while self.peek(1).is_alphanumeric() && !self.is_eof() {
            self.next();
        }

        let range = start_index..self.cursor;

        let kind = match &self.source[range] {
            "extern" => AnnotationKind::Extern,
            other => {
                if other.is_empty() {
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

                AnnotationKind::Other(other)
            }
        };

        TokenKind::Annotation(kind)
    }

    pub fn condition(&mut self, kind: TokenKind<'a>) -> TokenKind<'a> {
        let next_char = self.peek(1);
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
                if self.peek(2) == '<' {
                    self.next();
                    TokenKind::ShiftLeftOverflow
                } else {
                    TokenKind::ShiftLeft
                }
            }
            TokenKind::GreaterThan if next_char == '=' => TokenKind::GreaterThanOrEquals,
            TokenKind::GreaterThan if next_char == '>' => {
                if self.peek(2) == '>' {
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

    pub fn try_keyword(&mut self) -> TokenKind<'a> {
        let start_index = self.cursor - 1; // -1 for first char

        while {
            let c = self.peek(1);
            c.is_alphanumeric() || c == '_'
        } && !self.is_eof()
        {
            self.next();
        }

        let range = start_index..self.cursor;

        match &self.source[range] {
            "enum" => TokenKind::Keyword(KeywordKind::Enum),
            "fn" => TokenKind::Keyword(KeywordKind::Fn),
            "return" => TokenKind::Keyword(KeywordKind::Return),
            "let" => TokenKind::Keyword(KeywordKind::Let),
            "module" => TokenKind::Keyword(KeywordKind::Module),
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
                if other.is_empty() {
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

                TokenKind::Identifier(other)
            }
        }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        let ch = self.next();
        let token_kind = match ch {
            // Comments/division
            '/' => match self.peek(1) {
                '/' => self.line_comment(),
                '*' => self.block_comment(),
                _ => self.operator(TokenKind::Divide),
            },

            // Whitespace
            c if c.is_whitespace() => self.whitespace(),

            // Format string
            'f' if self.peek(1) == '"' => self.format_string(),

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

            '-' if self.peek(1) == '>' => {
                self.next();

                TokenKind::Arrow
            }

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
            _ => self.try_keyword(),
        };

        Token {
            len: self.consumed_len(),
            kind: token_kind,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();
        while !self.is_eof() {
            self.reset_consumed_len();
            tokens.push(self.next_token());
        }
        tokens
    }
}
