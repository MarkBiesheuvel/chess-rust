// External imports
use std::str;
// Absolute imports within crate
use crate::parser::ParseError;

// Wrapper for whitespace iterator that returns Results instead of Options
pub struct FieldIterator<'a> {
    iter: str::SplitWhitespace<'a>,
}
impl FieldIterator<'_> {
    pub fn new(specification: &str) -> FieldIterator {
        FieldIterator {
            iter: specification.split_whitespace(),
        }
    }

    pub fn next(&mut self) -> Result<&str, ParseError> {
        match self.iter.next() {
            Some(field) => Ok(field),
            None => Err(ParseError::UnexpectedEnd),
        }
    }
}
