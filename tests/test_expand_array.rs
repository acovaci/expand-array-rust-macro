use expand_array::arrr;

const TEST_CONST: [u8; 10] = arrr!(c"hello" as [u8; 10]);

#[test]
fn test_const() {
    assert_eq!(
        &TEST_CONST,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_u8_u8() {
    let output: [u8; 10] = arrr!([104, 101, 108, 108, 111] as [u8; 10]);

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_u8_byte() {
    let output: [u8; 10] = arrr!([b'h', b'e', b'l', b'l', b'o'] as [u8; 10]);

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_u8_bytestr() {
    let output: [u8; 10] = arrr!(b"hello" as [u8; 10]);

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

fn test_arrr_u8_str() {
    let output: [u8; 10] = arrr!("hello" as [u8; 10]);

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

fn test_arr_u8_c_char() {
    let output: [::std::ffi::c_char; 10] =
        arrr!([104u8, 101, 108, 108, 111] as [::std::ffi::c_char; 10]);
    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

fn test_arr_u8_c_str() {
    let output: [::std::ffi::c_char; 10] = arrr!(c"hello" as [::std::ffi::c_char; 10]);
    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_char_char() {
    let output: [char; 10] = arrr!(['h', 'e', 'l', 'l', 'o'] as [char; 10]);
    assert_eq!(
        &output,
        vec!['h', 'e', 'l', 'l', 'o', '\0', '\0', '\0', '\0', '\0'].as_slice()
    );
}

#[test]
fn test_arrr_char_str() {
    let output: [char; 10] = arrr!("hello" as [char; 10]);
    assert_eq!(
        &output,
        vec!['h', 'e', 'l', 'l', 'o', '\0', '\0', '\0', '\0', '\0'].as_slice()
    );
}

#[test]
fn test_arrr_c_char_u8() {
    let output: [::std::ffi::c_char; 10] =
        arrr!([104u8, 101, 108, 108, 111] as [::std::ffi::c_char; 10]);
    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_c_char_byte() {
    let output: [::std::ffi::c_char; 10] =
        arrr!([b'h', b'e', b'l', b'l', b'o'] as [::std::ffi::c_char; 10]);
    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_c_char_bytestr() {
    let output: [::std::ffi::c_char; 10] = arrr!(b"hello" as [::std::ffi::c_char; 10]);
    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_c_char_str() {
    let output: [::std::ffi::c_char; 10] = arrr!("hello" as [::std::ffi::c_char; 10]);
    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_c_char_c_char() {
    let output: [::std::ffi::c_char; 10] =
        arrr!([104i8, 101, 108, 108, 111] as [::std::ffi::c_char; 10]);

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_c_char_c_str() {
    let output: [::std::ffi::c_char; 10] = arrr!(c"hello" as [::std::ffi::c_char; 10]);
    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}
