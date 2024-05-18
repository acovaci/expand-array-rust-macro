pub mod flags;
pub mod main;

#[macro_export]
macro_rules! bitify {
    ( $str:tt ) => {{
        let s = stringify!($str);
        $crate::bitify_func(s.as_bytes())
    }};
}
