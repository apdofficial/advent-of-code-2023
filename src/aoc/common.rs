use std::fmt;
use std::fmt::Formatter;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum PuzzleResult {
    Number(u32),
    Text(String),
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

impl<'a> std::iter::Sum<&'a u32> for PuzzleResult {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a u32>,
    {
        let mut result = 0;
        for v in iter {
            result += v;
        }
        PuzzleResult::Number(result)
    }
}
