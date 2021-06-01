#[macro_export]
macro_rules! def_measure{
    (
        $(
        $vis:vis $alias:ident : $type:ty ,
        )+
    ) => {
        $(
        $crate::def_measure!($vis $alias : $type );
        )+
    };
    ( $vis:vis $alias:ident : $type:ty ) => {
        $vis type $alias<N> = metron_core::measure::Measure<N, $type>;
    };
}
