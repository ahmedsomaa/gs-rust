# Variables

A rust program to introduce variables in Rust.

## Covered in this Lesson

### Variables

- To declare a variable in Rust, use the keyword `let`. For example `let x = 5;`.
- Variables in Rust are _immutable_ by default which means once created their values cannot change. To create a mutable
  variable in Rust, declare it using the `mut` keyword. For example `let mut x = 5;`

### Constants

- To declare a constant value in Rust, use `const` keyword.
- Constants **_must_** be annotated with a data type.
- Unlike variables, constants are always _immutable_ which means the `mut` keyword cannot be used with a constant.
- Rust prefers capital case letters separated by underscore for constant names. For example,
  `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`

### Shadowing

- Shadowing is when a new variale is declared with the same name as previous variable(s).
  ```rs
  let x = 5;
  let x = 6;
  ```
- Shadowing leads to the first variable is shadowed by the second, which means that the second variable is what
  the compiler will see when you use the name of the variable. So in the example code above, the value of `x` would be 6
  not 5.
