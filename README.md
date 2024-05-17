# Expand Array Rust Macro

This project is a simple macro that allows you to expand an array into a list of
arguments. This is useful when you need to make available global constants in a
dynamic library, for example.

## Limitations

The library can only convert string-like types for now. This is what I needed
for my project, but I'm more than happy to accept pull requests that add more
types.

Available types:

- `u8`, from:
  - `[u8]`
  - `[i8]`
  - `[byte char]`
  - `byte string`
  - `&str`
  - `c_char`
  - `c string`
- `char`, from:
  - `[char]`
  - `&str`
- `c_char`, from:
  - `[u8]`
  - `[i8]`
  - `[byte char]`
  - `byte string`
  - `&str`
  - `c_char`
  - `c string`

## Usage

```rust
use expand_array::arrr;

let var: [<target_type>; target_len] = arrr!([<target_type>; target_len], <array>);
```

## Example

```rust
use expand_array::arrr;

let arr: [u8; 10] = arrr!([u8; 10], [1, 2, 3, 4, 5]);
assert_eq!(arr, [1, 2, 3, 4, 5, 0, 0, 0, 0, 0]);

let arr: [u8; 10] = arrr!([u8; 10], b"Hello");
assert_eq!(arr, [72, 101, 108, 108, 111, 0, 0, 0, 0, 0]);

let arr: [u8; 10] = arrr!([u8; 10], "Hello");
assert_eq!(arr, [72, 101, 108, 108, 111, 0, 0, 0, 0, 0]);

let arr: [char; 10] = arrr!([char; 10], ['H', 'e', 'l', 'l', 'o']);
assert_eq!(arr, ['H', 'e', 'l', 'l', 'o', '\0', '\0', '\0', '\0', '\0']);

let arr: [char; 10] = arrr!([char; 10], "Hello");
assert_eq!(arr, ['H', 'e', 'l', 'l', 'o', '\0', '\0', '\0', '\0', '\0']);

use ::std::ffi::c_char;

let arr: [c_char; 10] = arrr!([c_char; 10], [72i8, 101i8, 108i8, 108i8, 111i8]);
assert_eq!(arr, [72i8, 101i8, 108i8, 108i8, 111i8, 0i8, 0i8, 0i8, 0i8, 0i8]);

let arr: [c_char; 10] = arrr!([c_char; 10], c"Hello");
assert_eq!(arr, [72i8, 101i8, 108i8, 108i8, 111i8, 0i8, 0i8, 0i8, 0i8, 0i8]);

let arr: [c_char; 10] = arrr!([c_char; 10], [72u8, 101u8, 108u8, 108u8, 111u8]);
assert_eq!(arr, [72i8, 101i8, 108i8, 108i8, 111i8, 0i8, 0i8, 0i8, 0i8, 0i8]);

let arr: [c_char; 10] = arrr!([c_char; 10], [b'H', b'e', b'l', b'l', b'o']);
assert_eq!(arr, [72i8, 101i8, 108i8, 108i8, 111i8, 0i8, 0i8, 0i8, 0i8, 0i8]);

let arr: [c_char; 10] = arrr!([c_char; 10], b"Hello");
assert_eq!(arr, [72i8, 101i8, 108i8, 108i8, 111i8, 0i8, 0i8, 0i8, 0i8, 0i8]);

let arr: [c_char; 10] = arrr!([c_char; 10], "Hello");
assert_eq!(arr, [72i8, 101i8, 108i8, 108i8, 111i8, 0i8, 0i8, 0i8, 0i8, 0i8]);
```
