#[macro_export]
macro_rules! def_exp_scale_exp{
    (
        $(
            $vis:vis $name:ident[
            $(
                $type:ty = $value:expr ,
            )* ],
        )*
    ) => {
        $(
            $vis struct $name;
            $(
                $crate::impl_exp_scale_exp!($name | $type = $value);
            )*
        )*
    }
}

#[macro_export]
macro_rules! impl_exp_scale_exp {
    ( $name:ty | $type:ty = $value:expr ) => {
        impl $crate::unit::scale::exp::ScaleExponent<$type> for $name {
            const EXPONENT: $type = $value;
        }
    };
}
