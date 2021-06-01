#[macro_export]
macro_rules! impl_quantity_from{
    ($quantity:ty, ( $unit:ty ) ) => {
        impl<N> $crate::convert::FromUnit<N, $unit> for $quantity
        where
            <Self as $crate::quantity::Quantity>::BaseUnit: $crate::convert::FromUnit<N, $unit>,
        {
            type Output = 
                <<Self as $crate::quantity::Quantity>::BaseUnit as $crate::convert::FromUnit<N, $unit, >>::Output;
            fn from_unit(num: N) -> 
                <<Self as $crate::quantity::Quantity>::BaseUnit as $crate::convert::FromUnit<N, $unit, >>::Output
            {
                <<Self as $crate::quantity::Quantity>::BaseUnit as $crate::convert::FromUnit<N, $unit, >>::from_unit(num)
            }
        }
    };
}