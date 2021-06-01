use crate::convert::FromUnit;
use crate::{Measure, Unit};

pub trait Quantity {
    type BaseUnit: Unit;
}

pub fn from_unit<N, U, Q>(
    unit: Measure<N, U>,
) -> Measure<<<Q as Quantity>::BaseUnit as FromUnit<N, U>>::Output, Q>
where
    N: ,
    U: Unit,
    Q: Quantity,
    <Q as Quantity>::BaseUnit: FromUnit<N, U>,
{
    Measure::new(<<Q as Quantity>::BaseUnit as FromUnit<N, U>>::from_unit(
        unit.num,
    ))
}
