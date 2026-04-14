//! # kemsh syntax
//!
//! kemsh is an expression-oriented, semicolon terminated, explicitly and dynamically typed shell language.
//! It takes inspiration from Rust, Lua, Python, and Bash (and other shell languages). The thing that
//! prompted me (pun intended) to create a shell language was the frustration of escaping quotes. Not only
//! that, but a recognition of how easily command injection can occur made me believe there was a need for
//! a different, more explicit and verbose shell language.
//!
//! ## Features / Language Concepts
//!
//! The basic types, boolean, integer, and float, work as you'd expect. A small note is that integers can be
//! prefixed with `0b` or `0x` for binary and hexadecimal, respectively.
//!
//! Strings are different than the norm. First, they are raw by default, meaning that escaping does not work.
//! Second, string delimeters (aka the `"`) can be prefixed and suffixed with any number of `#`, where the
//! prefix and suffix match. For example, `###"This is a raw string. \ and " and \n are included as their
//! literal characters."###` is a perfectly valid string. Raw strings cannot span multiple lines (this does not
//! mean they cannot contain newline characters). This is to prevent the ambiguity of the carriage return
//! character being or not being included in the string and because it would get very ugly when indented and
//! the string suddenly fully unindenting. Escaped strings are also possible. They differ from raw strings in
//! three ways: (1) escape codes like \n work, (2) variables can be formatted using braces like Rust format
//! strings, and (3) backslashes can be used at the end of a line to remove whitespace including the newline
//! (like Rust). To summarize, strings are raw by default and escaping is opt in, string delimeters can have
//! matching numbers of hashtags prefixed and suffixed to differentiate them from the text inside the string,
//! and escaped strings look like Rust's.
//!
//! Lists are defined by `[]` and are comma separated, and a comma between the last item and the closing bracket
//! is optional. todo!(): maps.
//!
//! The types Option and Result are loantypes from Rust. todo!() ?They are opaque to their contained types?
//! todo!() ?Just like user defined types (see below)?, the fields are accessed with the `.` operator. They
//! are guaranteed to have the fields `is_ok`, `is_err`, `is_some`, and `is_none`, respectively, where all
//! are booleans. Depending on the state of the type, the fields `ok`, `err`, and `some` may or may not exist.
//! Accessing these fields when their associated boolean field is false will cause an error, because the field
//! does not exist.
//!
//! ### Let, Variables, and Void
//!
//! Variables are declared with the `let` keyword. Variables must be declared before they can be used. Variables
//! can be optionally initialized and optionally bound to a type when declared. If both, the expression is checked
//! to be of the correct type, and causes an error if they differ. If no type is specified, the variable simply
//! binds itself to whatever type is first assigned to it. Once a variable is bound to a type, it cannot become
//! a different type. The syntax for the `let` keyword is as follows `let variable[: type][ = expression]`. A
//! `let` expression always returns void. Void? Void is both a type and not a type. It cannot be operated upon.
//! Variables cannot store it. Functions (and expressions) can return it. It represents the lack of a value. Note
//! that the Option type is the correct type for representing the value of no value, and it can be operated upon.
//! Variables can be assigned values (after they are created) with a simple `variable[: type] = expression` syntax.
//! The optional type checks if the expression evaluates to the type, and this has no effect on variables which are
//! already bound to a type. The assignment expression always returns the value assigned, so they can be chained.
//! Assigning a variable to void deletes it. This can only be done in the same scope it was declared, so no outer
//! scope variable can be deleted. Variable names can be alphanumeric including underscores, but they cannot start
//! with a number.
//!
//! ## Control Flow
//! `loop`, `while`, `for`, `break`, and `return`. They work as you might think, no parenthesis required, and return
//! is for functions (see below). todo!() ?There is a special range syntax, like that in Rust, specifically for for
//! loops?
//!
//! ## Keywords and Operations
//!
//! `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `<`, `<=`, `>`, `>=` are the infix operations addition and concatenation,
//! subtraction, multiplication, division, modulo, equal, not equal, less than, less than or equal to, greater than,
//! and greater than or equal to. Integer division is floor division and always returns an integer. Comparison for
//! strings always compares the characters and not the location in memory. `-` and `!` are unary prefix operations
//! for negation (for integers and floats) and logical not (for booleans).
//!
//! `cd`, `echo`, `exit`.
//!
//! ## Processes
//!
//! `command`, `spawn`
//!
//! ## Functions
//!
//! `fn name(args) [-> retvalue] {}`
//! `let name = fn(args) [-> retvalue] {}`
//!

pub mod lexer;
pub mod parser;

use itertools::peek_nth;

use crate::{
    ast::Expression,
    syntax::{
        lexer::{Lexer, LexerError},
        parser::{Parser, ParserError},
    },
};

pub fn syntax<I, E>(iter: I) -> impl Iterator<Item = Result<Expression, ParserError<LexerError<E>>>>
where
    I: Iterator<Item = Result<char, E>>,
{
    Parser::new(peek_nth(
        Lexer::new(peek_nth(iter.map(|x| x.map_err(|e| LexerError::Source(e)))))
            .map(|x| x.map_err(|e| ParserError::Source(e))),
    ))
}

// let var;
// let var: [type];
// let var = [expr];
// let var: [type] = [expr];
// var = [expr];
// echo [expr];
// cd [expr: string];
// if [expr: boolean] {}
// loop {}
// while [expr: boolean] {}
// block [expr: pcmd]; -> pexit
// spawn [expr: pcmd]; -> phandle
// process [expr: string];
// let var = fn() -> [type] {}
// let var = fn() {}
// fn var(){} -> [type] {}
// fn var(){} {}
// ??for??
// ??return [?expr];??
// ??break [?expr];??
