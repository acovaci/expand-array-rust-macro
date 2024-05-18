#[macro_export]
macro_rules! arrr {
    ( $array:tt as $typ:tt $(or $default:tt )? ) => {{
        let bind = arrr!( @bind, $array, $typ );

        let mut other = arrr!( @init, $typ $(, $default)? );
        let mut i = 0;

        while i < bind.len() {
            other[i] = bind[i] as arrr!( @typ, $typ );
            i += 1;
        }

        other
    }};

    ( @bind, [$($x:tt),*], $typ:tt ) => {
        [$($x),*]
    };

    ( @bind, $x:literal, $typ:tt ) => {{
        let bytes_arr: [u8; arrr!( @size, $typ )] = $crate::bitify!($x);
        bytes_arr
    }};

    ( @typ, [$type:ty; $size:expr] ) => {
        $type
    };

    ( @size, [$type:ty; $size:expr] ) => {
        $size
    };

    ( @default, $default:expr ) => {
        $default
    };

    ( @init, $typ:tt, $default:expr ) => {
        [arrr!( @default, $default ); arrr!( @size, $typ )]
    };

    ( @init, $typ:tt ) => {
        [0 as arrr!( @typ, $typ ); arrr!( @size, $typ )]
    };
}
