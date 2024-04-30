use hug_lexer::{
    run_test,
    tokenizer::{Base, KeywordKind, LiteralKind, TokenKind},
};
use hug_lib::Ident;

const COMMENTS_PROGRAM: &str = r###"
// Hello
// This is a test
/* Of multiple diffirent comment types */
//
/**/
/*/ */
/ /
"###;

const COMMENTS_EXPECTED_RESULT: &[(TokenKind, usize)] = &[
    (TokenKind::Whitespace, 1),
    (TokenKind::LineComment, 9),
    (TokenKind::LineComment, 18),
    (TokenKind::BlockComment, 42),
    (TokenKind::LineComment, 3),
    (TokenKind::BlockComment, 5),
    (TokenKind::BlockComment, 7),
    (TokenKind::Divide, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Divide, 1),
    (TokenKind::Whitespace, 1),
];

#[test]
fn comments() {
    run_test(COMMENTS_PROGRAM, COMMENTS_EXPECTED_RESULT);
}

const VARIABLES_PROGRAM: &str = r###"
let some_Weird_hellishName: MyType[] = [MyType(50), MyType(13)]
let is_snake_case_epic = true
let andCamelCaseTooRight = true
let this_is_a_string = "wowie"
let this_is_a_formatted_string = f"This is wowie: {wowie}"
let this_is_an_escaped_string = "Hehehe, \""
"###;

const VARIABLES_EXPECTED_RESULT: &[(TokenKind, usize)] = &[
    (TokenKind::Whitespace, 1), // \n
    // var some_Weird_hellishName: MyType[] = [MyType(50), MyType(13)];
    (TokenKind::Keyword(KeywordKind::Let), 3),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier("some_Weird_hellishName"), 22),
    (TokenKind::Colon, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier("MyType"), 6),
    (TokenKind::OpenBracket, 1),
    (TokenKind::CloseBracket, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Assign, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::OpenBracket, 1),
    (TokenKind::Identifier("MyType"), 6),
    (TokenKind::OpenParenthesis, 1),
    (TokenKind::Literal(LiteralKind::Integer(Base::Decimal)), 2),
    (TokenKind::CloseParenthesis, 1),
    (TokenKind::Comma, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier("MyType"), 6),
    (TokenKind::OpenParenthesis, 1),
    (TokenKind::Literal(LiteralKind::Integer(Base::Decimal)), 2),
    (TokenKind::CloseParenthesis, 1),
    (TokenKind::CloseBracket, 1),
    (TokenKind::Whitespace, 1),
    // var is_snake_case_epic = true;
    (TokenKind::Keyword(KeywordKind::Let), 3),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier("is_snake_case_epic"), 18),
    (TokenKind::Whitespace, 1),
    (TokenKind::Assign, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Literal(LiteralKind::Boolean), 4),
    (TokenKind::Whitespace, 1),
    // var andCamelCaseTooRight = true;
    (TokenKind::Keyword(KeywordKind::Let), 3),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier("andCamelCaseTooRight"), 20),
    (TokenKind::Whitespace, 1),
    (TokenKind::Assign, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Literal(LiteralKind::Boolean), 4),
    (TokenKind::Whitespace, 1),
    // var this_is_a_string = "wowie";
    (TokenKind::Keyword(KeywordKind::Let), 3),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier("wowie"), 16),
    (TokenKind::Whitespace, 1),
    (TokenKind::Assign, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Literal(LiteralKind::String), 7),
    (TokenKind::Whitespace, 1),
    // var this_is_a_formatted_string = f"This is wowie: {wowie}";
    (TokenKind::Keyword(KeywordKind::Let), 3),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier("this_is_a_formatted_string"), 26),
    (TokenKind::Whitespace, 1),
    (TokenKind::Assign, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Literal(LiteralKind::FormatString), 25),
    (TokenKind::Whitespace, 1),
    // var this_is_an_escaped_string = "Hehehe, \"";
    (TokenKind::Keyword(KeywordKind::Let), 3),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier("this_is_an_escaped_string"), 25),
    (TokenKind::Whitespace, 1),
    (TokenKind::Assign, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Literal(LiteralKind::String), 12),
    (TokenKind::Whitespace, 1),
    (TokenKind::Whitespace, 1), // \n
];

#[test]
fn variables() {
    run_test(VARIABLES_PROGRAM, VARIABLES_EXPECTED_RESULT);
}
