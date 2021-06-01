#[macro_export]
macro_rules! impl_fmt_symbol{
    ($type:ty , ( $symbol:literal ) ) => {
        $crate::impl_fmt_symbol!( $type |: () => $symbol );
    };
    ($type:ty |: $lang:ty => $symbol:literal) => {
        impl $crate::fmt::Symbol<$lang> for $type{
            fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result{
                write!(f, "{}", $symbol)
            }
        }
    };
}