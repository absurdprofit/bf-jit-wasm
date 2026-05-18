use std::fmt::Display;

#[derive(PartialEq, Clone, Debug)]
pub struct SourceMapping {
    line: usize,
    column: usize,
    file_path: String,
}

impl SourceMapping {
    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }
}

impl Display for SourceMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file_path, self.line, self.column)
    }
}

#[derive(Clone)]
pub enum Token {
    Right(SourceMapping),
    Left(SourceMapping),
    Increment,
    Decrement,
    Input,
    Output,
    RightJump(SourceMapping),
    LeftJump(SourceMapping),
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        use Token::*;

        matches!(
            (self, other),
            (Right(_), Right(_))
                | (Left(_), Left(_))
                | (RightJump(_), RightJump(_))
                | (LeftJump(_), LeftJump(_))
                | (Increment, Increment)
                | (Decrement, Decrement)
                | (Input, Input)
                | (Output, Output)
        )
    }
}

pub fn tokenise(source: &str, path: &str) -> impl Iterator<Item = Token> {
    let mut line = 1;
    let mut column = 0;
    source.as_bytes().iter().filter_map(move |c| {
        if *c == b'\n' {
            line += 1;
            column = 0;
        } else {
            column += 1;
        }
        match c {
            b'>' => Some(Token::Right(SourceMapping {
                column,
                line,
                file_path: String::from(path),
            })),
            b'<' => Some(Token::Left(SourceMapping {
                column,
                line,
                file_path: String::from(path),
            })),
            b'+' => Some(Token::Increment),
            b'-' => Some(Token::Decrement),
            b'.' => Some(Token::Output),
            b',' => Some(Token::Input),
            b'[' => Some(Token::RightJump(SourceMapping {
                column,
                line,
                file_path: String::from(path),
            })),
            b']' => Some(Token::LeftJump(SourceMapping {
                column,
                line,
                file_path: String::from(path),
            })),
            _ => None,
        }
    })
}
