
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
pub struct Token {
    pub r#type: TokenType,
    pub val: String,
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

    fn peek(self: &Self) -> Option<u8> {
        if self.is_eof() {
            None
        } else {
            Some(self.source.as_bytes()[self.pos])
        }
    }

    fn advance(self: &mut Self) {
        if self.pos < self.source.len() {
            self.pos += 1;
        }
    }

    fn skip_space(self: &mut Self) {
        loop {
            match self.peek() {
                Some(c) => if !c.is_ascii_whitespace() { break },
                None=> break,
            }
            self.advance();
        }
    }

    fn make_token(self: &Self, r#type: TokenType) -> Token {
        Token {
            r#type: r#type,
            val: String::from(&self.source[self.start..self.pos]),
            pos: self.start,
        }
    }

    fn integer(self: &mut Self) -> Result<Option<Token>, LexerError> {
        loop {
            match self.peek() {
                Some(c) => if !c.is_ascii_digit() { break },
                None => break
            };
            self.advance();
        }
        Ok(Some(self.make_token(TokenType::INTEGER)))
    }

    fn string(self: &mut Self) -> Result<Option<Token>, LexerError> {
        self.advance(); // skip the '"'
        loop {
            let r = self.peek();
            match self.peek() {
                Some(b'"') => break,
                Some(_) => {}
                None =>  return Err(LexerError {
                            r#type: LexerErrorType::StringNotTerminated,
                            pos: self.start,
                        }),
            }
            self.advance();
        }
        self.advance(); // skip the '"'
        Ok(Some(self.make_token(TokenType::STRING)))
    }

    fn symbol(self: &mut Self) -> Result<Option<Token>, LexerError> {
        loop {
            match self.peek() {
                Some(c) => {
                    if c == b'(' || c == b')' || c.is_ascii_whitespace() {
                        break;
                    }
                }
                None => break
            }
            self.advance();
        }
        Ok(Some(self.make_token(TokenType::SYMBOL)))
    }

    pub fn next_token(self: &mut Self) -> Result<Option<Token>, LexerError> {
        self.skip_space();
        self.start = self.pos;
        match self.peek() {
            None => Ok(None),
            Some(b'(') => {
                self.advance();
                Ok(Some(self.make_token(TokenType::LPAREN)))
            }
            Some(b')') => {
                self.advance();
                Ok(Some(self.make_token(TokenType::RPAREN)))
            }
            Some(b'.') => {
                self.advance();
                Ok(Some(self.make_token(TokenType::DOT)))
            }
            Some(b'0'..=b'9') => self.integer(),
            Some(b'"') => self.string(),
            _ => self.symbol(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum LexerErrorType {
    StringNotTerminated,
}

#[derive(Debug)]
pub struct LexerError {
    pub r#type: LexerErrorType,
    pub pos: usize,
}

