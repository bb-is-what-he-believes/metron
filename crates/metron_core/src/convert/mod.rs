use std::ops::{Div, Mul};

use crate::{Unit, Quantity};
use crate::Measure;

/// Measurement conversion from some unit.
///
///
///
///
pub trait FromUnit<N, U: Unit>
{
    type Output;
    fn from_unit(num: N) -> Self::Output;
}

impl<NSelf, MSelf> Measure<NSelf, MSelf> {
    pub fn from_unit<NFrom, UFrom>(from: Measure<NFrom, UFrom>) -> Self
        where
            MSelf: FromUnit<NFrom, UFrom, Output = NSelf>,
            UFrom: Unit,
    {
        Self::new(MSelf::from_unit(from.num))
    }

    pub fn unit_into<NInto, UInto>(self) -> Measure<NInto, UInto>
        where
            MSelf: Unit,
            UInto: Unit + FromUnit<NSelf, MSelf, Output = NInto>,
    {
        Measure::new(UInto::from_unit(self.num))
    }
}

/// Measurement conversion from some quantity.
///
///
///
///
///
pub trait FromQuantity<N, Q: Quantity>
{
    type Output;
    fn from_quantity(num: N) -> Self::Output;
}

///
///
///
///
///
///
pub trait IntoMeasure {
    type Num;
    type Unit : Unit;
    fn into_measure(self) -> Measure<Self::Num, Self::Unit>  where Self: Sized;
}

impl <N, U: Unit> IntoMeasure for Measure<N, U>{
    type Num = N;
    type Unit = U;

    fn into_measure(self) -> Measure<Self::Num, Self::Unit> where Self: Sized {
        Measure::new(self.num)
    }
}

macro_rules! impl_into_measure{
    ( $( $type:ty ,)+ ) => {
        $(
        impl IntoMeasure for $type{
            type Num = $type;
            type Unit = ();
            fn into_measure(self) -> Measure<Self::Num, Self::Unit> where Self: Sized {
                Measure::new(self)
            }
        }
        )+
    };
}

impl_into_measure!{
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
             f32, f64,
}

///
///
///
///
///
///
pub trait MulMeasure<URhs: Unit, NSelf, NRhs = NSelf>
where
    Self: Unit,
    NSelf: Mul<NRhs>,
{
    type Output: Unit;
    fn mul_measure(num: NSelf, rhs: NRhs) -> <NSelf as Mul<NRhs>>::Output {
        num * rhs
    }
}

impl<USelf : Unit, NSelf: Mul<NRhs>, NRhs> MulMeasure<(), NSelf, NRhs> for USelf{
    type Output = USelf;
}

///
///
///
///
///
///
pub trait DivMeasure<URhs: Unit, NSelf, NRhs = NSelf>
where
    Self: Unit,
    NSelf: Div<NRhs>,
{
    type Output: Unit;
    fn div_unit(num: NSelf, rhs: NRhs) -> <NSelf as Div<NRhs>>::Output {
        num / rhs
    }
}


impl<USelf : Unit, NSelf: Div<NRhs>, NRhs> DivMeasure<(), NSelf, NRhs> for USelf{
    type Output = USelf;
}