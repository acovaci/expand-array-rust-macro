mod arrr;
mod bitify_impl;

pub use bitify_impl::main::bitify_func;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        let x = arrr!([1u8, 2, 3, 4] as [i8; 10]);
        assert_eq!(x, [1i8, 2, 3, 4, 0, 0, 0, 0, 0, 0]);

        let x = arrr!([1u8, 2, 3, 4] as [i8; 10] or 2);
        assert_eq!(x, [1i8, 2, 3, 4, 2, 2, 2, 2, 2, 2]);

        let x = arrr!([500u16] as [i8; 10]);
        assert_eq!(x, [-12, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let x = arrr!(['a', 'b', 'c'] as [i8; 10]);
        assert_eq!(x, [97i8, 98, 99, 0, 0, 0, 0, 0, 0, 0]);

        let x = arrr!([b'a', b'b', b'c'] as [i8; 10]);
        assert_eq!(x, [97i8, 98, 99, 0, 0, 0, 0, 0, 0, 0]);

        let x = arrr!(["abc", "def"] as [&str; 10] or "");
        assert_eq!(x, ["abc", "def", "", "", "", "", "", "", "", ""]);
    }

    #[test]
    fn test_array_str() {
        let x = arrr!("abc" as [i8; 10]);
        assert_eq!(x, [97i8, 98, 99, 0, 0, 0, 0, 0, 0, 0]);

        let x = arrr!(b"abcd" as [i8; 10]);
        assert_eq!(x, [97i8, 98, 99, 100, 0, 0, 0, 0, 0, 0]);

        let x = arrr!(c"abcde" as [i8; 10]);
        assert_eq!(x, [97i8, 98, 99, 100, 101, 0, 0, 0, 0, 0]);

        let x = arrr!("👟💻🦴" as [i8; 20]);
        assert_eq!(
            x,
            [
                -16i8, -97, -111, -97, -16, -97, -110, -69, -16, -97, -90, -76, 0, 0, 0, 0, 0, 0,
                0, 0
            ]
        );
    }
}
