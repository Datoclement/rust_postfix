use super::errors::PostfixError;
use paste;
use super::programs::compile_and_run;

macro_rules! test_case {
    ($( ($name:ident, $code:expr, $arguments:expr, $expected:expr),)*) => ($(
        paste::item! {
            #[test]
            fn [< test_ $name >] () -> Result<(), PostfixError> { 
                assert_eq!(compile_and_run($code, &$arguments), $expected);
                Ok(())
            }
        }
    )*);
}

#[cfg(test)]
mod test_suite {
    use super::*;
    use super::PostfixError::*;
    use super::super::enums::{
        PostfixFunction::*,
        PostfixCommand::*,
        PostfixArithmetic::*,
    };
    

    test_case! [
        ( stack_top_at_the_beginning, "(postfix 2)", vec! [3, 4], Ok(3) ),
        ( swap_arguments, "(postfix 2 swap)", vec! [3, 4], Ok(4) ),
        ( pop_and_swap_arguments, "(postfix 3 pop swap)", vec! [3, 4, 5], Ok(5) ),
        ( not_enough_arguments, "(postfix 2 swap)", vec![3], Err(WrongNumberOfArguments{ expected: 2, actual: vec![3] })),
        ( too_many_arguments, "(postfix 1 pop)", vec![4, 5], Err(WrongNumberOfArguments{ expected: 1, actual: vec![4, 5] })),
        ( simple_substraction, "(postfix 1 4 sub)", vec![3], Ok(-1) ),
        ( simple_arithmetics, "(postfix 1 4 add 5 mul 6 sub 7 div)", vec![3], Ok(4) ),
        ( simple_arithmetics_with_swap, "(postfix 5 add mul sub swap div)", vec![7, 6, 5, 4, 3], Ok(-20) ),
        ( simple_arithmetics_with_swap_pop, "(postfix 3 4000 swap pop add)", vec![300, 20, 1], Ok(4020) ),
        ( averaging, "(postfix 2 add 2 div)", vec![3, 7], Ok(5) ),
        ( division, "(postfix 1 3 div)", vec![17], Ok(5) ),
        ( remainder, "(postfix 1 3 rem)", vec![17], Ok(2) ),
        ( less_than_true, "(postfix 1 4 lt)", vec![3], Ok(1) ),
        ( less_than_false, "(postfix 1 4 lt)", vec![5], Ok(0) ),
        ( true_is_one, "(postfix 1 4 lt 10 add)", vec![3], Ok(11) ),
        ( wrong_number_of_arguments_for_addition, "(postfix 1 4 mul add)", vec![3], Err(WrongNumberOfFunctionArguments { function: ARITHMETIC(ADD), expected_number_of_arguments: 2}) ),
        ( divide_by_zero, "(postfix 2 4 sub div)", vec![4, 5], Err(DivideByZero) ),
        ( simple_nget_first, "(postfix 2 1 nget)", vec![4, 5], Ok(4) ),
        ( simple_nget_second, "(postfix 2 2 nget)", vec![4, 5], Ok(5) ),
        ( nget_index_too_large, "(postfix 2 3 nget)", vec![4, 5], Err(IndexOutOfRangeByNGETFunction { index: 3, min: 1, max: 2 }) ),
        ( nget_index_too_small, "(postfix 2 0 nget)", vec![4, 5], Err(IndexOutOfRangeByNGETFunction { index: 0, min: 1, max: 2 }) ),
        ( invalid_nget_result, "(postfix 1 (2 mul) 1 nget)", vec![3], Err(InvalidValueByNGETFunction { command: EXECUTE(vec![INTEGER(2), SPECIAL(ARITHMETIC(MUL))]) }) ),
        ( square_with_nget, "(postfix 1 1 nget mul)", vec![5], Ok(25) ),
        ( quadratic_formular_with_nget, "(postfix 4 4 nget 5 nget mul mul swap 4 nget mul add add)", vec![3, 4, 5, 2], Ok(25) ),
        ( simple_exec, "(postfix 1 (2 mul) exec)", vec![7], Ok(14) ),
        ( negation_routine_with_exec, "(postfix 0 (0 swap sub) 7 swap exec)", vec![], Ok(-7) ),
        ( non_integer_final_stack_top, "(postfix 0 (2 mul))", vec![], Err(NonNumeralFinalState { command: EXECUTE(vec![INTEGER(2), SPECIAL(ARITHMETIC(MUL))])}) ),
        ( wrong_type_of_arguments_for_arithmetic, "(postfix 0 3 (2 mul) gt)", vec![], Err(WrongTypeOfFunctionArguments { function: ARITHMETIC(GT)}) ),
        ( wrong_type_of_arguments_for_exec, "(postfix 0 3 exec)", vec![], Err(WrongTypeOfFunctionArguments {function: EXEC}) ),
        ( mildly_complicated_exec, "(postfix 0 (7 swap exec) (0 swap sub) swap exec)", vec![], Ok(-7) ),
        ( complicated_exec, "(postfix 2 (mul sub) (1 nget mul) 4 nget swap exec swap exec)", vec![-10, 2], Ok(42) ),
        ( simple_sel_true, "(postfix 1 2 3 sel)", vec![1], Ok(2) ),
        ( simple_sel_false, "(postfix 1 2 3 sel)", vec![0], Ok(3) ),
        ( simple_sel_non_zero, "(postfix 1 2 3 sel)", vec![17], Ok(2) ),
        ( wrong_type_of_arguments_for_sel, "(postfix 0 (2 mul) 3 4 sel)", vec![], Err(WrongTypeOfFunctionArguments {function: SEL}) ),
        ( sel_exec_first, "(postfix 4 lt (add) (mul) sel exec)", vec![3, 4, 5, 6], Ok(30) ),
        ( sel_exec_second, "(postfix 4 lt (add) (mul) sel exec)", vec![4, 3, 5, 6], Ok(11) ),
        ( absolute_value_neg, "(postfix 1 1 nget 0 lt (0 swap sub) () sel exec)", vec![-7], Ok(7) ),
        ( absolute_value_pos, "(postfix 1 1 nget 0 lt (0 swap sub) () sel exec)", vec![6], Ok(6) ),
    ];
    
}
