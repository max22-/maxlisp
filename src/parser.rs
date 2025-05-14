use crate::gc_heap::{GcHeap, Handle};
use crate::interner::Interner;
use crate::lexer::{Lexer, Token, TokenType};
use crate::sexp::Sexp;

#[derive(Debug)]
pub enum ParseErrorType {
    StringNotTerminated,
    FailedToParseInteger,
    UnexpectedRPAREN,
    UnexpectedEOF,
    MalformedList,
}

impl std::fmt::Display for ParseErrorType {
    fn fmt(self: &Self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::StringNotTerminated => write!(fmt, "string not terminated"),
            Self::FailedToParseInteger => write!(fmt, "failed to parse integer"),
            Self::UnexpectedRPAREN => write!(fmt, "unexpected `)`"),
            Self::UnexpectedEOF => write!(fmt, "unexpected end of file"),
            Self::MalformedList => write!(fmt, "malformed list"),
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub r#type: ParseErrorType,
    pub pos: usize,
}

impl ParseError {
    pub fn to_string(self: &Self, file_path: &String, source: &String) -> String {
        let mut line: usize = 1;
        let mut column: usize = 1;
        for i in 0..self.pos {
            if i >= source.len() {
                unreachable!()
            }
            if source.as_bytes()[i] == b'\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        format!("{}:{}:{}: {}", file_path, line, column, self.r#type)
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    look: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a String) -> Self {
        return Self {
            lexer: Lexer::new(source),
            look: None,
        };
    }

    fn advance(self: &mut Self) -> Result<(), ParseError> {
        let t = self.lexer.next_token()?;
        self.look = t;
        Ok(())
    }

    fn make_error(self: &Self, r#type: ParseErrorType) -> Result<Handle, ParseError> {
        return Err(ParseError {
            r#type: r#type,
            pos: if let Some(t) = &self.look { t.pos } else { 0 },
        });
    }

    fn parse_cdr(
        self: &mut Self,
        heap: &mut GcHeap,
        interner: &mut Interner,
    ) -> Result<Handle, ParseError> {
        match &self.look {
            None => self.make_error(ParseErrorType::UnexpectedEOF),
            Some(t) => match t.r#type {
                TokenType::RPAREN => Ok(heap.alloc(Sexp::Nil)),
                TokenType::DOT => {
                    self.advance()?;
                    let form = self.next_form(heap, interner)?;
                    if let Some(cdr) = form {
                        if let Some(t) = &self.look {
                            if t.r#type != TokenType::RPAREN {
                                self.make_error(ParseErrorType::MalformedList)
                            } else {
                                Ok(cdr)
                            }
                        } else {
                            self.make_error(ParseErrorType::UnexpectedEOF)
                        }
                    } else {
                        self.make_error(ParseErrorType::UnexpectedEOF)
                    }
                }
                _ => {
                    let form = self.next_form(heap, interner)?;
                    if let Some(car) = form {
                        let cdr = self.parse_cdr(heap, interner)?;
                        Ok(heap.alloc(Sexp::Pair(car, cdr)))
                    } else {
                        self.make_error(ParseErrorType::UnexpectedEOF)
                    }
                }
            },
        }
    }

    fn parse_list(
        self: &mut Self,
        heap: &mut GcHeap,
        interner: &mut Interner,
    ) -> Result<Handle, ParseError> {
        self.advance()?; // skip the '('
        let first = if let Some(s) = self.next_form(heap, interner)? {
            s
        } else {
            self.make_error(ParseErrorType::UnexpectedEOF)?
        };
        let cdr = self.parse_cdr(heap, interner)?;
        let result = heap.alloc(Sexp::Pair(first, cdr));
        self.advance()?; // skip the ')'
        Ok(result)
    }

    pub fn next_form(
        self: &mut Self,
        heap: &mut GcHeap,
        interner: &mut Interner,
    ) -> Result<Option<Handle>, ParseError> {
        if self.look == None {
            self.advance()?;
        }
        match &self.look {
            None => Ok(None),
            Some(t) => match t.r#type {
                TokenType::INTEGER => {
                    let i = match t.val.parse::<i64>() {
                        Ok(i) => i,
                        Err(_) => {
                            return Err(ParseError {
                                r#type: ParseErrorType::FailedToParseInteger,
                                pos: t.pos,
                            });
                        }
                    };
                    self.advance()?;
                    Ok(Some(heap.alloc(Sexp::Integer(i))))
                }
                TokenType::SYMBOL => {
                    let result = Sexp::Symbol(interner.intern(t.val.clone()));
                    self.advance()?;
                    Ok(Some(heap.alloc(result)))
                }
                TokenType::STRING => {
                    let result = Sexp::String(t.val.clone());
                    self.advance()?;
                    Ok(Some(heap.alloc(result)))
                }
                TokenType::LPAREN => Ok(Some(self.parse_list(heap, interner)?)),
                TokenType::RPAREN => Err(ParseError {
                    r#type: ParseErrorType::UnexpectedRPAREN,
                    pos: t.pos,
                }),
                _ => todo!(),
            },
        }
    }
}
