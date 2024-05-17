use expand_array::arrr;

#[test]
fn test_arrr_u8_u8() {
    let output: [u8; 10] = arrr!([u8; 10], [104, 101, 108, 108, 111]);

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_u8_byte() {
    let output: [u8; 10] = arrr!([u8; 10], [b'h', b'e', b'l', b'l', b'o']);

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_u8_bytestr() {
    let output: [u8; 10] = arrr!([u8; 10], b"hello");

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

fn test_arrr_u8_str() {
    let output: [u8; 10] = arrr!([u8; 10], "hello");

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

fn test_arr_u8_c_char() {
    let output: [::std::ffi::c_char; 10] =
        arrr!([::std::ffi::c_char; 10], [104, 101, 108, 108, 111]);

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

fn test_arr_u8_c_str() {
    let output: [::std::ffi::c_char; 10] = arrr!([::std::ffi::c_char; 10], "hello");

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_char_char() {
    let output: [char; 10] = arrr!([char; 10], ['h', 'e', 'l', 'l', 'o']);

    assert_eq!(
        &output,
        vec!['h', 'e', 'l', 'l', 'o', '\0', '\0', '\0', '\0', '\0'].as_slice()
    );
}

#[test]
fn test_arrr_char_str() {
    let output: [char; 10] = arrr!([char; 10], "hello");

    assert_eq!(
        &output,
        vec!['h', 'e', 'l', 'l', 'o', '\0', '\0', '\0', '\0', '\0'].as_slice()
    );
}

#[test]
fn test_arrr_c_char_u8() {
    let output: [::std::ffi::c_char; 10] =
        arrr!([::std::ffi::c_char; 10], [104, 101, 108, 108, 111]);

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_c_char_byte() {
    let output: [::std::ffi::c_char; 10] =
        arrr!([::std::ffi::c_char; 10], [b'h', b'e', b'l', b'l', b'o']);

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_c_char_bytestr() {
    let output: [::std::ffi::c_char; 10] = arrr!([::std::ffi::c_char; 10], b"hello");

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_c_char_str() {
    let output: [::std::ffi::c_char; 10] = arrr!([::std::ffi::c_char; 10], "hello");

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_c_char_c_char() {
    let output: [::std::ffi::c_char; 10] = arrr!(
        [::std::ffi::c_char; 10],
        [104i8, 101i8, 108i8, 108i8, 111i8]
    );

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}

#[test]
fn test_arrr_c_char_c_str() {
    let output: [::std::ffi::c_char; 10] = arrr!([::std::ffi::c_char; 10], c"hello");

    assert_eq!(
        &output,
        vec![104, 101, 108, 108, 111, 0, 0, 0, 0, 0].as_slice()
    );
}
