# Hello Cargo

A rust program to print `Hello, cargo!` to the screen and to get familiar with basic cargo commands.

## Covered in this Lesson

Cargo is Rust’s build system and package manager. Some basic cargo commands include:

- `cargo new <my_project>` to create a new cargo project. This command would create a new project under the `my_project`
  directory. The directory chould have a `src` directory and a `cargo.toml` file which contains information about
  the project's name, cargo version and edition. The `src` directory contains one file `main.rs`.
- `cargo build` to build your project. This command builds the project for the development environment and places
  the build artifacts under the `target/debug/` directory. To build for release, you need to pass the `--release`
  flag which would bulid the project with optimizations and places the build artifacts under the `target/release`
  directory.
- `cargo run` to build and execute binary build file.
- `cargo check` to check whether your project compiles without producing actual buld artifacts. This makes it much
  faster for checking the executability of your program as it grows up.
