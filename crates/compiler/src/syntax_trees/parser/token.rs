use nom_locate::LocatedSpan;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenValue<'a> {
    Identifier(&'a str),
    Int(i64),
    Bool(bool),
    StringLiteral(&'a str),
    CharLiteral(char),
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
    StringType,
    CharType,
    NoneType,
    RightArrow,
    Return,
    Lambda,
    For,
    Semicolon,
    Divide,
    Percent,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub token: TokenValue<'a>,
    pub span: LocatedSpan<&'a str>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tokens<'a> {
    pub tokens: &'a [Token<'a>],
    pub start: usize,
    pub end: usize
}

impl<'a> Tokens<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Tokens {
            tokens,
            start: 0,
            end: tokens.len(),
        }
    }
}

impl<'a> PartialEq<TokenValue<'a>> for Token<'a> {
    fn eq(&self, other: &TokenValue<'a>) -> bool {
        &self.token == other
    }
    
    fn ne(&self, other: &TokenValue<'a>) -> bool {
        !self.eq(other)
    }
}

impl<'a> nom::Input for Tokens<'a> {
    type Item = &'a Token<'a>;

    type Iter = std::slice::Iter<'a, Token<'a>>;

    type IterIndices = std::iter::Enumerate<Self::Iter>;

    fn input_len(&self) -> usize {
        self.tokens.len()
    }

    fn take(&self, index: usize) -> Self {
        Tokens {
            tokens: &self.tokens[0..index],
            start: 0,
            end: index,
        }
    }

    fn take_from(&self, index: usize) -> Self {
        Tokens {
            tokens: &self.tokens[index..],
            start: 0,
            end: self.tokens.len() - index,
        }
    }

    fn take_split(&self, index: usize) -> (Self, Self) {
        let (first, second) = self.tokens.split_at(index);
        let first_tokens = Tokens {
            tokens: first,
            start: 0,
            end: first.len()
        };
        let second_tokens = Tokens {
            tokens: second,
            start: 0,
            end: second.len()
        };

        (second_tokens, first_tokens)
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
      where
        P: Fn(Self::Item) -> bool {
        self.tokens.iter().position(predicate)
    }

    fn iter_elements(&self) -> Self::Iter {
        self.tokens.iter()
    }

    fn iter_indices(&self) -> Self::IterIndices {
        self.tokens.iter().enumerate()
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        if self.tokens.len() >= count {
            Ok(count)
        } else {
            Err(nom::Needed::Unknown)
        }
    }
}