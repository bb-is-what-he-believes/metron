// #![feature(once_cell)]
pub use once_cell::sync::Lazy;
// pub use std::lazy::Lazy;

pub trait Unit {
    const SYMBOL: Lazy<&'static str>;
}
pub trait Scale {
    const SYMBOL: Lazy<&'static str>;
}
pub trait ExponentialScale<Base, B, E>
where
    Base: ScaleBase<B>,
{
    type ScaleExponent: ScaleExponent<E>;
}
pub trait ScaleBase<B> {
    const BASE: B;
}
pub trait ScaleExponent<E> {
    const EXPONENT: E;
}
pub trait Quantity {
    type BaseUnit: Unit;
}

pub use measure::Measure;
pub mod measure;
