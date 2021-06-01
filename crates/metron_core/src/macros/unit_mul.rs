#[macro_export]
macro_rules! impl_unit_mul{
    ($unit_lhs:ty |* $unit_rhs:ty => $unit_mul:ty) => {
        impl<NLhs, NRhs> $crate::convert::MulMeasure<$unit_rhs, NLhs, NRhs> for $unit_lhs
        where
            NLhs: core::ops::Mul<NRhs>,
        {
            type Output = $unit_mul;
        }
    }
}