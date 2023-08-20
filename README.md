# Rust

A language empowering everyone to build reliable and efficient software.
Rust is a statically typed language.

## Why Rust ?

- Performance

- Reliability

- Productivity

## Quando utilizar Rust ?

Utilizado em ferramentas que demandam performance ( ex.: Firefox, Microsoft, etc )

## Instalação

- [Getting started](https://www.rust-lang.org/learn/get-started)

## Cargo

Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries.

Let’s recap what we’ve learned so far about Cargo:

- We can create a project using `cargo new`.
- We can build a project using `cargo build`.
- We can build and run a project in one step using `cargo run`.
- We can build a project without producing a binary to check for errors using `cargo check`.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

## Documentation

- [The Rust Programming Language](https://doc.rust-lang.org/stable/book/title-page.html)

## Data Types

- Scalar Types: A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

- Compound Types: can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

## Functions

### Statements && Expressions

- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value. Let’s look at some examples.

## Rust Code Style

- Rust code uses snake case as the conventional style for function and variable names.

## Ownership

First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped

### Material de apoio

[The cargo book](https://doc.rust-lang.org/cargo/)

[Gerencimento de memória - Stack x Heap](https://www.youtube.com/watch?v=7kJwVQGJCbw)

--- Continue here: [Using Structs to Structure Related Data](https://doc.rust-lang.org/stable/book/ch05-00-structs.html#using-structs-to-structure-related-data)