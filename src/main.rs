use std::fs;

mod enums;
mod postfix_errors;

use enums::{ Program, ToBeToken, Token, parse_one_character_with_tobetoken, ToCharacter, PostfixKeyword, Command, execute_command};
use postfix_errors::PostfixError;

fn parse_code_to_tokens(code: &str) -> Result<Vec<Token>, PostfixError> {
    let mut tokens = Vec::new();
    code.chars().map(|c| c.to_character()).fold(
        Ok(None),
        | tobetoken: Result<Option<ToBeToken>, PostfixError>, character | {
            let (new_tobetoken, mut new_tokens) 
                = parse_one_character_with_tobetoken(tobetoken?, character?)?;
            tokens.append(&mut new_tokens);
            Ok(new_tobetoken)
        }
    ).and(Ok(tokens))
}

fn parse_tokens_to_program(tokens: Vec<Token>) -> Result<Program, PostfixError> {
    use Token::*;
    use Command::*;
    if let [LEFT, KEYWORD(PostfixKeyword::HEAD), NUMBER(paramsize), remaining_tokens @ .., RIGHT] = tokens.as_slice() {
        Ok(Program { paramsize: *paramsize as u32, commands: {
            let mut commands = Vec::new();
            remaining_tokens.iter().fold(
                Ok(Vec::new()),
                |stack, token| {
                    
                    let mut stack = stack?;
                    match token {
                        LEFT => { stack.push(Vec::new()); Ok(stack) },
                        RIGHT => match stack.pop() {
                            None => Err(PostfixError::UnmatchedRightParenthese),
                            Some(current) => { 
                                let execute = EXECUTE(current);
                                match stack.pop() {
                                None => commands.push(execute), 
                                Some(mut parent) => { parent.push(execute); stack.push(parent) }
                            }; Ok(stack) }
                        },
                        NUMBER(d) => {
                            let integer = INTEGER(*d);
                            match stack.pop() {
                            None => { commands.push(integer); Ok(stack) },
                            Some(mut current) => { current.push(integer); stack.push(current); Ok(stack) }
                        }},
                        KEYWORD(PostfixKeyword::HEAD) => Err(PostfixError::InvalidPostfixKeyword(PostfixKeyword::HEAD)),
                        KEYWORD(PostfixKeyword::FUNCTION(func)) => {
                            let keyword = SPECIAL(*func);
                            match stack.pop() {
                                None => { commands.push(keyword); Ok(stack) },
                                Some(mut current) => { current.push(keyword); stack.push(current); Ok(stack) }
                            }
                        }
                    }
                }
            )?; commands
        }})
    } else { Err(PostfixError::PostfixShouldBeginWithLeftPostfixAndEndWithRight) }
}

fn execute_program(program: &Program, arguments: &Vec<i32>) -> Result<i32, PostfixError> {
    if arguments.len() != program.paramsize as usize {
        Err(PostfixError::WrongNumberOfArguments { expected: program.paramsize as usize, actual: arguments.to_owned() })
    } else {
        let mut stack = arguments.iter().rev().map(|&n| Command::INTEGER(n)).collect();
        let owned_program_commands = program.commands.to_owned();
        let mut commands: Vec<Command> = owned_program_commands.iter().rev().map(|command| command.to_owned()).collect();
        loop {
            println! ("{:?} || {:?}", commands, stack);
            match commands.pop() {
                None => break,
                Some(command) => execute_command(&command, &mut commands, &mut stack)?
            }
        };
        match stack.as_slice() {
            [.., Command::INTEGER(n)] => Ok(*n),
            [.., command] => Err(PostfixError::NonNumeralFinalState { command: command.clone() }),
            [] => Err(PostfixError::EmptyStackFinalState), 
        }
        
    }
}

#[cfg(test)]
mod test_suite {


}

enum CodeType<'a> {
    FILE(&'a str),
    CODE(String),
}

