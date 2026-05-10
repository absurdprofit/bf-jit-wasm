#[derive(PartialEq)]
pub enum Token {
    Right,
    Left,
    Increment,
    Decrement,
    Input,
    Output,
    RightJump,
    LeftJump,
}

// TODO: Create structs for Token variants. Add line and column number info to token struct.
pub fn tokenise(source: &String) -> impl Iterator<Item = Token> {
    source.as_bytes().iter().filter_map(|c| match c {
        b'>' => Some(Token::Right),
        b'<' => Some(Token::Left),
        b'+' => Some(Token::Increment),
        b'-' => Some(Token::Decrement),
        b'.' => Some(Token::Output),
        b',' => Some(Token::Input),
        b'[' => Some(Token::RightJump),
        b']' => Some(Token::LeftJump),
        _ => None,
    })
}
