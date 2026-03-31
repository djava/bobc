use crate::syntax_trees::ValueType;

use super::{token::*};
use nom::branch::alt;
use nom::bytes::complete::take;
use nom::combinator::{all_consuming, map, opt, verify};
use nom::multi::{many0, many1, separated_list0};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    And,
    Or,
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
    Not,
    Is,
    Asterisk,
    LeftShift,
    RightShift,
    Divide,
    Modulo,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    Int(i64),
    Bool(bool),
    Id(&'a str),
    Unary(Operator, Box<Expr<'a>>),
    Parens(Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, Operator, Box<Expr<'a>>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
    Ternary(Box<Expr<'a>>, Box<Expr<'a>>, Box<Expr<'a>>),
    Tuple(Vec<Expr<'a>>),
    Array(Vec<Expr<'a>>),
    Subscript(Box<Expr<'a>>, Box<Expr<'a>>),
    Lambda(Vec<&'a str>, Vec<Statement<'a>>),
    StringLiteral(&'a str),
    CharLiteral(char),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Expr(Expr<'a>),
    Assign(&'a str, Expr<'a>, Option<ValueType>),
    SubscriptAssign(Expr<'a>, Expr<'a>, Expr<'a>),
    If(Expr<'a>, Vec<Statement<'a>>),
    ElseIf(Expr<'a>, Vec<Statement<'a>>),
    Else(Vec<Statement<'a>>),
    While(Expr<'a>, Vec<Statement<'a>>),
    Return(Option<Expr<'a>>),
    For(
        Box<Statement<'a>>,
        Expr<'a>,
        Box<Statement<'a>>,
        Vec<Statement<'a>>,
    ),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub params: Vec<(&'a str, ValueType)>,
    pub return_type: ValueType,
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module<'a> {
    pub functions: Vec<Function<'a>>,
}

// parser! {
//     // `'t` is the lifetime of the tokens specifically, not the lifetime
//     // of the input (which is the slice of tokens, and only lives for as
//     // long as whoever sliced them). The TOKENS themself have the same
//     // lifetime as the source string, since their references are to the
//     // names of the variables in the source
//     grammar parse_tree<'t>() for [TokenValue<'t>] {
//         rule eof() = [TokenValue::Newline]* ![_]
//
//         rule shift_operator() -> Operator =
//             ([TokenValue::Less] [TokenValue::Less] { Operator::LeftShift }) /
//             ([TokenValue::Greater] [TokenValue::Greater] { Operator::RightShift })
//
//         rule operator() -> Operator =
//             shift_operator() /
//             op:[TokenValue::Minus | TokenValue::Plus | TokenValue::And | TokenValue::Or | TokenValue::Not |
//                 TokenValue::DoubleEquals | TokenValue::NotEquals | TokenValue::Greater | TokenValue::GreaterEquals
//                 | TokenValue::Less | TokenValue::LessEquals | TokenValue::Is | TokenValue::Asterisk | TokenValue::Divide
//                 | TokenValue::Percent] {
//                 match op {
//                     TokenValue::Minus         => Operator::Minus,
//                     TokenValue::Plus          => Operator::Plus,
//                     TokenValue::And           => Operator::And,
//                     TokenValue::Or            => Operator::Or,
//                     TokenValue::DoubleEquals  => Operator::Equals,
//                     TokenValue::NotEquals     => Operator::NotEquals,
//                     TokenValue::Greater       => Operator::Greater,
//                     TokenValue::GreaterEquals => Operator::GreaterEquals,
//                     TokenValue::Less          => Operator::Less,
//                     TokenValue::LessEquals    => Operator::LessEquals,
//                     TokenValue::Not           => Operator::Not,
//                     TokenValue::Is            => Operator::Is,
//                     TokenValue::Asterisk      => Operator::Asterisk,
//                     TokenValue::Divide        => Operator::Divide,
//                     TokenValue::Percent       => Operator::Modulo,
//                     _ => unreachable!()
//                 }
//             }
//
//         rule int_type() -> ValueType = [TokenValue::IntType] { ValueType::IntType }
//         rule bool_type() -> ValueType = [TokenValue::BoolType] { ValueType::BoolType }
//         rule primitive_type() -> ValueType = int_type() / bool_type()
//
//         rule tuple_type() -> ValueType =
//             [TokenValue::TupleType] [TokenValue::Less] types:(_type() ++ [TokenValue::Comma]) [TokenValue::Greater]
//             { ValueType::TupleType(types) }
//
//         rule array_type() -> ValueType =
//             [TokenValue::ArrayType] [TokenValue::Less] typ:_type() [TokenValue::Greater]
//             { ValueType::ArrayType(Box::new(typ)) }
//
//         rule callable_type() -> ValueType =
//             [TokenValue::CallableType] [TokenValue::Less]
//                 [TokenValue::OpenBracket] args:(_type() ** [TokenValue::Comma]) [TokenValue::CloseBracket]
//                 return_type:([TokenValue::Comma] ret:_type() {ret})?
//             [TokenValue::Greater]
//             { ValueType::FunctionType(args, Box::new(return_type.unwrap_or(ValueType::NoneType)))}
//
//         rule none_type() -> ValueType = [TokenValue::NoneType] { ValueType::NoneType }
//
//         rule string_type() -> ValueType = [TokenValue::StringType] { ValueType::ArrayType(Box::new(ValueType::CharType)) }
//
//         rule char_type() -> ValueType = [TokenValue::CharType] { ValueType::CharType }
//
//         rule _type() -> ValueType = array_type()
//                                   / tuple_type()
//                                   / primitive_type()
//                                   / callable_type()
//                                   / none_type()
//                                   / string_type()
//                                   / char_type()
//
//         // Trailing comma is mandatory for one elem but optional for multiple
//         rule tuple_elements() -> Vec<Expr<'t>> =
//             elems:((s:(expr() **<2,50> [TokenValue::Comma]) [TokenValue::Comma]? { s }) / (e:expr() [TokenValue::Comma] { vec![e] }))
//             { elems }
//
//         rule tuple() -> Expr<'t> =
//             [TokenValue::OpenParen] elems:tuple_elements() [TokenValue::CloseParen] { Expr::Tuple(elems) }
//
//         rule array_elements() -> Vec<Expr<'t>> =
//             s:(expr() ** [TokenValue::Comma]) [TokenValue::Comma]? { s }
//
//         rule array() -> Expr<'t> =
//             [TokenValue::OpenBracket] elems:array_elements() [TokenValue::CloseBracket] { Expr::Array(elems) }
//
//         rule lambda_oneliner_body() -> Vec<Statement<'t>> = e:expr() {
//             vec![Statement::Return(Some(e))]
//         }
//
//         rule lambda() -> Expr<'t> =
//             [TokenValue::Lambda] args:(([TokenValue::Identifier(id)] { id }) ** [TokenValue::Comma])
//             [TokenValue::Colon]
//             body:(lambda_oneliner_body() / statement_body())
//             { Expr::Lambda(args, body) }
//
//         rule ternary() -> Expr<'t> =
//             cond:precedence_expr() [TokenValue::QuestionMark] pos:expr() [TokenValue::Colon] neg:expr() {
//                 Expr::Ternary(Box::new(cond), Box::new(pos), Box::new(neg))
//             }
//
//         rule expr() -> Expr<'t> =
//               ternary()
//             / tuple()
//             / array()
//             / lambda()
//             / precedence_expr()
//
//         rule precedence_expr() -> Expr<'t> = precedence!{
//             // Lowest Precendence: Infix Operators, left-associative
//             l:(@) op:operator() r:@ { Expr::Binary(Box::new(l), op, Box::new(r)) }
//             --
//             // Prefix operators
//             op:operator() val:@ { Expr::Unary(op, Box::new(val)) }
//             --
//             // Postfix operators
//             e:(@) [TokenValue::OpenBracket] idx:expr() [TokenValue::CloseBracket] { Expr::Subscript(Box::new(e), Box::new(idx)) }
//             func:@ [TokenValue::OpenParen] args:(expr() ** [TokenValue::Comma]) [TokenValue::CloseParen] { Expr::Call(Box::new(func), args) }
//             --
//             // Highest: Atoms
//             [TokenValue::Identifier(id)] { Expr::Id(id) }
//             [TokenValue::Int(val)] { Expr::Int(val) }
//             [TokenValue::Bool(val)] { Expr::Bool(val) }
//             [TokenValue::OpenParen] e:expr() [TokenValue::CloseParen] { Expr::Parens(Box::new(e)) }
//             [TokenValue::StringLiteral(s)] { Expr::StringLiteral(s) }
//             [TokenValue::CharLiteral(c)] { Expr::CharLiteral(c) }
//         }
//
//         pub rule assign_type_hint() -> Option<ValueType> = ([TokenValue::Colon] t:_type() { t })?
//
//         pub rule assign() -> Statement<'t> =
//             [TokenValue::Identifier(id)] typ:assign_type_hint() [TokenValue::Equals] e:expr() { Statement::Assign(id, e, typ) }
//
//         pub rule subscript_assign() -> Statement<'t> =
//             [TokenValue::Identifier(container)] [TokenValue::OpenBracket] idx:expr() [TokenValue::CloseBracket] [TokenValue::Equals] e:expr()
//             { Statement::SubscriptAssign(Expr::Id(container), idx, e) }
//
//         pub rule statement_body() -> Vec<Statement<'t>> =
//             [TokenValue::OpenCurly] [TokenValue::Newline]*
//             ss:(if_chain() / (s:(while_statement() / for_statement() / simple_statement()) { vec![s] })) ** ([TokenValue::Newline]+)
//             [TokenValue::Newline]* [TokenValue::CloseCurly] {
//                 ss.into_iter().flatten().collect()
//             }
//
//         pub rule if_statement() -> Statement<'t> =
//             [TokenValue::If] cond:expr() body:statement_body() {
//                 Statement::If(cond, body)
//             }
//
//         pub rule else_if_statement() -> Statement<'t> =
//             [TokenValue::Else] [TokenValue::If] cond:expr() body:statement_body() {
//                 Statement::ElseIf(cond, body)
//             }
//
//         pub rule else_statement() -> Statement<'t> =
//             [TokenValue::Else] body:statement_body() {
//                 Statement::Else(body)
//             }
//
//         /// An if-chain: if { } [else if { }]* [else { }]?
//         /// No newlines required between parts
//         pub rule if_chain() -> Vec<Statement<'t>> =
//             head:if_statement() rest:([TokenValue::Newline]* s:(else_if_statement() / else_statement()) { s })* {
//             let mut v = vec![head];
//                 v.extend(rest);
//                 v
//             }
//
//         pub rule while_statement() -> Statement<'t> =
//             [TokenValue::While] cond:expr() body:statement_body() { Statement::While(cond, body) }
//
//         pub rule for_statement() -> Statement<'t> =
//             [TokenValue::For] [TokenValue::OpenParen]
//                 init:simple_statement() [TokenValue::Semicolon]
//                 cond:expr() [TokenValue::Semicolon]
//                 incr:simple_statement()
//             [TokenValue::CloseParen] body:statement_body() {
//                 Statement::For(Box::new(init), cond, Box::new(incr), body)
//             }
//
//         pub rule return_statement() -> Statement<'t> =
//             [TokenValue::Return] val:expr()? { Statement::Return(val) }
//
//         /// Simple statements (not if-chains)
//         pub rule simple_statement() -> Statement<'t> =
//             assign() / subscript_assign() / return_statement() / (e:expr() { Statement::Expr(e) })
//
//         /// A param is a name with a type specifier
//         pub rule param() -> (&'t str, ValueType) =
//             [TokenValue::Identifier(name)] [TokenValue::Colon] t:_type() { (name, t) }
//
//         pub rule param_list() -> Vec<(&'t str, ValueType)> =
//             [TokenValue::OpenParen] params:(param() ** [TokenValue::Comma]) [TokenValue::CloseParen] { params }
//
//         pub rule return_type() -> ValueType =
//             [TokenValue::RightArrow] t:_type() { t }
//
//         pub rule function() -> Function<'t> =
//             [TokenValue::Fn] [TokenValue::Identifier(name)] params:param_list() ret:(return_type()?) body:statement_body()
//                 { Function { name, params, return_type: ret.unwrap_or(ValueType::NoneType), statements: body } }
//
//         pub rule module() -> Module<'t> =
//             [TokenValue::Newline]* functions:(function() ** ([TokenValue::Newline]+)) eof() { Module { functions } }
//     }
// }

macro_rules! token_tag {
    ($name:ident, $tok: expr) => {
        fn $name<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Tokens<'t>> {
            verify(take(1usize), |t: &Tokens<'t>| t.tokens[0] == $tok).parse(rem)
        }
    };
}

token_tag!(open_paren_tag, TokenValue::OpenParen);
token_tag!(close_paren_tag, TokenValue::CloseParen);
token_tag!(equals_tag, TokenValue::Equals);
token_tag!(plus_tag, TokenValue::Plus);
token_tag!(minus_tag, TokenValue::Minus);
token_tag!(comma_tag, TokenValue::Comma);
token_tag!(newline_tag, TokenValue::Newline);
token_tag!(double_equals_tag, TokenValue::DoubleEquals);
token_tag!(not_equals_tag, TokenValue::NotEquals);
token_tag!(greater_tag, TokenValue::Greater);
token_tag!(greater_equals_tag, TokenValue::GreaterEquals);
token_tag!(less_tag, TokenValue::Less);
token_tag!(less_equals_tag, TokenValue::LessEquals);
token_tag!(not_tag, TokenValue::Not);
token_tag!(and_tag, TokenValue::And);
token_tag!(or_tag, TokenValue::Or);
token_tag!(if_tag, TokenValue::If);
token_tag!(else_tag, TokenValue::Else);
token_tag!(open_curly_tag, TokenValue::OpenCurly);
token_tag!(close_curly_tag, TokenValue::CloseCurly);
token_tag!(question_mark_tag, TokenValue::QuestionMark);
token_tag!(colon_tag, TokenValue::Colon);
token_tag!(while_tag, TokenValue::While);
token_tag!(is_tag, TokenValue::Is);
token_tag!(open_bracket_tag, TokenValue::OpenBracket);
token_tag!(close_bracket_tag, TokenValue::CloseBracket);
token_tag!(asterisk_tag, TokenValue::Asterisk);
token_tag!(fn_tag, TokenValue::Fn);
token_tag!(int_type_tag, TokenValue::IntType);
token_tag!(bool_type_tag, TokenValue::BoolType);
token_tag!(tuple_type_tag, TokenValue::TupleType);
token_tag!(array_type_tag, TokenValue::ArrayType);
token_tag!(callable_type_tag, TokenValue::CallableType);
token_tag!(string_type_tag, TokenValue::StringType);
token_tag!(char_type_tag, TokenValue::CharType);
token_tag!(none_type_tag, TokenValue::NoneType);
token_tag!(right_arrow_tag, TokenValue::RightArrow);
token_tag!(return_tag, TokenValue::Return);
token_tag!(lambda_tag, TokenValue::Lambda);
token_tag!(for_tag, TokenValue::For);
token_tag!(semicolon_tag, TokenValue::Semicolon);
token_tag!(divide_tag, TokenValue::Divide);
token_tag!(percent_tag, TokenValue::Percent);

fn parse_identifier<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, &'t str> {
    map(
        verify(take(1usize), |t: &Tokens<'t>| {
            matches!(t.tokens[0].token, TokenValue::Identifier(_))
        }),
        |t: Tokens<'t>| {
            if let TokenValue::Identifier(name) = t.tokens[0].token {
                name
            } else {
                unreachable!()
            }
        },
    )
    .parse(rem)
}

fn parse_int<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, i64> {
    map(
        verify(take(1usize), |t: &Tokens<'t>| {
            matches!(t.tokens[0].token, TokenValue::Int(_))
        }),
        |t: Tokens<'t>| {
            if let TokenValue::Int(val) = t.tokens[0].token {
                val
            } else {
                unreachable!()
            }
        },
    )
    .parse(rem)
}

fn parse_bool<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, bool> {
    map(
        verify(take(1usize), |t: &Tokens<'t>| {
            matches!(t.tokens[0].token, TokenValue::Bool(_))
        }),
        |t: Tokens<'t>| {
            if let TokenValue::Bool(val) = t.tokens[0].token {
                val
            } else {
                unreachable!()
            }
        },
    )
    .parse(rem)
}

fn parse_string_literal<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, &'t str> {
    map(
        verify(take(1usize), |t: &Tokens<'t>| {
            matches!(t.tokens[0].token, TokenValue::StringLiteral(_))
        }),
        |t: Tokens<'t>| {
            if let TokenValue::StringLiteral(s) = t.tokens[0].token {
                s
            } else {
                unreachable!()
            }
        },
    )
    .parse(rem)
}

fn parse_char_literal<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, char> {
    map(
        verify(take(1usize), |t: &Tokens<'t>| {
            matches!(t.tokens[0].token, TokenValue::CharLiteral(_))
        }),
        |t: Tokens<'t>| {
            if let TokenValue::CharLiteral(c) = t.tokens[0].token {
                c
            } else {
                unreachable!()
            }
        },
    )
    .parse(rem)
}

