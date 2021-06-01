#[macro_export]
macro_rules! def_exp_scale_base{
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
                $crate::impl_exp_scale_base!($name | $type = $value);
            )*
        )*
    }
}

#[macro_export]
macro_rules! impl_exp_scale_base{
    ( $name:ty | $type:ty = $value:expr ) => {
        impl $crate::unit::scale::exp::ScaleBase<$type> for $name{
            const BASE: $type = $value;
        }
    }
}