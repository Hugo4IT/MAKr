use hug_lib::ident_table::IdentTable;
use parser::{generate_pairs, TokenPair};
use tokenizer::{Token, TokenKind, Tokenizer};

pub mod parser;
pub mod tokenizer;

pub trait FilterUseless {
    fn filter_useless(self) -> Self;
}

impl FilterUseless for Vec<TokenPair> {
    fn filter_useless(self) -> Self {
        let mut new_self = Vec::with_capacity(self.capacity());

        for pair in self {
            match pair.token.kind {
                TokenKind::LineComment | TokenKind::BlockComment | TokenKind::Whitespace => (),
                _ => new_self.push(TokenPair {
                    text: pair.text.clone(),
                    token: pair.token,
                }),
            }
        }

        new_self.shrink_to_fit();
        new_self
    }
}

pub trait CustomDisplay {
    fn display(&self) -> String;
}
impl CustomDisplay for Vec<TokenPair> {
    fn display(&self) -> String {
        let mut buffer = String::new();
        let max_len = self
            .iter()
            .fold(0, |a, p| a.max(format!("{:?}", p.token.kind).len()));

        buffer.push_str(&format!("(Size {})\n", self.len()));
        buffer.push_str(&format!("{0}==={0}\n", "=".repeat(max_len + 1)));
        buffer.push_str(&format!(
            " TokenKind{} | Original text\n",
            " ".repeat(max_len - 8)
        ));

        buffer.push_str(&format!("{0}==={0}\n", "=".repeat(max_len + 1)));

        for pair in self.iter() {
            buffer.push_str(&format!(
                " {:?}:{} | \"{}\"\n",
                pair.token.kind,
                " ".repeat(max_len - format!("{:?}", pair.token.kind).len()),
                pair.text.escape_debug()
            ));
        }

        buffer.push_str(&format!("{0}==={0}\n", "=".repeat(max_len + 1)));
        buffer
    }
}

pub fn lex(program: &str) -> Vec<TokenPair> {
    let tokens = tokenize(program);
    generate_pairs(program, tokens)
}

pub fn tokenize(program: &str) -> Vec<Token> {
    let mut ident_table = IdentTable::new();

    let mut buffer = String::new();
    buffer.push_str("import core");

    Tokenizer::new(&mut ident_table, program).tokenize()
}

pub fn run_test(program: &str, expected_result: &[(TokenKind, usize)]) {
    let tokens = tokenize(program);
    for (token, (expected_kind, expected_len)) in tokens.iter().zip(expected_result.iter()) {
        println!(
            "Token: {:?}, Expected: {:?}, len: {}",
            *token, expected_kind, expected_len
        );
        assert_eq!(token.kind, *expected_kind);
        assert_eq!(token.len, *expected_len);
    }
}
