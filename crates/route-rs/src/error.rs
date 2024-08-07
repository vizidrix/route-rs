use crate::lexer::LexerError;

#[derive(Clone, Debug, PartialEq)]
pub enum InsertError {
    AmbiguousParams,
    EmptyPath,
    InvalidPath(Option<usize>, String),
    TrailingSlash(usize),
    TrailingWildcardPath,
}

impl From<LexerError> for InsertError {
    fn from(src: LexerError) -> InsertError {
        match src {
           LexerError::InvalidPath(position, path) => InsertError::InvalidPath(position, path),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MatchError {
    NotFound,
    InvalidPath(Option<usize>, String),
}

impl From<LexerError> for MatchError {
    fn from(src: LexerError) -> MatchError {
        match src {
            LexerError::InvalidPath(position, path) => MatchError::InvalidPath(position, path),
        }
    }
}