# simple recursive descent parser

A simple recursive descent parser for a toy language.
The parser uses [pratt algorithm](https://en.wikipedia.org/wiki/Operator-precedence_parser#Pratt_parsing)
to implement operator-precedence.

# language

Here is an example of the toy language implemented.

    x <- 10
    y <- 5 - 1
    x + y * 10

The language support assignment to variables and `+` `-` `*` `/` operations.
Each assignment statments is separed by a newline (`\n`).
Last line must be an expression, which is the result of the program.

## grammar

Language grammar rules are as follows.

    file =
      | (assignment EOL)* expression EOL?

    assignment =
      | NAME '<-' expression

    expression =
      | atom
      | expression '+' expression
      | expression '-' expression
      | expression '*' expression
      | expression '/' expression

    atom =
      | NAME
      | NUMBER

Where upper case denote following tokens:

    NAME = [a-z]+
    NUMBER = [0-9]+
    EOL = '\n'


# usage

    $ pratt [--tokens] [--ast] source-file0 source-file1 ...

The `pratt` binary will parse and evaluate each specified source file.
The result will be printed to stdout. Note that error handling is very primitive, `pratt` will panic or crash on invalid syntax.

The `--tokens` and `--ast` specify that parsed tokens and AST should be printed as well.

# building

Make sure you have the standard Rust tooling installed.
Build the binary with:

    cargo build

Run the test with:

    cargo test
