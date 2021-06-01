#[macro_export]
macro_rules! impl_unit_from {
    ( $unit_into:ty, ( $unit_from:ty =* $factor:literal ) ) => {
        $crate::impl_unit_from!( $unit_into, $unit_from =* $factor );
    };
    ( $unit_into:ty, ( $unit_from:ty =/ $factor:literal ) ) => {
        $crate::impl_unit_from!( $unit_into:ty, $unit_from:ty =/ $factor:literal);
    };
    ( $unit_into:ty, ( $unit_from:ty => | $num:ident : $num_from:ty| -> $num_out:ty { $conv:expr } ) ) =>{
        $crate::impl_unit_from!( $unit_into, $unit_from => | $num : $num_from | -> $num_out { $conv });
    };
    ( $unit_into:ty, ( $unit_from:ty where N: $num_from_bound:path => | $num:ident $(:N)?| -> $num_out:ty { $conv:expr } ) ) =>{
        $crate::impl_unit_from!( $unit_into, $unit_from where N: $num_from_bound => | $num | -> $num_out { $conv });
    };
    ( $unit_into:ty, $unit_from:ty =* $factor:literal) => {
        $crate::impl_unit_from!( impl N, $unit_into, $unit_from, N:std::ops::Mul<f32, Output=N>, num, N, num * $factor);
    };
    ( $unit_into:ty, $unit_from:ty =/ $factor:literal) => {
        $crate::impl_unit_from!( impl N, $unit_into, $unit_from, N:std::ops::Div<f32, Output=N>, num, N, num / $factor);
    };
    ( $unit_into:ty, $unit_from:ty => | $num:ident : $num_from:ty| -> $num_out:ty { $conv:expr }) =>{
        $crate::impl_unit_from!( impl  , $unit_into, $unit_from, $num_from, $num, $num_out, $conv);
    };
    ( $unit_into:ty, $unit_from:ty where N: $num_from_bound:path => | $num:ident $(:N)?| -> $num_out:ty { $conv:expr }) =>{
        $crate::impl_unit_from!( impl N, $unit_into, $unit_from, N:$num_from_bound, $num, $num_out, $conv);
    };
    ( impl $($generics:ident)?, $unit_into:ty, $unit_from:ty, $num_from:ty $(:$num_from_bound:path)?, $num:ident, $num_out:ty, $conv:expr) =>{
        impl $(<$generics>)? $crate::convert::FromUnit<$num_from, $unit_from> for $unit_into
        $(where $num_from: $num_from_bound)?
        {
            type Output = $num_out;
            fn from_unit($num: $num_from) -> $num_out{
                $conv
            }
        }
    };
}