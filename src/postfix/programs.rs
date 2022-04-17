use super::{
    lexer::{
        parse_code_to_tokens,
        Token,
    },
    errors::PostfixError,
    enums::{
        execute_command,
        PostfixKeyword,
        PostfixProgram,
        ProgramBuilder,
        PostfixCommand,
    }
};



pub fn parse_tokens_to_program(tokens: Vec<Token>) -> Result<PostfixProgram, PostfixError> {
    use Token::*;
    if let [LEFT, KEYWORD(PostfixKeyword::HEAD), NUMBER(paramsize), remaining_tokens @ .., RIGHT] = tokens.as_slice() {
        Ok(remaining_tokens.iter().fold(
            Ok(ProgramBuilder::new(*paramsize as usize)),
            |builder, token| builder?.consume(*token)
        )?.build())
    } else { Err(PostfixError::PostfixShouldBeginWithLeftPostfixAndEndWithRight) }
}

fn execute_program(program: &PostfixProgram, arguments: &Vec<i32>) -> Result<i32, PostfixError> {
    if arguments.len() != program.paramsize as usize {
        Err(PostfixError::WrongNumberOfArguments { expected: program.paramsize as usize, actual: arguments.to_owned() })
    } else {
        let mut stack = arguments.iter().rev().map(|&n| PostfixCommand::INTEGER(n)).collect();
        let owned_program_commands = program.commands.to_owned();
        let mut commands: Vec<PostfixCommand> = owned_program_commands.iter().rev().map(|command| command.to_owned()).collect();
        loop {
            println! ("{:?} || {:?}", commands, stack);
            match commands.pop() {
                None => break,
                Some(command) => execute_command(&command, &mut commands, &mut stack)?
            }
        };
        match stack.as_slice() {
            [.., PostfixCommand::INTEGER(n)] => Ok(*n),
            [.., command] => Err(PostfixError::NonNumeralFinalState { command: command.clone() }),
            [] => Err(PostfixError::EmptyStackFinalState), 
        }
        
    }
}

pub fn compile_and_run(code: &str, arguments: &Vec<i32>) -> Result<i32, PostfixError> {
    println! ("{:?}", code);
    let tokens = parse_code_to_tokens(code)?;
    let program = parse_tokens_to_program(tokens)?;
    let output = execute_program(&program, arguments)?;
    Ok(output)
}