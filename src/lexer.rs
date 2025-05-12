#[derive(Debug)]
pub enum TokenType {
    LPAREN,
    RPAREN,
    DOT,
    INTEGER,
    REAL,
    STRING,
    SYMBOL,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub r#type: TokenType,
    pub val: &'a [u8],
    pub pos: usize,
}
pub struct Lexer<'a> {
    source: &'a String,
    start: usize,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source: source,
            start: 0,
            pos: 0,
        }
    }

    fn is_eof(self: &Self) -> bool {
        self.pos >= self.source.len()
    }

    fn peek(self: &Self) -> Result<u8, LexerError> {
        if self.is_eof() {
            Err(LexerError {
                r#type: LexerErrorType::Eof,
                pos: self.pos,
            })
        } else {
            return Ok(self.source.as_bytes()[self.pos]);
        }
    }

    fn advance(self: &mut Self) {
        if self.pos < self.source.len() {
            self.pos += 1;
        }
    }

    fn skip_space(self: &mut Self) {
        loop {
            let r = self.peek();
            match r {
                Ok(c) => if !c.is_ascii_whitespace() { break },
                Err(e) => break,
            }
            self.advance();
        }
    }

    fn make_token(self: &Self, r#type: TokenType) -> Token<'a> {
        Token {
            r#type: r#type,
            val: &self.source.as_bytes()[self.start..self.pos],
            pos: self.start,
        }
    }

    fn integer(self: &mut Self) -> Result<Token<'a>, LexerError> {
        loop {
            let r = self.peek();
            match r {
                Ok(c) => if !c.is_ascii_digit() { break },
                Err(e) => {
                    if e.r#type == LexerErrorType::Eof {
                        break;
                    } else {
                        return Err(e);
                    }
                }
            };
            self.advance();
        }
        Ok(self.make_token(TokenType::INTEGER))
    }

    fn string(self: &mut Self) -> Result<Token<'a>, LexerError> {
        self.advance(); // skip the '"'
        loop {
            let r = self.peek();
            match r {
                Ok(b'"') => break,
                Ok(_) => {}
                Err(e) => match e.r#type {
                    LexerErrorType::Eof => {
                        return Err(LexerError {
                            r#type: LexerErrorType::StringNotTerminated,
                            pos: self.start,
                        });
                    }
                    LexerErrorType::StringNotTerminated => unreachable!(),
                },
            }
            self.advance();
        }
        self.advance(); // skip the '"'
        Ok(self.make_token(TokenType::STRING))
    }

    fn symbol(self: &mut Self) -> Result<Token<'a>, LexerError> {
        loop {
            let r = self.peek();
            match r {
                Ok(c) => {
                    if c == b'(' || c == b')' || c.is_ascii_whitespace() {
                        break;
                    }
                }
                Err(e) => match e.r#type {
                    LexerErrorType::Eof => break,
                    LexerErrorType::StringNotTerminated => unreachable!(),
                },
            }
            self.advance();
        }
        Ok(self.make_token(TokenType::SYMBOL))
    }

    pub fn next_token(self: &mut Self) -> Result<Token<'a>, LexerError> {
        self.skip_space();
        self.start = self.pos;
        let c = self.peek()?;
        match c {
            b'(' => {
                self.advance();
                Ok(self.make_token(TokenType::LPAREN))
            }
            b')' => {
                self.advance();
                Ok(self.make_token(TokenType::RPAREN))
            }
            b'.' => {
                self.advance();
                Ok(self.make_token(TokenType::DOT))
            }
            b'0'..=b'9' => self.integer(),
            b'"' => self.string(),
            _ => self.symbol(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum LexerErrorType {
    Eof,
    StringNotTerminated,
}

#[derive(Debug)]
pub struct LexerError {
    r#type: LexerErrorType,
    pos: usize,
}

impl <'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(x) => Some(Ok(x)),
            Err(e) => match e.r#type {
                LexerErrorType::Eof => None,
                _ => Some(Err(e))
            }
        }
    }
}

