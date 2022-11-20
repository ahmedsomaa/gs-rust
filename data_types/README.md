# Data Types

A rust program to get started with rust data types.

## Covered in this Lesson

Rust has two data types: _scalar_ and _compound_.

### Scalar

A _scalar_ type represents a single value. Rust has four primary scalar types: integers, floating-point numbers,
Booleans, and characters.

- Integers, `u8, i8, u16, i16, u32, i32...etc`.
- Folating points, `f32, f64`.
- Characters `char`. Chars are 4 bytes in size.
- Booleans `bool`.

### Compounds

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

#### Tuples

A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
Example is `let tup : (bool, u8, c) = (true, 8, 'y')`. You can access a tuple value by either _destructing_ or using
the dot notation.

```rs
    // destructuring
    let tup : (bool, u8, char) = (true, 8, 'y');

    let (b, u, c) = tup;

    // dot notation
    let condition = tup.0;
    let integer = tup.1;
    let charachter = tup.2;
```

#### Arrays

Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must
have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length. Array
elements can be accessed using the bracket notaion `arr[0]` for example.

```rs
    let arr = [1, 2, 3];
    let list: [char; 4] = ['R', 'u', 's', 'T'];
    let init = [3; 5];
```
