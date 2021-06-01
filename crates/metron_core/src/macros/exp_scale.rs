#[macro_export]
macro_rules! def_exp_scale{
    (
        $(
            $vis:vis $name:ident[
            $(
                $base:ty =^ $exp:ty ,
            )*
            ]
            $(
                : default $base_default:ty[$b:ty, $e:ty]
            )?
            ,
        )*
    ) => {
        $(
            $vis struct $name;
            $(
                $crate::impl_exp_scale!($name = $base =^ $exp);
            )*
            $(
                $crate::impl_exp_scale_default!($name = $base_default[$b, $e]);
            )?
        )*
    }
}

#[macro_export]
macro_rules! impl_exp_scale{
    ( $name:ty = $base:ty =^ $exp:ty) => {
        impl $crate::unit::scale::exp::ExponentialScale<$base> for $name{
            type ScaleExponent = $exp;
        }
    }
}

#[macro_export]
macro_rules! impl_exp_scale_default{
    ( $name:ty = $base:ty[$b:ty, $e:ty]) => {
        impl $crate::unit::scale::exp::ExponentialScaleDefault for $name{
            type ScaleBase = $base;
            type Base = $b;
            type Exponent = $e;
        }
    }
}