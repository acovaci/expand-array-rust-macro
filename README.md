# Expand Array Rust Macro

![](https://img.shields.io/github/actions/workflow/status/acovaci/expand-array-rust-macro/rust.yml)
[![](https://img.shields.io/crates/v/expand-array)
](https://crates.io/crates/expand-array)
[![](https://img.shields.io/crates/d/expand-array)
](https://crates.io/crates/expand-array)
[![](https://img.shields.io/crates/l/expand-array)
](https://crates.io/crates/expand-array)
[![](https://docs.rs/expand_array/badge.svg)
](https://docs.rs/expand-array)

This crate provides a macro called `arr!` to convert a static array to a
fixed-size array. Limited type conversion is supported. This is useful when you
need to make available global constants in a dynamic library, for example.

As a bonus, the crate also provides a `bitify!` macro to convert a string
literal to a fixed-size array of bytes.

## Usage

The `arr!` macro can take any two primitive types. On top of that, it can take
string literals (`&str`, byte strings and C strings). Using the `bitify!` macro,
it can convert it to a fixed-size array of any primitive type.

```rust
use expand_array::arrr;

let var: [<target_type>; target_len] = arrr!(<source> as [<target_type>; target_len]);
let var_with_default: [<target_type>; target_len] = arrr!(<source> as [<target_type>; target_len] with <default_value>);
```

## Example

```rust
use expand_array::arrr;

let arr: [u8; 10] = arrr!([1, 2, 3, 4, 5] as [u8; 10]);
assert_eq!(arr, [1, 2, 3, 4, 5, 0, 0, 0, 0, 0]);

let arr: [u8; 10] = arrr!([1, 2, 3, 4, 5] as [u8; 10] with 2);
assert_eq!(arr, [1, 2, 3, 4, 5, 2, 2, 2, 2, 2]);

let arr: [u8; 10] = arrr!(b"Hello" as [u8; 10]);
assert_eq!(arr, [72, 101, 108, 108, 111, 0, 0, 0, 0, 0]);

let arr: [u8; 10] = arrr!("Hello" as [u8; 10]);
assert_eq!(arr, [72, 101, 108, 108, 111, 0, 0, 0, 0, 0]);

let arr: [char; 10] = arrr!(['H', 'e', 'l', 'l', 'o'] as [char; 10]);
assert_eq!(arr, ['H', 'e', 'l', 'l', 'o', '\0', '\0', '\0', '\0', '\0']);

let arr: [&str; 10] = arrr!(["Hello", "world"] as [&str; 10] with '!');
assert_eq!(arr, ["Hello", "world", "!", "!", "!", "!", "!", "!", "!", "!"]);

use ::std::ffi::c_char;

let arr: [c_char; 10] = arrr!([72i8, 101, 108, 108, 111] as [c_char; 10]);
assert_eq!(arr, [72i8, 101, 108, 108, 111, 0, 0, 0, 0, 0]);

let arr: [c_char; 10] = arrr!(c"Hello" as [c_char; 10]);
assert_eq!(arr, [72i8, 101, 108, 108, 111, 0, 0, 0, 0, 0]);

let arr: [c_char; 10] = arrr!([72u8, 101, 108, 108, 111] as [c_char; 10]);
assert_eq!(arr, [72i8, 101, 108, 108, 111, 0, 0, 0, 0, 0]);

let arr: [c_char; 10] = arrr!([b'H', b'e', b'l', b'l', b'o'] as [c_char; 10]);
assert_eq!(arr, [72i8, 101, 108, 108, 111, 0, 0, 0, 0, 0]);

let arr: [c_char; 10] = arrr!(b"Hello" as [c_char; 10]);
assert_eq!(arr, [72i8, 101, 108, 108, 111, 0, 0, 0, 0, 0]);

let arr: [c_char; 10] = arrr!("Hello" as [c_char; 10] with "!");
assert_eq!(arr, [72i8, 101, 108, 108, 111, 33, 33, 33, 33, 33]);
```