macro_rules! in_brackets {
    ($x: expr) => {
        delimited(open_bracket_tag, $x, close_bracket_tag)
    };
}

macro_rules! in_parens {
    ($x: expr) => {
        delimited(open_paren_tag, $x, close_paren_tag)
    };
}

macro_rules! comma_list {
    ($x: expr) => {
        separated_list0(comma_tag, $x)
    };
}

fn parse_type<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, ValueType> {
    alt((
        map(int_type_tag, |_| ValueType::IntType),
        map(bool_type_tag, |_| ValueType::BoolType),
        map(string_type_tag, |_| ValueType::string()),
        map(char_type_tag, |_| ValueType::CharType),
        map(none_type_tag, |_| ValueType::NoneType),
        map(
            preceded(array_type_tag, in_brackets!(parse_type)),
            |elem_type| ValueType::ArrayType(Box::new(elem_type)),
        ),
        map(
            preceded(array_type_tag, in_brackets!(parse_type)),
            |elem_type| ValueType::ArrayType(Box::new(elem_type)),
        ),
        map(
            preceded(tuple_type_tag, in_brackets!(comma_list!(parse_type))),
            |elem_types| ValueType::TupleType(elem_types),
        ),
        map(
            preceded(
                callable_type_tag,
                in_brackets!(separated_pair(
                    in_brackets!(comma_list!(parse_type)),
                    comma_tag,
                    parse_type
                )),
            ),
            |(param_types, return_type)| {
                ValueType::FunctionType(param_types, Box::new(return_type))
            },
        ),
    ))
    .parse(rem)
}

