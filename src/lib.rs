#![doc = include_str!("../README.md")]

mod char;
mod comment;
mod float;
mod int;
mod keyword;
mod lexer;
mod string;

pub use float::Float;
pub use int::Int;
pub use keyword::Keyword;
pub use lexer::Token;

/// C lexeme
#[derive(Debug, Clone)]
pub struct Lexeme<'l> {
    /// Token kind
    pub token: Token,
    /* TODO:
    /// Position in source code
    pub location: Location,
    */
    pub span: core::ops::Range<usize>,
    /// String slice
    pub slice: &'l str,
}

impl<'l> core::ops::Deref for Lexeme<'l> {
    type Target = Token;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

impl<'l> AsRef<str> for Lexeme<'l> {
    fn as_ref(&self) -> &'l str {
        self.slice
    }
}

impl<'l> Lexeme<'l> {
    /// Extract keyword
    pub fn keyword(&self) -> Option<Keyword> {
        if self.token == Token::Identifier {
            self.slice.parse().ok()
        } else {
            None
        }
    }

    /// Extract text from comment
    pub fn comment(&self) -> Option<String> {
        if self.token == Token::Comment {
            comment::extract(self.slice)
        } else {
            None
        }
    }

    /// Extract text from character literal
    pub fn char(&self) -> Option<char> {
        if self.token == Token::Char {
            char::extract(self.slice)
        } else {
            None
        }
    }

    /// Extract text from string literal
    pub fn string(&self) -> Option<String> {
        if self.token == Token::String {
            string::extract(self.slice)
        } else {
            None
        }
    }

    /// Extract number from integer literal
    pub fn int<T: Int>(&self) -> Option<T> {
        if self.token == Token::Int {
            int::extract(self.slice)
        } else {
            None
        }
    }

    /// Extract number from floating-point literal
    pub fn float<T: Float>(&self) -> Option<T> {
        if self.token == Token::Float {
            float::extract(self.slice)
        } else {
            None
        }
    }
}

/* TODO:
/// C token location
#[derive(Debug, Clone)]
pub struct Location {
    /// Source code position
    pub point: u32,
    /// Source code line
    pub line: u32,
    /// Source code column
    pub column: u32,
}
*/

/// C Lexer
pub struct Lexer<'l> {
    inner: logos::Lexer<'l, Token>,
}

impl<'l> From<&'l str> for Lexer<'l> {
    fn from(s: &'l str) -> Self {
        Self {
            inner: logos::Lexer::new(s),
        }
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Lexeme<'l>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|token| {
            let span = self.inner.span();
            let slice = self.inner.slice();
            Lexeme { token, span, slice }
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
