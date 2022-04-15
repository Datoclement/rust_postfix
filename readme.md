# Postfix in Rust

Implementation of Postfix (defined in the book *Design Concepts in Programming Languages* by Franklyn A. Turbak, David K. Gifford) in Rust.

Just a personal beginner project to learn Rust.


First Impression of Rust:

1. Interesting combo of a very useful Enum structure and pattern matching. Not the first language that I learned having it (OCamel and Haskell). But with its imperative syntax the usage seems way more intuitive. During the coding, I tried to avoid at all possibility usage of `if`. Indeed, personal take is that `match` is just more visually pleasing and feel safer because it forces programmers to exhaust all cases or explicitly ignore them (by using wildcast `_` to pattern match all the remaining cases). I feel that unless we are absolutely sure of what is being ignored, we should never use `_`.

2. The whole process of writing the program becomes quite *type-oriented*. A very different style for me coming from Python, which maybe the exact opposite (though, the gap between dynamically-typed and statically-typed is becoming smaller and smaller, with on one hand optional and gradual typing getting introduced to dynamic languages and on the other hand type inferences are becoming smarter and smarter). 

    Interesting things about writing programs from a design of types are that 
    1. sometimes I got lost in the battle against types and compilers and lost the sight of which part I was actually coding; 
    2. However, when the compiler was finally pleased, the program would mostly work.

3. Borrowing and ownership is something that is not in the central part of this project. The constant attention drawing of the borrow-checker in this regard can be a bit annoying because it is constantly dragging my mind out of the domain of the project but instead into the memory stuffs that for the moment are not the real issues.