fn parse_operator<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Operator> {
    alt((
        map(plus_tag, |_| Operator::Plus),
        map(minus_tag, |_| Operator::Minus),
        map(and_tag, |_| Operator::And),
        map(or_tag, |_| Operator::Or),
        map(double_equals_tag, |_| Operator::Equals),
        map(not_equals_tag, |_| Operator::NotEquals),
        map(greater_equals_tag, |_| Operator::GreaterEquals),
        map(less_equals_tag, |_| Operator::LessEquals),
        map((less_tag, less_tag), |_| Operator::LeftShift),
        map((greater_tag, greater_tag), |_| Operator::RightShift),
        map(greater_tag, |_| Operator::Greater),
        map(less_tag, |_| Operator::Less),
        map(not_tag, |_| Operator::Not),
        map(is_tag, |_| Operator::Is),
        map(asterisk_tag, |_| Operator::Asterisk),
        map(divide_tag, |_| Operator::Divide),
        map(percent_tag, |_| Operator::Modulo),
    ))
    .parse(rem)
}

fn parse_binary_expr<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Expr<'t>> {
    map((parse_expr, parse_operator, parse_expr), |(l, op, r)| {
        Expr::Binary(Box::new(l), op, Box::new(r))
    })
    .parse(rem)
}

fn parse_unary_prefix_expr<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Expr<'t>> {
    map((parse_operator, parse_expr), |(op, exp)| {
        Expr::Unary(op, Box::new(exp))
    })
    .parse(rem)
}

