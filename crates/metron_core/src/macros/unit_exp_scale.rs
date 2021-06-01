#[macro_export]
macro_rules! impl_unit_exp_scale {
    ( $unit_self:ty , ( $exp_scale:ty ) ) => {
        $crate::impl_unit_exp_scale!($exp_scale |: $unit_self => $unit_self);
    };
    ( $exp_scale:ty |: $unit_base:ty => $unit_self:ty ) => {
        impl metron_core::unit::scale::exp::ExponentialScaledUnit for $unit_self {
            type ExponentialScale = $exp_scale;
            type BaseUnit = $unit_base;
        }
    };
}
