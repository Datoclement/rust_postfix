use crate::enums::{Alphabet, PostfixKeyword, PostfixFunction, Command};

#[derive(Debug)]
pub enum PostfixError {
    InvalidCharacterError(char),
    NumberLiteralFollowedByAlphabetError(i32, Alphabet),
    InvalidFunctionNameError(String),
    PostfixShouldBeginWithLeftPostfixAndEndWithRight,
    UnmatchedRightParenthese,
    InvalidPostfixKeyword(PostfixKeyword),
    WrongNumberOfArguments{ expected: usize, actual: Vec<i32> },
    WrongNumberOfFunctionArguments{ function: PostfixFunction, expected_number_of_arguments: usize },
    WrongTypeOfFunctionArguments{ function: PostfixFunction },
    IndexOutOfRangeByNGETFunction { index: i32, min: usize, max: usize },
    InvalidValueByNGETFunction { command: Command },
    NonNumeralFinalState { command: Command },
    EmptyStackFinalState,
    DivideByZero,
}
