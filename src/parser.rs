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

#[derive(Debug)]
pub struct ParseError {
    pub r#type: ParseErrorType,
    pub pos: usize,
}

pub struct Parser<'a> {
    source: &'a String,
    lexer: Lexer<'a>,
    look: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a String) -> Self {
        return Self {
            source: source,
            lexer: Lexer::new(source),
            look: None,
        };
    }

    fn advance(self: &mut Self) -> Result<(), ParseError> {
        let t = self.lexer.next_token()?;
        self.look = t;
        Ok(())
    }

    fn make_error(self: &Self, r#type: ParseErrorType) -> Result<Sexp, ParseError> {
        return Err(ParseError {
            r#type: r#type,
            pos: if let Some(t) = &self.look { t.pos } else { 0 },
        });
    }

    fn parse_cdr(self: &mut Self, interner: &mut Interner) -> Result<Sexp, ParseError> {
        match &self.look {
            None => self.make_error(ParseErrorType::UnexpectedEOF),
            Some(t) => match t.r#type {
                TokenType::RPAREN => Ok(Sexp::Nil),
                TokenType::DOT => {
                    self.advance()?;
                    let form = self.next_form(interner)?;
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
                },
                _ => {
                    let form = self.next_form(interner)?;
                    if let Some(car) = form {
                        Ok(Sexp::Pair(Box::new(car), Box::new(self.parse_cdr(interner)?)))
                    } else {
                        self.make_error(ParseErrorType::UnexpectedEOF)
                    }
                }
            }
        }
    }

    fn parse_list(self: &mut Self, interner: &mut Interner) -> Result<Sexp, ParseError> {
        self.advance()?; // skip the '('
        let first = if let Some(s) = self.next_form(interner)? {
            s
        } else {
            self.make_error(ParseErrorType::UnexpectedEOF)?
        };
        let result = Sexp::Pair(Box::new(first), Box::new(self.parse_cdr(interner)?));
        self.advance()?; // skip the ')'
        Ok(result)
    }

    pub fn next_form(self: &mut Self, interner: &mut Interner) -> Result<Option<Sexp>, ParseError> {
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
                    Ok(Some(Sexp::Integer(i)))
                }
                TokenType::SYMBOL => {
                    let result = Sexp::Symbol(interner.intern(t.val.clone()));
                    self.advance()?;
                    Ok(Some(result))
                },
                TokenType::STRING => {
                    let result = Sexp::String(t.val.clone());
                    self.advance()?;
                    Ok(Some(result))
                },
                TokenType::LPAREN => {
                    Ok(Some(self.parse_list(interner)?))
                },
                TokenType::RPAREN => Err(ParseError {
                    r#type: ParseErrorType::UnexpectedRPAREN,
                    pos: t.pos,
                }),
                _ => todo!(),
            },
        }
    }
}
