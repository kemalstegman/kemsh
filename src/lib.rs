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
//! grammars. Curly braces `{}`, in any context, with one exception, denote a new lexical scope.
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
//! `void` cannot be operated upon, including on the right-hand side of `=`).
//! A similar syntax is used to assign declared variables values in general:
//! `var = expression`. The assignment expression evaluates to the expression
//! in the assignment.
//!
//! ## Values
//!
//! `integer`, `float`, and `boolean` are the most primitive primitives.
//! `boolean`s have the `true` and `false` keywords reserved. `integer`s are
//! signed 64-bit integers and `float`s are 64-bit floating-point numbers,
//! specifically the one defined in IEEE 754. `integer`s are defined as the digits, consecutively,
//! without whitespace, and underscores can be used to separate the digits, but
//! it cannot start with one. `integer`s can also be prefixed with `0b` or `0x`
//! for binary or hexadecimal. `float`s are 1 or more digits, followed by 1
//! period, followed by 1 or more digits.
//!
//! ### Strings
//!
//! `string`s are also primitives, and are UTF-8. They are raw by default,
//! meaning all characters are read literally without escaping. The `"`
//! delimiters can also be prefixed and suffixed with matching `#` (to allow
//! `"` and `"#` inside the string). `string`s cannot span multiple lines, with
//! one exception. The starting string delimiter can be prefixed with an `e` to
//! opt into escaping. Escape sequences (starting with a backslash) work. `{}`
//! can be used inside the string to contain variables to be formatted into the
//! string. A string can span multiple lines if the line ends with a backslash,
//! where then all the whitespace after, including the newline, until a
//! non-whitespace character is reached, will be ignored. The string continues
//! after the ignore whitespace. This can only work with escaped strings.
//!
//! ### Lists and Maps
//!
//! `list`s are defined by `[]`. Elements are comma separated, and the trailing
//! comma is optional. `map`s are defined by `#{}#`. This is the one exception
//! to curly braces where they do not create a new scope. key-value pairs are
//! initialized like `#{ abc: "def" }#` or `#{ "abc": "def" }#`. Again, the
//! trailing comma is optional. Indexing into either `list`s or `map`s is done
//! with `[expression]` where the expression evaluate to an `integer` for
//! `list`s and a `string` for `map`s. `map`s can also access their members
//! via the `.` operator, treating them like variable name fields.
//!
//! ### Options and Results
//!
//! `result`s and `option`s are extremely useful types for storing values that
//! may not exist, or values that could have failed. The `ok`, `err`, `some`,
//! and `none` keywords take an argument (except for `none`) and evaluate to
//! their corresponding type. They have `is_ok`, `is_err`, `is_some`, and
//! `is_none` fields that are accessible via the `.` operator which are
//! `boolean`s describing their state. The fields `ok`, `err`, and `some` may
//! or may not exist, and will cause an error if you try to access a field that
//! does not exist. Those fields contain their values.
//!
//! # Control Flow
//!
//! `loop {}` denotes an infinite loop. `while condition {}` denotes a
//! conditional loop. `break` exits the innermost loop. `if condition {}`,
//! `else if condition {}`, and `else {}` denote typical conditional branching.
//! `function`s are defined anonymously, and are values of the `fn` keyword.
//! The `let` keyword can be used to give the function a name:
//! `let fn_name = fn() -> void {}`. The `void` is the return type, and can
//! and will change when you want to return non-`void` values. However, you can
//! also just drop the arrow and type, and the function could return different
//! types of values. The `return` keyword is used for returning early out of
//! the function. It must always have an argument, and it will check against
//! the function return type, if the type is specified. Functions return void
//! if no return was hit, and have the same grammar as brace blocks for
//! returning the last expression.
//!
//! # Typical Operations
//!
//! `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `<`, `<=`, `>`, `>=` are the infix
//! operations addition and concatenation, subtraction, multiplication,
//! division, modulo, equal, not equal, less than, less than or equal to,
//! greater than, and greater than or equal to. Integer division is floor
//! division and always returns an integer. Comparison for strings always
//! compares the characters and not the location in memory. `-` and `!` are
//! unary prefix operations for negation (for integers and floats) and logical
//! not (for booleans).
//!
//! # Unusual Operations
//!
//! The `cd` keyword is used for changing directories. It prefixes a string
//! and evaluates to a `result`, where the `ok` value is the last directory,
//! as a `string`. This is useful because `let wd = (cd ".").ok;` can be used
//! to not only get the working directory, but to also assert that it exists.
//!
//! The `exit` keyword exits with a code. It prefixes an integer. Note that
//! not all integers can work as exit codes, depending on the platform. `exit`
//! can have no argument at all if it is directly before a `;` (e.g. `exit;`).
//! This default syntax is equivalent to `exit 0;`. `exit` cannot have a return
//! type because execution stops. If it did, it would be `void`.
//!
//! The `echo` keyword exists solely for cross platform compatibility. It echoes
//! the value it prefixes. It evaluates to `void`. `echon` is the same except
//! it does not print a newline.

//!
//!
//!

//! # Processes and Files
//!
//! unfinished; ignore. skeleton ideas.
//! `run "ls > lsout";`
//! `run (["ls"] > "lsout");`
//! `let p = spawn (["ls"] > "lsout");`
//! `wait p;`
//!
//!
//! `is` keyword for type checking
//!
//! another thing is how processes will work. I think I've finally settled on how
//! they will work. `run`, `spawn`, and `wait` will be the three operator keywords
//!  and `pcommand`, `phandle`, and `pexit` will be the three types. I will use the
//!  pipe | for piping, and that will become a new operator. I also think I can get
//!  away with making the pipe a unary operator to turn a string or a list of strings
//!  into a pcommand without needing another keyword solely for that. the pipe operator
//!  will function on pcommands, strings, and lists of strings, always outputing
//! a pcommand. I will overload the >, >>, and <, operators to also work on strings
//! , lists of strings, and pcommands for the normal shell meanings of those symbols.
//!  now, the three keywords. `run` does typical blocking behavior and outputs a phandle.
//!  it can take a string, list of strings, pcommand, or a phandle. the spawn command
//! is similar but it does background processes and only works on strings, lists of
//! strings, and pcommands. the wait command can take a phandle and it will wait until
//!  it is finished. now, run can only take a phandle if it isn't already running.
//! wait can only take a phandle if it is running. this is my solution to the fact
//! that it cannot be guaranteed that a process is truly terminated. the phandle type
//!  will be similar to results and options, in that it will have a is_terminated and
//!  is_running fields and will optionally have a .exit field, if it is terminated,
//! that will house a pexit. the pexit will have a way of giving the exit code or
//! the signal used to terminate it.
