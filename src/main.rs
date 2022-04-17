use std::fs;

mod postfix {   
    pub mod enums;
    pub mod lexer;
    pub mod tests;
    pub mod errors;
    pub mod programs;
}

use postfix :: {
    programs :: {
        compile_and_run,
    }
};

enum CodeType<'a> {
    FILE(&'a str),
    CODE(&'a str),
}

fn main () {
    use CodeType::*;
    let testcases = vec![
        (FILE("code/postfix/ex5.postfix"), vec![]),
        (FILE("code/postfix/ex6.postfix"), vec![]),
        (FILE("code/postfix/ex7.postfix"), vec![]),
        (FILE("code/postfix/ex8.postfix"), vec![]),
        (FILE("code/postfix/ex9.postfix"), vec![]),
        (CODE("(postfix 2)"), vec![3, 4]),
        (CODE("(postfix 2 swap)"), vec![3, 4]),
        (CODE("(postfix 3 pop swap)"), vec![3, 4, 5]),
        (CODE("(postfix 2 swap)"), vec![3]),
        (CODE("(postfix 1 pop)"), vec![4, 5]),
        (CODE("(postfix 1 4 sub)"), vec![3]),
        (CODE("(postfix 1 4 add 5 mul 6 sub 7 div)"), vec![3]),
        (CODE("(postfix 5 add mul sub swap div)"), vec![7, 6, 5, 4, 3]),
        (CODE("(postfix 3 4000 swap pop add)"), vec![300, 20, 1]),
        (CODE("(postfix 2 add 2 div)"), vec![3, 7]),
        (CODE("(postfix 1 3 div)"), vec![17]),
        (CODE("(postfix 1 3 rem)"), vec![17]),
        (CODE("(postfix 1 4 lt)"), vec![3]),
        (CODE("(postfix 1 4 lt)"), vec![5]),
        (CODE("(postfix 1 4 lt 10 add)"), vec![3]),
        (CODE("(postfix 1 4 mul add)"), vec![3]),
        (CODE("(postfix 2 4 sub div)"), vec![4, 5]),
        (CODE("(postfix 2 1 nget)"), vec![4, 5]),
        (CODE("(postfix 2 2 nget)"), vec![4, 5]),
        (CODE("(postfix 2 3 nget)"), vec![4, 5]),
        (CODE("(postfix 2 0 nget)"), vec![4, 5]),
        (CODE("(postfix 1 (2 mul) 1 nget)"), vec![3]),
        (CODE("(postfix 1 1 nget mul)"), vec![5]),
        (CODE("(postfix 4 4 nget 5 nget mul mul swap 4 nget mul add add)"), vec![3, 4, 5, 2]),
        (CODE("(postfix 1 (2 mul) exec)"), vec![7]),
        (CODE("(postfix 0 (0 swap sub) 7 swap exec)"), vec![]),
        (CODE("(postfix 0 (2 mul))"), vec![]),
        (CODE("(postfix 0 3 (2 mul) gt)"), vec![]),
        (CODE("(postfix 0 3 exec)"), vec![]),
        (CODE("(postfix 0 (7 swap exec) (0 swap sub) swap exec)"), vec![]),
        (CODE("(postfix 2 (mul sub) (1 nget mul) 4 nget swap exec swap exec)"), vec![-10, 2]),
        (CODE("(postfix 1 2 3 sel)"), vec![1]),
        (CODE("(postfix 1 2 3 sel)"), vec![0]),
        (CODE("(postfix 1 2 3 sel)"), vec![17]),
        (CODE("(postfix 0 (2 mul) 3 4 sel)"), vec![]),
        (CODE("(postfix 4 lt (add) (mul) sel exec)"), vec![3, 4, 5, 6]),
        (CODE("(postfix 4 lt (add) (mul) sel exec)"), vec![4, 3, 5, 6]),
        (CODE("(postfix 1 1 nget 0 lt (0 swap sub) () sel exec)"), vec![-7]),
        (CODE("(postfix 1 1 nget 0 lt (0 swap sub) () sel exec)"), vec![6]),

        // 2 x - 5
        (CODE("(postfix 1 ((3 nget swap exec) (2 mul swap exec) swap) (5 sub) swap exec exec)"), vec![2]),

        // not
        (CODE("(postfix 1 0 1 sel)"), vec![6]),
        (CODE("(postfix 1 0 1 sel)"), vec![0]),
        (CODE("(postfix 1 0 1 sel)"), vec![1]),

        // and
        (CODE("(postfix 2 (1 0 sel) (0) sel exec)"), vec![6, 0]),
        (CODE("(postfix 2 (1 0 sel) (0) sel exec)"), vec![0, 0]),
        (CODE("(postfix 2 (1 0 sel) (0) sel exec)"), vec![6, 1]),
        (CODE("(postfix 2 (1 0 sel) (0) sel exec)"), vec![0, 3]),

        // short-circuit and
        (CODE("(postfix 2 (1 nget) (0) sel exec)"), vec![0, 3]),
        (CODE("(postfix 2 (1 nget) (0) sel exec)"), vec![123, 3]),
        (CODE("(postfix 2 (1 nget) (0) sel exec)"), vec![123, 0]),
        (CODE("(postfix 2 (1 nget) (0) sel exec)"), vec![0, 0]),
    ];

    testcases.iter().map(|(testcase, arguments)|{
        let code = match testcase {
            FILE(filename) => fs::read_to_string(filename),
            CODE(code) => Ok(code.to_string()),
        };
        match code {
            Ok(code) => println! ("{:?}", compile_and_run(&code, arguments)),
            Err(_) => panic! ("read file failure!"),
        }
    }).collect()
}