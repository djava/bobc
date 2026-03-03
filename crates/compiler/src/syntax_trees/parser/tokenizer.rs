use peg::*;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take_while},
    combinator::{all_consuming, eof, peek, recognize},
    multi::many0,
};
use nom_locate::LocatedSpan;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenValue<'a> {
    Identifier(&'a str),
    Int(i64),
    Bool(bool),
    OpenParen,
    CloseParen,
    Equals,
    Plus,
    Minus,
    Comma,
    Newline,
    DoubleEquals,
    NotEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
    Not,
    And,
    Or,
    If,
    Else,
    OpenCurly,
    CloseCurly,
    QuestionMark,
    Colon,
    While,
    Is,
    OpenBracket,
    CloseBracket,
    Asterisk,
    Fn,
    IntType,
    BoolType,
    TupleType,
    ArrayType,
    CallableType,
    NoneType,
    RightArrow,
    Return,
    Lambda,
    For,
    Semicolon,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token: TokenValue<'a>,
    pub position: LocatedSpan<&'a str>,
}

const ID_START_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
const WORD_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_";

macro_rules! keyword_token {
    ($string: expr, $tok: expr) => {
        (tag($string), peek(word_delimiter)).map(|(s, _)| (s, $tok))
    };
}

macro_rules! punctuation_token {
    ($string: expr, $tok: expr) => {
        tag($string).map(|s| (s, $tok))
    };
}

fn word_delimiter(rem: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, ()> {
    peek(is_not(WORD_CHARS)).map(|_| ()).parse(rem)
}

fn newline<'a>(rem: LocatedSpan<&'a str>) -> IResult<LocatedSpan<&'a str>, Token<'a>> {
    let (rem, span) = is_a("\r\n").parse(rem)?;

    Ok((
        rem,
        Token {
            token: TokenValue::Newline,
            position: span,
        },
    ))
}

fn whitespace(input: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, ()> {
    alt((tag(" "), tag("\t"))).map(|_| ()).parse(input)
}

fn keyword_parser<'a>(rem: LocatedSpan<&'a str>) -> IResult<LocatedSpan<&'a str>, Token<'a>> {
    use TokenValue::*;

    // let (rem, _) = position(input)?;
    let (rem, (span, token)) = alt((
        keyword_token!("true", Bool(true)),
        keyword_token!("false", Bool(false)),
        keyword_token!("if", If),
        keyword_token!("else", Else),
        keyword_token!("while", While),
        keyword_token!("for", For),
        keyword_token!("and", And),
        keyword_token!("or", Or),
        keyword_token!("not", Not),
        keyword_token!("is", Is),
        keyword_token!("fn", Fn),
        keyword_token!("int", IntType),
        keyword_token!("bool", BoolType),
        keyword_token!("tuple", TupleType),
        keyword_token!("array", ArrayType),
        keyword_token!("callable", CallableType),
        keyword_token!("none", NoneType),
        keyword_token!("return", Return),
        keyword_token!("lambda", Lambda),
    ))
    .parse(rem)?;

    Ok((rem, Token { token, position: span }))
}

fn punctuation_parser<'a>(rem: LocatedSpan<&'a str>) -> IResult<LocatedSpan<&'a str>, Token<'a>> {
    use TokenValue::*;
    
    // let (rem, _) = position(input)?;
    let (rem, (span, token)) = alt((
        alt((
            punctuation_token!("->", RightArrow),
            punctuation_token!("==", DoubleEquals),
            punctuation_token!("!=", NotEquals),
            punctuation_token!(">=", GreaterEquals),
            punctuation_token!("<=", LessEquals),
            punctuation_token!(">", Greater),
            punctuation_token!("<", Less),
            punctuation_token!("&&", And),
            punctuation_token!("||", Or),
            punctuation_token!("!", Not),
            punctuation_token!("(", OpenParen),
            punctuation_token!(")", CloseParen),
            punctuation_token!("=", Equals),
            punctuation_token!("+", Plus),
            punctuation_token!("-", Minus),
            punctuation_token!(",", Comma),
            punctuation_token!("{", OpenCurly),
            punctuation_token!("}", CloseCurly),
            punctuation_token!("[", OpenBracket),
            punctuation_token!("]", CloseBracket),
            punctuation_token!("?", QuestionMark),
        )), // Each `alt` supports a max of 21 choices
        alt((
            punctuation_token!(":", Colon),
            punctuation_token!("*", Asterisk),
            punctuation_token!(";", Semicolon),
        )),
    ))
    .parse(rem)?;


    Ok((rem, Token { token, position: span }))
}

fn int_parser(rem: LocatedSpan<&'_ str>) -> IResult<LocatedSpan<&'_ str>, Token<'_>> {
    let (rem, token_span) = recognize(nom::character::complete::i64)
        .parse(rem)?;

    let int_val = token_span.clone().into_fragment().parse::<i64>().expect("Couldn't parse i64");
    let token = TokenValue::Int(int_val);

    Ok((rem, Token { token, position: token_span }))
}

fn id_parser(rem: LocatedSpan<&'_ str>) -> IResult<LocatedSpan<&'_ str>, Token<'_>> {
    let (rem, id_span) = take_while(|c| WORD_CHARS.contains(c))(rem)?;


    // Make sure the first char is a valid start char
    let _ = is_a(ID_START_CHARS)(id_span)?;

    Ok((
        rem,
        Token {
            token: TokenValue::Identifier(id_span.clone().into_fragment()),
            position: id_span,
        },
    ))
}

fn token_parser(rem: LocatedSpan<&'_ str>) -> IResult<LocatedSpan<&'_ str>, Token<'_>> {
    let (rem, _) = many0(whitespace).parse(rem)?;

    let tok = alt((
        newline,
        keyword_parser,
        int_parser,
        punctuation_parser,
        id_parser,
    ))
    .parse(rem);

    tok
}

pub fn tokenize(
    input: &'_ str,
) -> Result<Vec<Token<'_>>, nom::Err<nom::error::Error<LocatedSpan<&'_ str>>>> {
    let res = all_consuming((many0(token_parser), eof)).parse(LocatedSpan::new(input));

    if let Ok((rem, (tokens, _))) = res {
        assert!(rem.is_empty());
        dbg!(&tokens);
        Ok(tokens)
    } else if let Err(err) = res {
        Err(err)
    } else {
        unreachable!()
    }
}
