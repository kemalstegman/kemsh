//! kemsh is an interpreted shell language designed to fix issues with shell
//! scripting and prompt parsing. It is expression-oriented, semicolon
//! terminated, and features dynamic yet strict typing as well as
//! unconventional syntax (particularly for shells). Though its design choices
//! focus on readability, explicitness, and verbosity, all for scripting, I
//! hope it gains favor in usage for interactive shells as well.
//!
//! # General Structure
//!
//! Each line of kemsh is technically an expression that gets evaluated. The
//! outermost or top level expression is terminated by a semicolon. In an
//! interactive shell, the value of each TLE is printed to the terminal, with
//! one exception (keep reading!). Many keywords act as prefix unary operators
//! to values, and evaluate to another value. Some keywords have custom
//! grammars. Curly braces `{}`, in any context, denote a new lexical scope.
//! Variables within a scope will be dropped when the scope ends. A `{}` can
//! evaluate to a value. The last TLE in a `{}` can have no `;`. This causes
//! the `{}` to evaluate to the value of that TLE. The end of a `{}` also causes
//! the executor to reap terminated processes. The executor must reap process at
//! the end of a `{}`, but may also reap processes any other time as well.
//!
//! # Let, Variables, Void, and Values
//!
//! The `let` keyword has a custom grammar and is used to declare variables.
//! A type may be specified and the variable can be initialized or
//! uninitialized. Variable names can be alphanumeric including underscores,
//! but they cannot start with a number. A `let` expression looks like
//! `let var[: type][ = expression];`. The variable, once bound to a type,
//! cannot change its type. It will reject any value not of its type and cause
//! an error. If it has no type, it binds to the type of its first
//! initialization. The `let` expression evaluates to `void`. `void` is both
//! a type and not a type, and both a value and not a value. Variables cannot
//! store `void`. `void` cannot be operated upon. If the TLE evaluates to
//! `void`, nothing will be printed. There are a couple usages for `void`, one
//! of them being to force the expression to never be operated upon. Another is
//! that `void` is used to drop variables before the end of the scope, allowing
//! another declaration with the same variable name to occur. It cannot be the
//! "value" `void`. It must be the keyword `void`: `var = void;`. (The value
//! `void` cannot be operated upon, including on the righthand side of `=`).
//! A similar syntax is used to assign declared variables values in general:
//! `var = expression`. The assignment expression evaluates to the expression
//! in the assignment.
//!
//! ## Values
//!
//! `integer`, `float`, and `boolean` are the most primitive primitives.
//! `boolean`s have the `true` and `false` keywords reserved. `integer`s and
//! `float`s are 64-bit. `integer`s are defined as the digits, consecutively,
//! without whitespace, and underscores can be used to separate the digits, but
//! it cannot start with one. `integer`s can also be prefixed with `0b` or `0x`
//! for binary or hexadecimal. `float`s are 1 or more digits, followed by 1
//! period, followed by 1 or more digits.
//!
//! ### Strings
//!
//! `string`s are also primitives, and are UTF-8. They are raw by default,
//! meaning all characters are read literally without escaping. The `"`
//! delimeters can also be prefixed and suffixed with matching `#` (to allow
//! `"` and `"#` inside the string). `string`s cannot span multiple lines, with
//! one exception. The starting string delimeter can be prefixed with an `e` to
//! opt into escaping. Escape sequences (starting with a backslash) work. `{}`
//! can be used inside the string to contain variables to be formatted into the
//! string. A string can span multiple lines if the line ends with a backslash,
//! where then all the whitespace after, including the newline, until a
//! non-whitespace character is reached, will be ignored. The string continues
//! after the ignore whitespace. This can only work with escaped strings.
//!
//! Lists are defined by `[]`. Elements are comma separated, and the oxford
//! comma is optional.
//!
//! # Control Flow
//! `loop {}` denotes an infinite loop. `while condition {}` denotes a
//! conditional loop. `break` exits the innermost loop. `if condition {}`,
//! `else if condition {}`, and `else {}` denote typical conditional branching.
//!
//! # Typical Operations
//! `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `<`, `<=`, `>`, `>=` are the infix
//! operations addition and concatenation, subtraction, multiplication,
//! division, modulo, equal, not equal, less than, less than or equal to,
//! greater than, and greater than or equal to. Integer division is floor
//! division and always returns an integer. Comparison for strings always
//! compares the characters and not the location in memory. `-` and `!` are
//! unary prefix operations for negation (for integers and floats) and logical
//! not (for booleans).

pub mod syntax;

pub mod ast;

pub mod executor;

// lookahead iterator
// todo: make own crate
pub mod abstract_lookahead;