fn parse_subscript_expr<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Expr<'t>> {
    map((parse_expr, in_brackets!(parse_expr)), |(exp, idx)| {
        Expr::Subscript(Box::new(exp), Box::new(idx))
    })
    .parse(rem)
}

fn parse_parens_expr<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Expr<'t>> {
    map(in_parens!(parse_expr), |e| Expr::Parens(Box::new(e))).parse(rem)
}

fn parse_call_expr<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Expr<'t>> {
    map(
        (parse_expr, in_parens!(comma_list!(parse_expr))),
        |(f, args)| Expr::Call(Box::new(f), args),
    )
    .parse(rem)
}

fn parse_array_expr<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Expr<'t>> {
    map(in_brackets!(comma_list!(parse_expr)), |elems| {
        Expr::Array(elems)
    })
    .parse(rem)
}

fn parse_tuple_expr<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Expr<'t>> {
    map(in_parens!(comma_list!(parse_expr)), |elems| {
        Expr::Tuple(elems)
    })
    .parse(rem)
}

fn parse_ternary_expr<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Expr<'t>> {
    map(
        separated_pair(
            parse_expr,
            question_mark_tag,
            separated_pair(parse_expr, colon_tag, parse_expr),
        ),
        |(cond, (pos, neg))| Expr::Ternary(Box::new(cond), Box::new(pos), Box::new(neg)),
    )
    .parse(rem)
}

