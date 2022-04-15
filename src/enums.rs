use crate::postfix_errors::PostfixError;


#[derive(Clone,Debug)]
pub struct Alphabet (char);

#[derive(Clone,Debug)]
pub struct Digit (u8);

impl From<char> for Digit {
    fn from(c: char) -> Digit {Digit(c as u8 - '0' as u8)}
}

#[derive(Clone,Debug)]
pub enum NameCharacter {
    ALPHABET(Alphabet),
    DIGIT(Digit),
}

#[derive(Debug)]
pub enum BreakingCharacter {
    LEFT,
    RIGHT,
    SPACE,
}

#[derive(Debug)]
pub enum Character {
    BREAK(BreakingCharacter),
    NAME(NameCharacter),
}

pub trait ToCharacter { fn to_character(self) -> Result<Character, PostfixError>; }

impl ToCharacter for char {
    fn to_character(self) -> Result<Character, PostfixError> {
        match self {
            '(' => Ok(Character::BREAK(BreakingCharacter::LEFT)),
            ')' => Ok(Character::BREAK(BreakingCharacter::RIGHT)),
            ' ' => Ok(Character::BREAK(BreakingCharacter::SPACE)),
            'a'..='z' | 'A'..='Z' => Ok(Character::NAME(NameCharacter::ALPHABET(Alphabet(self)))),
            '0'..='9' => Ok(Character::NAME(NameCharacter::DIGIT(self.into()))),
            c => Err(PostfixError::InvalidCharacterError(c))
        }
    }
}


#[derive(Debug,Copy,Clone)]
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

#[derive(Debug,Copy,Clone)]
pub enum PostfixFunction {
    ARITHMETIC(PostfixArithmetic),
    EXEC,
    NGET,
    POP,
    SEL,
    SWAP,
}

#[derive(Debug)]
pub enum PostfixKeyword {
    FUNCTION(PostfixFunction),
    HEAD
}

#[derive(Debug)]
pub struct ToBeFunction {
    head: Alphabet,
    tail: Vec<NameCharacter>,
}

impl ToBeFunction {
    fn to_string(self) -> String {
        use NameCharacter::*;
        let Alphabet(head) = self.head;
        let tail: String = self.tail.into_iter().map(|nc| match nc {
            ALPHABET(Alphabet(c)) => c,
            DIGIT(Digit(d)) => d.into()
        }).collect();
        let mut final_string = head.to_string();
        final_string.push_str(&tail);
        final_string
    }
}

impl TryFrom<ToBeFunction> for PostfixKeyword {
    type Error = PostfixError;
    fn try_from(tbf: ToBeFunction) -> Result<PostfixKeyword, PostfixError> {
        use NameCharacter::*;
        use PostfixFunction::*;
        use PostfixKeyword::*;
        use PostfixArithmetic::*;
        match (tbf.head, tbf.tail.as_slice()) {
            (Alphabet('e'), [ALPHABET(Alphabet('q'))]) => Ok(FUNCTION(ARITHMETIC(EQ))),
            (Alphabet('l'), [ALPHABET(Alphabet('t'))]) => Ok(FUNCTION(ARITHMETIC(LT))),
            (Alphabet('g'), [ALPHABET(Alphabet('t'))]) => Ok(FUNCTION(ARITHMETIC(GT))),

            (Alphabet('a'), [ALPHABET(Alphabet('d')), ALPHABET(Alphabet('d'))]) => Ok(FUNCTION(ARITHMETIC(ADD))),
            (Alphabet('d'), [ALPHABET(Alphabet('i')), ALPHABET(Alphabet('v'))]) => Ok(FUNCTION(ARITHMETIC(DIV))),
            (Alphabet('m'), [ALPHABET(Alphabet('u')), ALPHABET(Alphabet('l'))]) => Ok(FUNCTION(ARITHMETIC(MUL))),
            (Alphabet('r'), [ALPHABET(Alphabet('e')), ALPHABET(Alphabet('m'))]) => Ok(FUNCTION(ARITHMETIC(REM))),
            (Alphabet('s'), [ALPHABET(Alphabet('u')), ALPHABET(Alphabet('b'))]) => Ok(FUNCTION(ARITHMETIC(SUB))),

            (Alphabet('p'), [ALPHABET(Alphabet('o')), ALPHABET(Alphabet('p'))]) => Ok(FUNCTION(POP)),
            (Alphabet('s'), [ALPHABET(Alphabet('e')), ALPHABET(Alphabet('l'))]) => Ok(FUNCTION(SEL)),

            (Alphabet('e'), [ALPHABET(Alphabet('x')), ALPHABET(Alphabet('e')), ALPHABET(Alphabet('c'))]) => Ok(FUNCTION(EXEC)),
            (Alphabet('n'), [ALPHABET(Alphabet('g')), ALPHABET(Alphabet('e')), ALPHABET(Alphabet('t'))]) => Ok(FUNCTION(NGET)),
            (Alphabet('s'), [ALPHABET(Alphabet('w')), ALPHABET(Alphabet('a')), ALPHABET(Alphabet('p'))]) => Ok(FUNCTION(SWAP)),
            
            (Alphabet('p'), [
                ALPHABET(Alphabet('o')), ALPHABET(Alphabet('s')), ALPHABET(Alphabet('t')), 
                ALPHABET(Alphabet('f')), ALPHABET(Alphabet('i')), ALPHABET(Alphabet('x'))
            ]) => Ok(HEAD),

            (head, tail_slice) => Err(PostfixError::InvalidFunctionNameError(ToBeFunction{ head, tail: tail_slice.into()}.to_string()))
        }
    }
}

