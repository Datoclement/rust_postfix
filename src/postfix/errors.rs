use super::{
    lexer::{
        Alphabet, 
    },
    enums::{
        PostfixKeyword, 
        PostfixFunction, 
        PostfixCommand
    },
};

#[derive(Debug,PartialEq)]
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
    InvalidValueByNGETFunction { command: PostfixCommand },
    NonNumeralFinalState { command: PostfixCommand },
    EmptyStackFinalState,
    DivideByZero,
}