fn parse_lambda_expr<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Expr<'t>> {
    map(
        separated_pair(
            preceded(lambda_tag, comma_list!(parse_identifier)),
            colon_tag,
            alt((
                parse_statement_body,
                map(parse_expr, |e| vec![Statement::Return(Some(e))]),
            )),
        ),
        |(args, body)| Expr::Lambda(args, body),
    )
    .parse(rem)
}

fn parse_atom_expr<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Expr<'t>> {
    alt((
        map(parse_int, |i| Expr::Int(i)),
        map(parse_bool, |b| Expr::Bool(b)),
        map(parse_string_literal, |s| Expr::StringLiteral(s)),
        map(parse_char_literal, |c| Expr::CharLiteral(c)),
        map(parse_identifier, |i| Expr::Id(i)),
    ))
    .parse(rem)
}

fn parse_expr<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Expr<'t>> {
    alt((
        parse_ternary_expr,
        parse_tuple_expr,
        parse_array_expr,
        parse_lambda_expr,
        parse_binary_expr,
        parse_unary_prefix_expr,
        parse_subscript_expr,
        parse_call_expr,
        parse_parens_expr,
        parse_atom_expr,
    ))
    .parse(rem)
}

fn parse_assign_statement<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Statement<'t>> {
    let type_hint_parser = opt(preceded(colon_tag, parse_type));
    let normal_assign_parser = map(
        separated_pair((parse_identifier, type_hint_parser), equals_tag, parse_expr),
        |((id, hint), val)| Statement::Assign(id, val, hint),
    );

    let subscript_assign_parser = map(
        separated_pair(
            (parse_identifier, in_brackets!(parse_expr)),
            equals_tag,
            parse_expr,
        ),
        |((container, idx), val)| Statement::SubscriptAssign(Expr::Id(container), idx, val),
    );

    alt((normal_assign_parser, subscript_assign_parser)).parse(rem)
}