#[derive(Debug)]
pub enum ToBeToken {
    TOBENUMBER(i32),
    TOBEWORD(ToBeFunction),
}

#[derive(Debug)]
pub enum Token {
    LEFT,
    RIGHT,
    NUMBER(i32),
    KEYWORD(PostfixKeyword),
}

impl ToBeToken {
    fn to_token(self) -> Result<Token, PostfixError> {
        Ok(match self {
            Self::TOBENUMBER(d) => Token::NUMBER(d),
            Self::TOBEWORD(w) => Token::KEYWORD(w.try_into()?)
        })
    }
}

fn parse_name_character_to_transformable_tobetoken(
    transformable: ToBeToken,
     character: NameCharacter
    ) -> Result<ToBeToken, PostfixError> {
        use ToBeToken::*;
        use NameCharacter::*;
        match transformable {
            TOBENUMBER(n) => match character {
                ALPHABET(a) => Err(PostfixError::NumberLiteralFollowedByAlphabetError(n, a)),
                DIGIT(Digit(d)) => Ok(TOBENUMBER(n * 10 + d as i32)),
            },
            TOBEWORD(mut tbf) => { tbf.tail.push(character); Ok(TOBEWORD(tbf)) },
        }
    }

fn parse_name_character_to_tobetoken(tobetoken_option: Option<ToBeToken>, character: NameCharacter) 
    -> Result<ToBeToken, PostfixError> {
        use ToBeToken::*;
        use NameCharacter::*;
        Ok( match tobetoken_option {
            None => match character {
                ALPHABET(alphabet) => TOBEWORD(ToBeFunction{ head: alphabet, tail: Vec::new()}),
                DIGIT(Digit(d)) => TOBENUMBER(d as i32),
            },
            Some(transformable) => parse_name_character_to_transformable_tobetoken(transformable, character)?
            
        } )
}

pub fn parse_one_character_with_tobetoken(tobetoken: Option<ToBeToken>, character: Character) 
-> Result<(Option<ToBeToken>, Vec<Token>), PostfixError> {
    use Character::*;
    use BreakingCharacter::*;
    let result = Ok(match character {
        BREAK(character) => (None, vec! [
            match tobetoken {
                Some(t) => Some(t.to_token()?),
                None => None,
            },
            match character {
                LEFT => Some(Token::LEFT),
                RIGHT => Some(Token::RIGHT),
                SPACE => None,
            }
        ].into_iter().flatten().collect()),
        NAME(character) => (Some(parse_name_character_to_tobetoken(
            tobetoken, character
        )?), vec! []),

    });
    result
}

#[derive(Debug,Clone)]
pub enum Command {
    INTEGER(i32),
    SPECIAL(PostfixFunction),
    EXECUTE(Vec<Command>),
}

#[derive(Debug)]
pub struct Program {pub paramsize: u32, pub commands: Vec<Command>}

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

fn execute_postfix_function(function: &PostfixFunction, remaining_commands: &mut Vec<Command>, stack: &mut Vec<Command>) -> Result<(), PostfixError> {
    use PostfixFunction::*;
    use Command::*;
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

pub fn execute_command(command: &Command, remaining_commands: &mut Vec<Command>, stack: &mut Vec<Command>) -> Result<(), PostfixError> {
    use Command::*;
    match command {

        SPECIAL(command) => { execute_postfix_function(command, remaining_commands, stack) }
        command => { stack.push(command.clone()); Ok(())}
    }
}