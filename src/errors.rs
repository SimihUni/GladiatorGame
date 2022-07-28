use std::io;

#[derive(Debug)]
pub enum GameError{
    IoError(io::Error),
    ParseError(String),
    GameOver,
    IncorrectMenuValue,
}
