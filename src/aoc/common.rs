use std::fmt;
use std::fmt::Formatter;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum PuzzleResult {
    Number(u64),
    Text(String)
}

impl fmt::Debug for PuzzleResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PuzzleResult::Number(value) => {
                write!(f, "PuzzleResult::Number({})", value)
            }
            PuzzleResult::Text(value) => {
                write!(f, "PuzzleResult::Text({})", value)
            }
        }
    }
}