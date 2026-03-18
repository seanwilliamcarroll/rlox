# rlox

A Rust implementation of the Lox interpreter from Robert Nystrom's
[*Crafting Interpreters*](https://craftinginterpreters.com/).

## Goal

Work through the fundamentals of interpreter design — scanning, parsing, and
evaluation — by implementing Lox in Rust rather than the book's Java/C.

## What's here

- A **scanner** that tokenizes the full Lox token set: literals, operators,
  keywords, strings, and numbers
- An **AST** (`Expr`) with binary, unary, grouping, and literal nodes
- An **AST printer** for inspecting the tree structure
- A REPL and file-runner entry point

## Next steps

- Parser: turn the token stream into an AST
- Interpreter: walk the AST and evaluate expressions
- Statements, variables, control flow, functions, and classes (chapters 8+)
