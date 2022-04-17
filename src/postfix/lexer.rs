use super::{
    enums::{ 
        PostfixKeyword, 
        PostfixFunction, 
        PostfixArithmetic,
    },
    errors::PostfixError,
};


#[derive(Clone,Debug,PartialEq)]
pub struct Alphabet (char);

#[derive(Clone,Debug)]
pub struct Digit (pub u8);

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

// routing type: control creation of all its managed types (BreakingCharacter, NameCharacter)
impl TryFrom<char> for Character {
    type Error = PostfixError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Character::BREAK(BreakingCharacter::LEFT)),
            ')' => Ok(Character::BREAK(BreakingCharacter::RIGHT)),
            ' ' => Ok(Character::BREAK(BreakingCharacter::SPACE)),
            'a'..='z' | 'A'..='Z' => Ok(Character::NAME(NameCharacter::ALPHABET(Alphabet(c)))),
            '0'..='9' => Ok(Character::NAME(NameCharacter::DIGIT(Digit(c as u8 - '0' as u8)))),
            c => Err(PostfixError::InvalidCharacterError(c))
        }
    }
}

#[derive(Debug)]
pub struct FunctionBuilder {
    pub head: Alphabet,
    pub tail: Vec<NameCharacter>,
}

impl FunctionBuilder {
    pub fn to_string(self) -> String {
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

#[derive(Debug)]
pub enum TokenBuilder {
    TOBENUMBER(i32),
    TOBEWORD(FunctionBuilder),
}

impl TryFrom<FunctionBuilder> for PostfixKeyword {
    type Error = PostfixError;
    fn try_from(tbf: FunctionBuilder) -> Result<PostfixKeyword, PostfixError> {
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

            (head, tail_slice) => Err(PostfixError::InvalidFunctionNameError(FunctionBuilder{ head, tail: tail_slice.into()}.to_string()))
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub enum Token {
    LEFT,
    RIGHT,
    NUMBER(i32),
    KEYWORD(PostfixKeyword),
}

impl TryFrom<TokenBuilder> for Token {
    type Error = PostfixError;
    fn try_from(tbt: TokenBuilder) -> Result<Self, Self::Error> {
        use TokenBuilder::*;
        Ok(match tbt {
            TOBENUMBER(d) => Self::NUMBER(d),
            TOBEWORD(w) => Self::KEYWORD(w.try_into()?)
        })
    }
}

impl TokenBuilder {
    fn consume(self: Self, character: NameCharacter) -> Result<Self, PostfixError> {
        use TokenBuilder::*;
        use NameCharacter::*;
        match self {
            TOBENUMBER(n) => match character {
                ALPHABET(a) => Err(PostfixError::NumberLiteralFollowedByAlphabetError(n, a)),
                DIGIT(Digit(d)) => Ok(TOBENUMBER(n * 10 + d as i32)),
            },
            TOBEWORD(mut tbf) => { tbf.tail.push(character); Ok(TOBEWORD(tbf)) },
        }
    }
}

fn parse_name_character_to_tobetoken(tobetoken_option: Option<TokenBuilder>, character: NameCharacter) -> Result<TokenBuilder, PostfixError> {
    use TokenBuilder::*;
    use NameCharacter::*;
    Ok( match tobetoken_option {
        None => match character {
            ALPHABET(alphabet) => TOBEWORD(FunctionBuilder{ head: alphabet, tail: Vec::new()}),
            DIGIT(Digit(d)) => TOBENUMBER(d as i32),
        },
        Some(transformable) => transformable.consume(character)?
        
    } )
}

fn parse_one_character_with_tobetoken(tobetoken: Option<TokenBuilder>, character: Character) 
-> Result<(Option<TokenBuilder>, Vec<Token>), PostfixError> {
    use Character::*;
    use BreakingCharacter::*;
    let result = Ok(match character {
        BREAK(character) => (None, vec! [
            match tobetoken {
                Some(t) => Some(t.try_into()?),
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

pub fn parse_code_to_tokens(code: &str) -> Result<Vec<Token>, PostfixError> {
    let mut tokens = Vec::new();
    code.chars().map(|c| c.try_into()).fold(
        Ok(None),
        | tobetoken: Result<Option<TokenBuilder>, PostfixError>, character | {
            let (new_tobetoken, mut new_tokens) 
                = parse_one_character_with_tobetoken(tobetoken?, character?)?;
            tokens.append(&mut new_tokens);
            Ok(new_tobetoken)
        }
    ).and(Ok(tokens))
}