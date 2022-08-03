# Writing a C Compiler

## Context

I am currently studying Compilers as a part of my Computer Science and Engineering Curriculum. Given the slow pace of class I have decided that I want a fruitful outcome from this course and hence am building an actual compiler.

## Resource

I have been following the [Writing a C Compiler](https://norasandler.com/2017/11/29/Write-a-Compiler.html) Series by [Nora Sandler](https://norasandler.com/about/). Given the series was made for x32 bit architecture and that isn't the industry standard now I have tried myself to change many of the code generation calls into x86_x64 bit alternative.

### Testing

The series is provided with a [github repository](https://github.com/nlsandler/write_a_c_compiler) consisting of several test cases for each stage of series.

## Learnings

- You’ll learn about abstract syntax trees (ASTs) and how programs can represent and manipulate other programs. Handy for working with linters, static analyzers, and metaprogramming of all sorts.
- You’ll learn about assembly, calling conventions, and all the gritty, low-level details of how computers, like, do stuff.
- It seems like an impossibly hard project (but isn’t!), so writing one will make you feel like a badass.

## Preliminaries

- I have been excited about Rust since I got to know about it. Writing a compiler requires us to have capabilities for **sum types** and **pattern matching**. Hence Rust shines with memory safety as well so am going ahead with it.
- I also won't be using automatic parser and scanner generators instead as in series will be implementing the lexer and a recursive decent parser.

## Modules

- [main.rs](./src/main.rs) is the main file which will call other module functions.
- [lexer.rs](./src/lex/mod.rs) is responsible for tokenizing and setting up the model for tokens.
- [ast.rs](./src/ast/mod.rs) is our parser which generated the Abstract Syntax Tree based on the provided grammar.
- [codegen.rs](./src/codegen/mod.rs) generates the assembly code for x86_x64 architecture provided the AST.

## Status

- [x] Compile a program that returns a single integer
- [x] Adding three unary operators (~,-,!)
- [x] Binary operations to support basic arithmetic while handling operator precedence and associativity

## Grammar

The following is grammar supported as of now in [Backus Naur Form](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form):

```
<program> ::= <function>
<function> ::= "int" <id> "(" ")" "{" <statement> "}"
<statement> ::= "return" <exp> ";"
<exp> ::= <logical-and-exp> { "||" <logical-and-exp> }
<logical-and-exp> ::= <equality-exp> { "&&" <equality-exp> }
<equality-exp> ::= <relational-exp> { ("!=" | "==") <relational-exp> }
<relational-exp> ::= <additive-exp> { ("<" | ">" | "<=" | ">=") <additive-exp> }
<additive-exp> ::= <term> { ("+" | "-") <term> }
<term> ::= <factor> { ("*" | "/") <factor> }
<factor> ::= "(" <exp> ")" | <unary_op> <factor> | <int>
<unary_op> ::= "!" | "~" | "-"
```

## Tokens

Follwing are the supported tokens given the current state of compiler:

- Open brace {
- Close brace }
- Open parenthesis (
- Close parenthesis )
- Semicolon ;
- Int keyword int
- Return keyword return
- Identifier [a-zA-Z_]\w*
- Integer literal [0-9]+
- Minus -
- Bitwise complement ~
- Logical negation !
- Addition +
- Multiplication *
- Division /
