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
        $vis type $alias<N> = $crate::measure::Measure<N, $type>;
    };
    ( $vis:vis $alias:ident<$num:ty> = $type:ty ) => {
        $vis type $alias = $crate::measure::Measure<$num, $type>;
    };
    ( $vis:vis $alias:ident = $type:ty ) => {
        $vis type $alias<N> = $crate::measure::Measure<N, $type>;
    };
}
