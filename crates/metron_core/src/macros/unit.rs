
#[macro_export]
macro_rules! def_unit {
    ( $vis:vis $unit_name:ident; ) => {
        $vis struct $unit_name;
        impl $crate::unit::Unit for $unit_name {}
        impl<N> $crate::convert::FromUnit<N, $unit_name> for $unit_name{
            type Output = N;
            fn from_unit(num: N) -> <Self as $crate::convert::FromUnit<N, $unit_name>>::Output { num }
        }
    };
    ( $vis:vis $unit_name:ident {
        $(from $from_body:tt ,)*
        // $(into $into_body:tt ,)*
        $(exp   $exp_body:tt ,)*
        $(sym   $sym_body:tt ,)?
        // $(qua   $qua_body:tt ,)?
        // $(dim   $dim_body:tt ,)?
    }) => {
        $crate::def_unit!($vis $unit_name;);
        $( $crate::impl_unit_from!     ($unit_name, $from_body); )*
        // $( $crate::impl_unit_into!     ($unit_name, $into_body); )*
        $( $crate::impl_unit_exp_scale!($unit_name,  $exp_body); )*
        $( $crate::impl_fmt_symbol!    ($unit_name,  $sym_body); )?
    };

    ( $vis:vis $scale_name:ident |: $base_unit:ty => $unit_exp_name:ident; ) => {
        $crate::def_unit!($vis $unit_exp_name;);
        $crate::impl_unit_exp_scale!($scale_name |: $base_unit => $unit_exp_name);
    };
    ( $vis:vis $scale_name:ident |: $base_unit:ty => $unit_exp_name:ident {
        $(from $from_body:tt ,)*
        $(sym   $sym_body:tt ,)?
    } ) => {
        $crate::def_unit!($vis $scale_name |: $base_unit => $unit_exp_name;);
        $( $crate::impl_unit_from! ($unit_exp_name, $from_body); )*
        $( $crate::impl_fmt_symbol!($unit_exp_name,  $sym_body); )?
    };

    ( $vis:vis $lhs:ty |* $rhs:ty => $mul:ident; ) => {
        $crate::def_unit!($vis $mul;);
        $crate::impl_unit_mul!($lhs |* $rhs => $mul);
    };
    ( $vis:vis $lhs:ty |* $rhs:ty => $mul:ident {
        $(from $from_body:tt ,)*
        $(sym   $sym_body:tt ,)?
    } ) => {
        $crate::def_unit!($vis $lhs |* $rhs => $mul;);
        $( $crate::impl_unit_from!  ($mul, $from_body); )*
        $( $crate::impl_fmt_symbol! ($mul,  $sym_body); )?
    };

    ( $lhs:ty |/ $rhs:ty => $vis:vis $div:ident; ) => {
        $crate::def_unit!($vis $div;);
        $crate::impl_unit_div!($lhs |/ $rhs => $div);
    };
    ( $lhs:ty |/ $rhs:ty => $vis:vis $div:ident {
        $(from $from_body:tt ,)*
        $(sym   $sym_body:tt ,)?
    } ) => {
        $crate::def_unit!($lhs |/ $rhs => $vis $div;);
        $( $crate::impl_unit_from!  ($div, $from_body); )*
        $( $crate::impl_fmt_symbol! ($div,  $sym_body); )?
    };
}