fn main () {
    use CodeType::*;
    let testcases = vec![
        (FILE("code/postfix/ex5.postfix"), vec![]),
        (FILE("code/postfix/ex6.postfix"), vec![]),
        (FILE("code/postfix/ex7.postfix"), vec![]),
        (FILE("code/postfix/ex8.postfix"), vec![]),
        (FILE("code/postfix/ex9.postfix"), vec![]),
        (CODE("(postfix 2)".to_owned()), vec![3, 4]),
        (CODE("(postfix 2 swap)".to_owned()), vec![3, 4]),
        (CODE("(postfix 3 pop swap)".to_owned()), vec![3, 4, 5]),
        (CODE("(postfix 2 swap)".to_owned()), vec![3]),
        (CODE("(postfix 1 pop)".to_owned()), vec![4, 5]),
        (CODE("(postfix 1 4 sub)".to_owned()), vec![3]),
        (CODE("(postfix 1 4 add 5 mul 6 sub 7 div)".to_owned()), vec![3]),
        (CODE("(postfix 5 add mul sub swap div)".to_owned()), vec![7, 6, 5, 4, 3]),
        (CODE("(postfix 3 4000 swap pop add)".to_owned()), vec![300, 20, 1]),
        (CODE("(postfix 2 add 2 div)".to_owned()), vec![3, 7]),
        (CODE("(postfix 1 3 div)".to_owned()), vec![17]),
        (CODE("(postfix 1 3 rem)".to_owned()), vec![17]),
        (CODE("(postfix 1 4 lt)".to_owned()), vec![3]),
        (CODE("(postfix 1 4 lt)".to_owned()), vec![5]),
        (CODE("(postfix 1 4 lt 10 add)".to_owned()), vec![3]),
        (CODE("(postfix 1 4 mul add)".to_owned()), vec![3]),
        (CODE("(postfix 2 4 sub div)".to_owned()), vec![4, 5]),
        (CODE("(postfix 2 1 nget)".to_owned()), vec![4, 5]),
        (CODE("(postfix 2 2 nget)".to_owned()), vec![4, 5]),
        (CODE("(postfix 2 3 nget)".to_owned()), vec![4, 5]),
        (CODE("(postfix 2 0 nget)".to_owned()), vec![4, 5]),
        (CODE("(postfix 1 (2 mul) 1 nget)".to_owned()), vec![3]),
        (CODE("(postfix 1 1 nget mul)".to_owned()), vec![5]),
        (CODE("(postfix 4 4 nget 5 nget mul mul swap 4 nget mul add add)".to_owned()), vec![3, 4, 5, 2]),
        (CODE("(postfix 1 (2 mul) exec)".to_owned()), vec![7]),
        (CODE("(postfix 0 (0 swap sub) 7 swap exec)".to_owned()), vec![]),
        (CODE("(postfix 0 (2 mul))".to_owned()), vec![]),
        (CODE("(postfix 0 3 (2 mul) gt)".to_owned()), vec![]),
        (CODE("(postfix 0 3 exec)".to_owned()), vec![]),
        (CODE("(postfix 0 (7 swap exec) (0 swap sub) swap exec)".to_owned()), vec![]),
        (CODE("(postfix 2 (mul sub) (1 nget mul) 4 nget swap exec swap exec)".to_owned()), vec![-10, 2]),
        (CODE("(postfix 1 2 3 sel)".to_owned()), vec![1]),
        (CODE("(postfix 1 2 3 sel)".to_owned()), vec![0]),
        (CODE("(postfix 1 2 3 sel)".to_owned()), vec![17]),
        (CODE("(postfix 0 (2 mul) 3 4 sel)".to_owned()), vec![]),
        (CODE("(postfix 4 lt (add) (mul) sel exec)".to_owned()), vec![3, 4, 5, 6]),
        (CODE("(postfix 4 lt (add) (mul) sel exec)".to_owned()), vec![4, 3, 5, 6]),
        (CODE("(postfix 1 1 nget 0 lt (0 swap sub) () sel exec)".to_owned()), vec![-7]),
        (CODE("(postfix 1 1 nget 0 lt (0 swap sub) () sel exec)".to_owned()), vec![6]),

        // 2 x - 5
        (CODE("(postfix 1 ((3 nget swap exec) (2 mul swap exec) swap) (5 sub) swap exec exec)".to_owned()), vec![2]),

        // not
        (CODE("(postfix 1 0 1 sel)".to_owned()), vec![6]),
        (CODE("(postfix 1 0 1 sel)".to_owned()), vec![0]),
        (CODE("(postfix 1 0 1 sel)".to_owned()), vec![1]),

        // and
        (CODE("(postfix 2 (1 0 sel) (0) sel exec)".to_owned()), vec![6, 0]),
        (CODE("(postfix 2 (1 0 sel) (0) sel exec)".to_owned()), vec![0, 0]),
        (CODE("(postfix 2 (1 0 sel) (0) sel exec)".to_owned()), vec![6, 1]),
        (CODE("(postfix 2 (1 0 sel) (0) sel exec)".to_owned()), vec![0, 3]),

        // short-circuit and
        (CODE("(postfix 2 (1 nget) (0) sel exec)".to_owned()), vec![0, 3]),
        (CODE("(postfix 2 (1 nget) (0) sel exec)".to_owned()), vec![123, 3]),
        (CODE("(postfix 2 (1 nget) (0) sel exec)".to_owned()), vec![123, 0]),
        (CODE("(postfix 2 (1 nget) (0) sel exec)".to_owned()), vec![0, 0]),
    ];
    testcases.iter().map(|(testcase, arguments)|{
        let code = match testcase {
            FILE(filename) => fs::read_to_string(filename),
            CODE(code) => Ok(code.to_owned()),
        };
        match code {
            Ok(code) => {
                println! ("{:?}", code);
                match parse_code_to_tokens(&code) {
                    Err(error) =>  println! ("Error: {:?}", error),
                    Ok(tokens) => match parse_tokens_to_program(tokens) {
                        Err(error) => println! ("Error: {:?}", error),
                        Ok(program) => {
                            println! ("Final result: {:?}", program);
                            match execute_program(&program, &arguments) {
                                Err(error) => println! ("Error: {:?}", error),
                                Ok( value ) => println! ("Run Result: {:?}", value),
                            }
                        },
                    }
                }
            },
            Err(_) => panic! ("read file failure!")
        }
    }).collect()
}