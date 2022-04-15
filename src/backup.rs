// use crate::postfix_errors::PostfixError;
// use strum_macros::EnumString;

// #[derive(Clone)]
// pub struct Alphabet (char);

// #[derive(Clone)]
// pub struct Digit (u8);

// impl From<char> for Digit {
//     fn from(c: char) -> Digit {Digit(c as u8 - '0' as u8)}
// }

// #[derive(Clone)]
// pub enum NameCharacter {
//     ALPHABET(Alphabet),
//     DIGIT(Digit),
// }

// pub enum BreakingCharacter {
//     LEFT,
//     RIGHT,
//     SPACE,
// }

// pub enum Character {
//     BREAK(BreakingCharacter),
//     NAME(NameCharacter),
// }

// pub trait ToCharacter { fn to_character(self) -> Result<Character, PostfixError>; }

// impl ToCharacter for char {
//     fn to_character(self) -> Result<Character, PostfixError> {
//         match self {
//             '(' => Ok(Character::BREAK(BreakingCharacter::LEFT)),
//             ')' => Ok(Character::BREAK(BreakingCharacter::RIGHT)),
//             ' ' => Ok(Character::BREAK(BreakingCharacter::SPACE)),
//             'a'..='z' | 'A'..='Z' => Ok(Character::NAME(NameCharacter::ALPHABET(Alphabet(self)))),
//             '0'..='9' => Ok(Character::NAME(NameCharacter::DIGIT(self.into()))),
//             c => Err(PostfixError::InvalidCharacterError(c))
//         }
//     }
// }



// #[derive(Debug,EnumString)]
// pub enum PostfixFunction {
//     ADD,
//     DIV,
//     EQ,
//     EXEC,
//     GT,
//     LT,
//     MUL,
//     NGET,
//     POP,
//     REM,
//     SEL,
//     SUB,
//     SWAP,
//     POSTFIX,
// }

// pub struct ToBeFunction {
//     head: Alphabet,
//     tail: Vec<NameCharacter>,
// }

// impl ToBeFunction {
//     fn to_string(self) -> String {
//         use NameCharacter::*;
//         let Alphabet(head) = self.head;
//         let tail: String = self.tail.into_iter().map(|nc| match nc {
//             ALPHABET(Alphabet(c)) => c,
//             DIGIT(Digit(d)) => d.into()
//         }).collect();
//         let mut final_string = head.to_string();
//         final_string.push_str(&tail);
//         final_string
//     }
// }

// impl TryFrom<ToBeFunction> for PostfixFunction {
//     type Error = PostfixError;
//     fn try_from(tbf: ToBeFunction) -> Result<PostfixFunction, PostfixError> {
//         use NameCharacter::*;
//         use PostfixFunction::*;
//         match (tbf.head, tbf.tail.as_slice()) {
//             (Alphabet('E'), [ALPHABET(Alphabet('Q'))]) => Ok(EQ),
//             (Alphabet('L'), [ALPHABET(Alphabet('T'))]) => Ok(LT),
//             (Alphabet('G'), [ALPHABET(Alphabet('T'))]) => Ok(GT),

//             (Alphabet('A'), [ALPHABET(Alphabet('D')), ALPHABET(Alphabet('D'))]) => Ok(ADD),
//             (Alphabet('D'), [ALPHABET(Alphabet('I')), ALPHABET(Alphabet('V'))]) => Ok(DIV),
//             (Alphabet('M'), [ALPHABET(Alphabet('U')), ALPHABET(Alphabet('L'))]) => Ok(MUL),
//             (Alphabet('P'), [ALPHABET(Alphabet('O')), ALPHABET(Alphabet('P'))]) => Ok(POP),
//             (Alphabet('R'), [ALPHABET(Alphabet('E')), ALPHABET(Alphabet('M'))]) => Ok(REM),
//             (Alphabet('S'), [ALPHABET(Alphabet('E')), ALPHABET(Alphabet('L'))]) => Ok(SEL),
//             (Alphabet('S'), [ALPHABET(Alphabet('U')), ALPHABET(Alphabet('B'))]) => Ok(SUB),

