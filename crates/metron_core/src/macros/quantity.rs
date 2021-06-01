/// Generate define of Quantity
///
/// # Examples
/// Define Length
///
/// ```
/// use metron_core::{def_unit, def_quantity};
/// def_unit!{pub Metre{ from (Feet =*0.3048), }}
/// def_unit!{pub Feet;}
///
/// def_quantity!{
///     /// Length is a measure of distance.
///     pub Length<Metre> {
///         from( Feet ),
///     }
/// }
///
/// ```
/// if you write abobe
/// macro will generates like below
/// ```
/// use metron_core::{def_unit, Quantity, FromUnit};
/// def_unit!{pub Metre{ from (Feet =* 0.3048), }}
/// def_unit!{pub Feet;}
///
/// /// Length is a measure of distance.
/// pub struct Length;
///
/// impl metron_core::Quantity for Length {
///     type BaseUnit = Metre;
/// }
///
/// impl <N> FromUnit<N, Metre> for Length{
///     type Output = <Metre as FromUnit<N, Metre>>::Output;
///     fn from_unit(num: N) -> Self::Output {
///         /* conversion */
///         num
///     }
/// }
///
/// impl <N> FromUnit<N, Feet> for Length {
///     type Output = <<Self as Quantity>::BaseUnit as FromUnit<N, Metre>>::Output;
///     fn from_unit(num: N) -> Self::Output{
///         /* conversion */
///         num
///     }
/// }
///
/// ```
#[macro_export]
macro_rules! def_quantity{
    (
        $(#[$meta_for_quantity:meta])*
        $vis:vis $quantity:ident
        $(#[$meta_for_base_unit:meta])*
        < $unit:ty >;
    ) => {
        $(#[$meta_for_quantity])*
        $vis struct $quantity;
        impl $crate::quantity::Quantity for $quantity {
            $(#[$meta_for_base_unit])*
            type BaseUnit = $unit;
        }
        $crate::impl_quantity_from! ($quantity, ( $unit ) );
    };
    (
        $(#[$meta_for_quantity:meta])*
        $vis:vis $quantity:ident
        $(#[$meta_for_base_unit:meta])*
        < $unit:ty > {
        $(
            $(#[$meta_for_from:meta])*
            from $from_body:tt ,
        )*
        // $(into $into_body:tt ,)*
        $(sym   $sym_body:tt ,)?
        // $(dim   $dim_body:tt ,)?
    }) => {
        $crate::def_quantity!(
            $(#[$meta_for_quantity])*
            $vis $quantity
            $(#[$meta_for_base_unit])*
            < $unit >;
        );
        $( $crate::impl_quantity_from! ($quantity, $from_body); )*
        // $( $crate::impl_quantity_into! ($quantity, $into_body); )*
        $( $crate::impl_fmt_symbol!    ($quantity,  $sym_body); )?
    };
}