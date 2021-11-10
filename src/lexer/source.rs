use super::error::*;

use std::{ iter, vec };
/// A source code iterator. \
/// Code `position` and `line` are handled internally
pub struct Source {
    source: iter::Peekable<vec::IntoIter<char>>,
    pos: i32,
    line: i32,
}

impl<'s> Source {
    /// Creates a new source code iterator from a string
    pub fn new(src: &'s String) -> Source {
        Source {
            source: src.chars()
                .collect::<Vec<char>>()
                .into_iter()
                .peekable(),
            pos: 0,
            line: 1,
        }
    }

    /// Consumes and returns the next character in the iterator, or `TokenError` if there is none
    pub fn next(&mut self) -> Result<char, TokenError> {
        if let Some(c) = self.source.next() {
            if c == '\n' { 
                self.line += 1; 
                self.pos = 0;
            } 
            else {
                self.pos += 1;
            }

            Ok(c)
        } 
        else {
            Err(TokenError::UnexpectedEOF)
        }
    }

    /// Checks what the next character is in the source iterator without consuming
    pub fn peek(&mut self) -> char {
        if let Some(c) = self.source.peek() {
            *c
        } 
        else {
            '\0'
        }
    }

    /// A tuple of the current `(line, position)` in the source code
    pub fn lp(&self) -> (i32, i32) {
        (self.line, self.pos)
    }

    /// Checks if the source iterator has more characters left in it
    pub fn has_next(&self) -> bool {
        self.source.size_hint().0 > 0
    }

    /// Consumes the next character and returns an error if it is not the given expected character
    pub fn expect(&mut self, c: char) -> Result<(), TokenError> {
        let next = self.next()?;
        if next == c {
            Ok(())
        } 
        else {
            Err(TokenError::Expected(c, next, self.lp()))
        }
    }
}