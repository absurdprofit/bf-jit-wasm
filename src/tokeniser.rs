use std::fmt::Display;

#[derive(PartialEq)]
pub struct SourceMapping {
    line: usize,
    column: usize,
    file_path: String,
}

impl Display for SourceMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file_path, self.line, self.column)
    }
}

#[derive(PartialEq)]
pub enum Token {
    Right,
    Left,
    Increment,
    Decrement,
    Input,
    Output,
    RightJump(SourceMapping),
    LeftJump(SourceMapping),
}

pub fn tokenise(source: &String, path: &String) -> impl Iterator<Item = Token> {
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
            b'>' => Some(Token::Right),
            b'<' => Some(Token::Left),
            b'+' => Some(Token::Increment),
            b'-' => Some(Token::Decrement),
            b'.' => Some(Token::Output),
            b',' => Some(Token::Input),
            b'[' => Some(Token::RightJump(SourceMapping {
                column,
                line,
                file_path: path.clone(),
            })),
            b']' => Some(Token::LeftJump(SourceMapping {
                column,
                line,
                file_path: path.clone(),
            })),
            _ => None,
        }
    })
}
