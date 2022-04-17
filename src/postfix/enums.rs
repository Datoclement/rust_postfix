use super::errors::PostfixError;


#[derive(Debug,Copy,Clone,PartialEq)]
pub enum PostfixArithmetic {
    ADD,
    DIV,
    EQ,
    GT,
    LT,
    MUL,
    SUB,
    REM,
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum PostfixFunction {
    ARITHMETIC(PostfixArithmetic),
    EXEC,
    NGET,
    POP,
    SEL,
    SWAP,
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum PostfixKeyword {
    FUNCTION(PostfixFunction),
    HEAD
}


#[derive(Debug,Clone,PartialEq)]
pub enum PostfixCommand {
    INTEGER(i32),
    SPECIAL(PostfixFunction),
    EXECUTE(Vec<PostfixCommand>),
}

#[derive(Debug)]
pub struct PostfixProgram { pub paramsize: u32, pub commands: Vec<PostfixCommand> }

pub struct ProgramBuilder {
    paramsize: usize,
    commands: Vec<PostfixCommand>,
    stack: Vec<Vec<PostfixCommand>>,
}

impl ProgramBuilder {

    pub fn new(paramsize: usize) -> Self {
        Self {
            paramsize,
            commands: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn build(self: &Self) -> PostfixProgram {
        PostfixProgram { paramsize: self.paramsize as u32, commands: self.commands.to_owned() }
    }

    pub fn consume(mut self: Self, token: super::lexer::Token) -> Result<Self, PostfixError> {
        use super::lexer::Token::*;
        use PostfixCommand::*;
        match token {
            LEFT => { self.stack.push(Vec::new()); Ok(self) },
            RIGHT => match self.stack.pop() {
                None => Err(PostfixError::UnmatchedRightParenthese),
                Some(current) => { 
                    let execute = EXECUTE(current);
                    match self.stack.pop() {
                    None => self.commands.push(execute), 
                    Some(mut parent) => { parent.push(execute); self.stack.push(parent) }
                }; Ok(self) }
            },
            NUMBER(d) => {
                let integer = INTEGER(d);
                match self.stack.pop() {
                None => { self.commands.push(integer); Ok(self) },
                Some(mut current) => { current.push(integer); self.stack.push(current); Ok(self) }
            }},
            KEYWORD(PostfixKeyword::HEAD) => Err(PostfixError::InvalidPostfixKeyword(PostfixKeyword::HEAD)),
            KEYWORD(PostfixKeyword::FUNCTION(func)) => {
                let keyword = SPECIAL(func);
                match self.stack.pop() {
                    None => { self.commands.push(keyword); Ok(self) },
                    Some(mut current) => { current.push(keyword); self.stack.push(current); Ok(self) }
                }
            }
        }
    }
}

fn compute_arithmetic_function(arithmetic: &PostfixArithmetic, pre_operand: i32, post_operand: i32) -> i32 {
    use PostfixArithmetic::*;
    match arithmetic {
        ADD => pre_operand + post_operand,
        SUB => pre_operand - post_operand,
        MUL => pre_operand * post_operand,
        DIV => pre_operand / post_operand,
        REM => pre_operand % post_operand,
        EQ => if pre_operand == post_operand {1} else {0},
        GT => if pre_operand > post_operand {1} else {0},
        LT => if pre_operand < post_operand {1} else {0},
    }
}

fn execute_postfix_function(function: &PostfixFunction, remaining_commands: &mut Vec<PostfixCommand>, stack: &mut Vec<PostfixCommand>) -> Result<(), PostfixError> {
    use PostfixFunction::*;
    use PostfixCommand::*;
    match function {
        ARITHMETIC(operator) => match (stack.pop(), stack.pop()) {
            (None, _) | (_, None) => Err(PostfixError::WrongNumberOfFunctionArguments { function: ARITHMETIC(*operator), expected_number_of_arguments: 2 }),
            (Some(INTEGER(post_operand)), Some(INTEGER(pre_operand))) => { 
                match (operator, post_operand) {
                    (PostfixArithmetic::DIV, 0) => Err(PostfixError::DivideByZero),
                    (operator, post_operand) => {
                        stack.push(INTEGER(compute_arithmetic_function(operator, pre_operand, post_operand))); Ok(())
                    },
                }
            }
            _ => Err(PostfixError::WrongTypeOfFunctionArguments { function: ARITHMETIC(*operator) })
        }
       
        EXEC => match stack.pop() {
            None => Err(PostfixError::WrongNumberOfFunctionArguments { function: EXEC, expected_number_of_arguments: 1 }),
            Some(EXECUTE(commands)) => { commands.iter().rev().fold((), |(), command| remaining_commands.push(command.to_owned())); Ok(()) },
            _ => Err(PostfixError::WrongTypeOfFunctionArguments { function: EXEC })
        }
       
        NGET => match stack.pop() {
            Some(INTEGER(n)) => {
                match usize::try_from(n) {
                    Ok(n) if n > 0 => match (stack.len() as i32 - n as i32) >= 0  {
                        true => match stack.get(stack.len() - n) {
                            Some(INTEGER(v)) => { let v = v.to_owned(); stack.push(INTEGER(v)); Ok(()) },
                            Some(command) => Err(PostfixError::InvalidValueByNGETFunction { command: command.clone() }),
                            _ => Err(PostfixError::IndexOutOfRangeByNGETFunction { index: n as i32, min: 1, max: stack.len() }),
                        },
                        false => Err(PostfixError::IndexOutOfRangeByNGETFunction { index: n as i32, min: 1, max: stack.len() })
                    }
                    _ => { Err(PostfixError::IndexOutOfRangeByNGETFunction { index: n, min: 1, max: stack.len() }) }
                }
                
            },
            Some(_) => Err(PostfixError::WrongTypeOfFunctionArguments { function: NGET }),
            _ => Err(PostfixError::WrongNumberOfFunctionArguments { function: NGET, expected_number_of_arguments: 1 }),
        }

        POP => match stack.pop() {
            Some(_) => Ok(()),
            _ => Err(PostfixError::WrongNumberOfFunctionArguments { function: POP, expected_number_of_arguments: 1 }),
        },
        
        SEL => match (stack.pop(), stack.pop(), stack.pop()) {
            (None, _, _) | (_, None, _) | (_, _, None) => Err(PostfixError::WrongNumberOfFunctionArguments { function: SEL, expected_number_of_arguments: 3 }),
            (Some(command), _, Some(INTEGER(0))) => { stack.push(command); Ok(()) },
            (_, Some(command), Some(INTEGER(_))) => { stack.push(command); Ok(()) },
            _ => Err(PostfixError::WrongTypeOfFunctionArguments { function: SEL }),
        }
        
        SWAP => match (stack.pop(), stack.pop()) {
            (Some(command1), Some(command2)) => { stack.push(command1); stack.push(command2); Ok(()) },
            _ => Err(PostfixError::WrongNumberOfFunctionArguments { function: SWAP, expected_number_of_arguments: 2 }),
        },
    }
}

pub fn execute_command(command: &PostfixCommand, remaining_commands: &mut Vec<PostfixCommand>, stack: &mut Vec<PostfixCommand>) -> Result<(), PostfixError> {
    use PostfixCommand::*;
    match command {

        SPECIAL(command) => { execute_postfix_function(command, remaining_commands, stack) }
        command => { stack.push(command.clone()); Ok(())}
    }
}