//             (Alphabet('E'), [ALPHABET(Alphabet('X')), ALPHABET(Alphabet('E')), ALPHABET(Alphabet('C'))]) => Ok(EXEC),
//             (Alphabet('N'), [ALPHABET(Alphabet('G')), ALPHABET(Alphabet('E')), ALPHABET(Alphabet('T'))]) => Ok(NGET),
//             (Alphabet('S'), [ALPHABET(Alphabet('W')), ALPHABET(Alphabet('A')), ALPHABET(Alphabet('P'))]) => Ok(SWAP),
            
//             (Alphabet('P'), [
//                 ALPHABET(Alphabet('O')), ALPHABET(Alphabet('S')), ALPHABET(Alphabet('T')), 
//                 ALPHABET(Alphabet('F')), ALPHABET(Alphabet('I')), ALPHABET(Alphabet('X'))
//             ]) => Ok(POSTFIX),

//             (head, tail_slice) => Err(PostfixError::InvalidFunctionNameError(ToBeFunction{ head, tail: tail_slice.into()}.to_string()))
//         }
//     }
// }

// pub enum TransformableToBeToken {
//     TOBENUMBER(i32),
//     TOBEWORD(ToBeFunction),
// }
// pub enum ToBeToken {
//     EMPTY,
//     TRANSFORMABLE(TransformableToBeToken),
// }

// pub enum Token {
//     LEFT,
//     RIGHT,
//     NUMBER(i32),
//     FUNCTION(PostfixFunction),
// }

// pub enum GeneralToken {
//     TOBETOKEN(ToBeToken),
//     TOKEN(Token)
// }

// impl TransformableToBeToken {
//     fn to_token(self) -> Result<Token, PostfixError> {
//         Ok(match self {
//             Self::TOBENUMBER(d) => Token::NUMBER(d),
//             Self::TOBEWORD(w) => Token::FUNCTION(w.try_into()?)
//         })
//     }
// }

// fn parse_name_character_to_transformable_tobetoken(
//     transformable: TransformableToBeToken,
//      character: NameCharacter
//     ) -> Result<TransformableToBeToken, PostfixError> {
//         use TransformableToBeToken::*;
//         use NameCharacter::*;
//         match transformable {
//             TOBENUMBER(n) => match character {
//                 ALPHABET(a) => Err(PostfixError::NumberLiteralFollowedByAlphabetError(n, a)),
//                 DIGIT(Digit(d)) => Ok(TOBENUMBER(n * 10 + d as i32)),
//             },
//             TOBEWORD(mut tbf) => { tbf.tail.push(character); Ok(TOBEWORD(tbf)) },
//         }
//     }

// fn parse_name_character_to_tobetoken(tobetoken: ToBeToken, character: NameCharacter) 
//     -> Result<ToBeToken, PostfixError> {
//         use ToBeToken::*;
//         use NameCharacter::*;
//         use TransformableToBeToken::*;
//         Ok( match tobetoken {
//             EMPTY => match character {
//                 ALPHABET(alphabet) => TRANSFORMABLE(TOBEWORD(ToBeFunction{ head: alphabet, tail: Vec::new()})),
//                 DIGIT(Digit(d)) => TRANSFORMABLE(TOBENUMBER(d as i32)),
//             },
//             TRANSFORMABLE(transformable) => TRANSFORMABLE(
//                 parse_name_character_to_transformable_tobetoken(transformable, character)?
//             )
//         } )
// }

// fn parse_one_character_with_tobetoken(tobetoken: ToBeToken, character: Character) 
// -> Result<(ToBeToken, Vec<Token>), PostfixError> {
//     use Character::*;
//     use ToBeToken::*;
//     use BreakingCharacter::*;
//     Ok(match character {
//         BREAK(character) => (EMPTY, vec! [
//             match tobetoken {
//                 TRANSFORMABLE(t) => Some(t.to_token()?),
//                 EMPTY => None,
//             },
//             match character {
//                 LEFT => Some(Token::LEFT),
//                 RIGHT => Some(Token::RIGHT),
//                 SPACE => None,
//             }
//         ].into_iter().flatten().collect()),
//         NAME(character) => (parse_name_character_to_tobetoken(
//             tobetoken, character
//         )?, vec! []),

//     })
// }
// pub struct Program;