fn parse_while_statement<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Statement<'t>> {
    map(
        (preceded(while_tag, parse_expr), parse_statement_body),
        |(cond, body)| Statement::While(cond, body),
    )
    .parse(rem)
}

fn parse_for_statement<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Statement<'t>> {
    map(
        (
            preceded(
                for_tag,
                in_parens!((
                    parse_basic_statement,
                    preceded(semicolon_tag, parse_expr),
                    preceded(semicolon_tag, parse_basic_statement)
                )),
            ),
            parse_statement_body,
        ),
        |((init, cond, incr), body)| Statement::For(Box::new(init), cond, Box::new(incr), body),
    )
    .parse(rem)
}

fn parse_expr_statement<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Statement<'t>> {
    map(parse_expr, |e| Statement::Expr(e)).parse(rem)
}

fn parse_if_chain<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Vec<Statement<'t>>> {
    let mut if_parser = map(
        (preceded(if_tag, parse_expr), parse_statement_body),
        |(cond, body)| Statement::If(cond, body),
    );

    let else_if_parser = map(
        (
            preceded((else_tag, if_tag), parse_expr),
            parse_statement_body,
        ),
        |(cond, body)| Statement::ElseIf(cond, body),
    );

    let else_parser = map(preceded(else_tag, parse_statement_body), |body| {
        Statement::Else(body)
    });

    let mut blocks = vec![];
    let (rem, if_block) = if_parser.parse(rem)?;
    blocks.push(if_block);

    let (rem, elseif_blocks) = many0(else_if_parser).parse(rem)?;
    blocks.extend(elseif_blocks);

    if let (rem, Some(else_block)) = opt(else_parser).parse(rem)? {
        blocks.push(else_block);
        Ok((rem, blocks))
    } else {
        Ok((rem, blocks))
    }
}

