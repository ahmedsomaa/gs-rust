# Guessing Game

A rust program to implement a simple guessing game.

## Covered in this Lesson

### Variables

- To declare a variable in Rust, use the keyword `let`. For example `let age = 26;`.
- Variables in Rust are _immutable_ by default which means once created their values cannot change. To create a mutable
  variable in Rust, declare it using the `mut` keyword. For example `let mut age = 26;`

### Standard Library & Crates

- Rust has a set of items defined in the standard library that it brings into the scope of every program. To import
  the standard library into your program, add `use std::{item_name}` at the top of your code file. An example of the
  items in the standard library would be `io` for handling input/output operations. To import it, add `use std::io;` at
  the top of your file.
- To add an external crate, library, to your project, add the crate name under the `[dependencies]` section in the
  `Cargo.toml` file using the following format `<crate_name> = "<crate_version>"`. For example, to add `rand` crate,
  add `rand = "0.8.3"`. This tells cargo to add the `rand` crate alongside with its dependencies compatible with
  the version specified. Then you need to run the `cargo build` command to bring the `rand` and its dependencies to
  your project so you can use its functinality.
- [Crates.io](https://crates.io) is a registery for rust community crates that you can add to your project.

### `match` experssion

A `match` expression is made up of _arms_. An arm consists of a _pattern_ to match against, and the code that should be run if the value given to `match` fits that arm’s pattern. Rust takes the value given to `match` and looks through each arm’s pattern in turn.
