#[macro_export]
macro_rules! impl_unit_div{
    ($unit_lhs:ty |/ $unit_rhs:ty => $unit_div:ty) => {
        impl<NLhs, NRhs> $crate::convert::DivMeasure<$unit_rhs, NLhs, NRhs> for $unit_lhs
        where
            NLhs: core::ops::Div<NRhs>,
        {
            type Output = $unit_div;
        }
    }
}