fn parse_return_statement<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Statement<'t>> {
    map(preceded(return_tag, opt(parse_expr)), |e| {
        Statement::Return(e)
    })
    .parse(rem)
}

fn parse_basic_statement<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Statement<'t>> {
    alt((parse_assign_statement, parse_expr_statement)).parse(rem)
}

fn parse_fn_params<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Vec<(&'t str, ValueType)>> {
    in_parens!(comma_list!(separated_pair(
        parse_identifier,
        colon_tag,
        parse_type
    )))
    .parse(rem)
}

fn parse_statement_body<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Vec<Statement<'t>>> {
    let (rem, s) = delimited(
        open_curly_tag,
        separated_list0(
            many1(newline_tag),
            alt((
                map(
                    alt((
                        parse_while_statement,
                        parse_for_statement,
                        parse_assign_statement,
                        parse_expr_statement,
                        parse_return_statement,
                    )),
                    |s| vec![s],
                ),
                parse_if_chain,
            )),
        ),
        close_curly_tag,
    )
    .parse(rem)?;

    Ok((rem, s.into_iter().flatten().collect()))
}

fn parse_function<'t>(rem: Tokens<'t>) -> IResult<Tokens<'t>, Function<'t>> {
    let (rem, (_, name, params, opt_return_type, body)) = (
        fn_tag,
        parse_identifier,
        parse_fn_params,
        opt(preceded(right_arrow_tag, parse_type)),
        preceded(many0(newline_tag), parse_statement_body),
    )
        .parse(rem)?;

    Ok((
        rem,
        Function {
            name,
            params,
            return_type: opt_return_type.unwrap_or(ValueType::NoneType),
            statements: body,
        },
    ))
}

pub fn parse_tokens<'t>(tokens: Tokens<'t>) -> Result<Module<'t>, nom::Err<nom::error::Error<Tokens<'t>>>> {
    let (_, functions) = all_consuming(many0(parse_function))
        .parse_complete(tokens)?;

    Ok(Module { functions })
}
