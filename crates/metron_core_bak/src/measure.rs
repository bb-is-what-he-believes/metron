pub struct Measure<N, S, B, E, U>
where
    S: Scale,
    U: Unit,
{
    num: N,
    _scale: PhantomData<S>,
    _base: PhantomData<B>,
    _exponent: PhantomData<E>,
    _unit: PhantomData<U>,
}

pub trait TryFromScale<N, SFrom, SInto, SBase, B, E, U>
where
    SFrom: Scale,
    SInto: Scale,
    SBase: ScaleBase<B>,
    U: Unit,
{
    fn try_from_scale(from: Measure<N, SFrom, B, E, U>) -> anyhow::Result<Self>
    where
        Self: Sized;
}
pub trait TryIntoScale<N, SFrom, SInto, SBase, B, E, U>
where
    SFrom: Scale,
    SInto: Scale,
    SBase: ScaleBase<B>,
    U: Unit,
{
    fn try_into_scale(self) -> anyhow::Result<Measure<N, SFrom, B, E, U>>
    where
        Self: Sized;
}

impl<N, SFrom, SInto, SBase, B, E, U> TryFromScale<N, SFrom, SInto, SBase, B, E, U>
    for Measure<N, SInto, B, E, U>
where
    SBase: ScaleBase<B>,
    SFrom: Scale + ExponentialScale<SBase, B, E>,
    SInto: Scale + ExponentialScale<SBase, B, E>,
    N: Mul<N, Output = N> + Pow<N, Output = N>,
    B: Into<N>,
    E: PartialOrd + Sub<E, Output = E> + Into<N>,
    U: Unit,
    Self: Sized,
{
    fn try_from_scale(from: Measure<N, SFrom, B, E, U>) -> anyhow::Result<Self> {
        try_convert_scale(from)
    }
}
impl<N, SFrom, SInto, SBase, B, E, U> TryIntoScale<N, SFrom, SInto, SBase, B, E, U>
    for Measure<N, SInto, B, E, U>
where
    SBase: ScaleBase<B>,
    SFrom: Scale + ExponentialScale<SBase, B, E>,
    SInto: Scale + ExponentialScale<SBase, B, E>,
    N: Mul<N, Output = N> + Pow<N, Output = N>,
    B: Into<N>,
    E: PartialOrd + Sub<E, Output = E> + Into<N>,
    U: Unit,
    Self: Sized,
{
    fn try_into_scale(self) -> anyhow::Result<Measure<N, SFrom, B, E, U>>
    where
        Self: Sized,
    {
        try_convert_scale(self)
    }
}

fn try_convert_scale<N, SFrom, SInto, SBase, B, E, U>(
    from: Measure<N, SFrom, B, E, U>,
) -> anyhow::Result<Measure<N, SInto, B, E, U>>
where
    SBase: ScaleBase<B>,
    SFrom: Scale + ExponentialScale<SBase, B, E>,
    SInto: Scale + ExponentialScale<SBase, B, E>,
    N: Mul<N, Output = N> + Pow<N, Output = N>,
    B: Into<N>,
    E: PartialOrd + Sub<E, Output = E> + Into<N>,
    U: Unit,
{
    let from_exp = <SFrom as ExponentialScale<SBase, B, E>>::ScaleExponent::EXPONENT;
    let into_exp = <SInto as ExponentialScale<SBase, B, E>>::ScaleExponent::EXPONENT;
    let ordering = {
        let opt = from_exp.partial_cmp(&into_exp);
        if opt.is_none() {
            anyhow::bail!("Can't compare exponents");
        }
        opt.unwrap()
    };
    let num = match ordering {
        Ordering::Equal => from.num.into(),
        _ => {
            let base: N = SBase::BASE.into();
            let exp: N = (from_exp - into_exp).into();
            from.num * base.pow(exp)
        }
    };
    Ok(Measure::new(num))
}

use crate::{ExponentialScale, Scale, ScaleBase, ScaleExponent, Unit};
use core::fmt;
use core::ops::Add;
use num_traits::Pow;
use once_cell::sync::Lazy;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{AddAssign, Div, Mul, Sub};

impl<N, S: Scale, B, E, U: Unit> Measure<N, S, B, E, U> {
    const SYMBOL: Lazy<String> = Lazy::new(|| S::SYMBOL.to_string() + *U::SYMBOL);
    fn new(num: N) -> Self {
        Self {
            num,
            _scale: PhantomData,
            _base: PhantomData,
            _exponent: PhantomData,
            _unit: PhantomData,
        }
    }

    fn into_inner(self) -> N {
        self.num
    }
}
impl<N, S: Scale, B, E, U: Unit> From<N> for Measure<N, S, B, E, U> {
    fn from(num: N) -> Self {
        Self::new(num)
    }
}
impl<N, S: Scale, B, E, U: Unit> Debug for Measure<N, S, B, E, U>
where
    N: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use core::any::type_name;
        f.debug_tuple(&format!(
            "Measure<{}, {}, {}>",
            type_name::<N>(),
            type_name::<S>(),
            type_name::<U>()
        ))
        .field(&self.num)
        .finish()
    }
}
impl<N, S: Scale, B, E, U: Unit> Display for Measure<N, S, B, E, U>
where
    N: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", &self.num, *Self::SYMBOL)
    }
}

impl<NOther, NSelf, S, B, E, U> Add<Measure<NOther, S, B, E, U>> for Measure<NSelf, S, B, E, U>
where
    NSelf: Add<NOther>,
    S: Scale,
    U: Unit,
{
    type Output = Measure<<NSelf as Add<NOther>>::Output, S, B, E, U>;
    fn add(self, rhs: Measure<NOther, S, B, E, U>) -> Self::Output {
        Measure::new(self.num + rhs.num)
    }
}

impl<NOther, NSelf, S, B, E, U> AddAssign<Measure<NOther, S, B, E, U>>
    for Measure<NSelf, S, B, E, U>
where
    NSelf: AddAssign<NOther>,
    S: Scale,
    U: Unit,
{
    fn add_assign(&mut self, rhs: Measure<NOther, S, B, E, U>) {
        self.num += rhs.num
    }
}

impl<NOther, NSelf, S, B, E, U> Sub<Measure<NOther, S, B, E, U>> for Measure<NSelf, S, B, E, U>
where
    NSelf: Sub<NOther>,
    S: Scale,
    U: Unit,
{
    type Output = Measure<<NSelf as Sub<NOther>>::Output, S, B, E, U>;
    fn sub(self, rhs: Measure<NOther, S, B, E, U>) -> Self::Output {
        Measure::new(self.num - rhs.num)
    }
}

impl<NOther, NSelf, S, B, E, U> Mul<NOther> for Measure<NSelf, S, B, E, U>
where
    NSelf: Mul<NOther>,
    S: Scale,
    U: Unit,
{
    type Output = Measure<<NSelf as Mul<NOther>>::Output, S, B, E, U>;
    fn mul(self, rhs: NOther) -> Self::Output {
        Measure::new(self.num * rhs)
    }
}

impl<NOther, NSelf, S, B, E, U> Div<NOther> for Measure<NSelf, S, B, E, U>
where
    NSelf: Div<NOther>,
    S: Scale,
    U: Unit,
{
    type Output = Measure<<NSelf as Div<NOther>>::Output, S, B, E, U>;
    fn div(self, rhs: NOther) -> Self::Output {
        Measure::new(self.num / rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::measure::{Measure, TryFromScale, TryIntoScale};
    use crate::{ExponentialScale, Scale, ScaleBase, ScaleExponent, Unit};
    use once_cell::sync::Lazy;

    pub struct ScaleBase10;
    impl ScaleBase<u8> for ScaleBase10 {
        const BASE: u8 = 10;
    }
    pub struct ScaleExponentM3;
    impl ScaleExponent<i8> for ScaleExponentM3 {
        const EXPONENT: i8 = -3;
    }
    pub struct ScaleExponent0;
    impl ScaleExponent<i8> for ScaleExponent0 {
        const EXPONENT: i8 = 0;
    }
    pub struct ScaleExponentP3;
    impl ScaleExponent<i8> for ScaleExponentP3 {
        const EXPONENT: i8 = 3;
    }
    pub struct ScaleExponentP6;
    impl ScaleExponent<i8> for ScaleExponentP6 {
        const EXPONENT: i8 = 6;
    }

    struct SMilli;
    impl Scale for SMilli {
        const SYMBOL: Lazy<&'static str> = Lazy::new(|| "m");
    }
    impl ExponentialScale<ScaleBase10, u8, i8> for SMilli {
        type ScaleExponent = ScaleExponentM3;
    }

    struct SOne;
    impl Scale for SOne {
        const SYMBOL: Lazy<&'static str> = Lazy::new(|| "");
    }
    impl ExponentialScale<ScaleBase10, u8, i8> for SOne {
        type ScaleExponent = ScaleExponent0;
    }

    struct SKilo;
    impl Scale for SKilo {
        const SYMBOL: Lazy<&'static str> = Lazy::new(|| "k");
    }
    impl ExponentialScale<ScaleBase10, u8, i8> for SKilo {
        type ScaleExponent = ScaleExponentP3;
    }

    struct SMega;
    impl Scale for SMega {
        const SYMBOL: Lazy<&'static str> = Lazy::new(|| "M");
    }
    impl ExponentialScale<ScaleBase10, u8, i8> for SMega {
        type ScaleExponent = ScaleExponentP6;
    }

    struct UMeter;
    impl Unit for UMeter {
        const SYMBOL: Lazy<&'static str> = Lazy::new(|| "m");
    }

    type MilliMeter<N> = Measure<N, SMilli, u8, i8, UMeter>;
    type Meter<N> = Measure<N, SOne, u8, i8, UMeter>;
    type KiloMeter<N> = Measure<N, SKilo, u8, i8, UMeter>;

    #[test]
    fn debug() {
        let one_kilo_meter = Meter::new(1);
        println!("{:?}", &one_kilo_meter)
    }
    #[test]
    fn format_default() {
        let one_kilo_meter = KiloMeter::new(1);
        assert_eq!(format!("{}", &one_kilo_meter), "1km");
        let one_kilo_meter = KiloMeter::new(1.1);
        assert_eq!(format!("{}", &one_kilo_meter), "1.1km");
    }
    #[test]
    fn add() {
        let one = Meter::new(1);
        let two = Meter::new(2);
        let three = one + two;
        assert_eq!(3, three.into_inner());
    }
    #[test]
    fn sub() {
        let three = Meter::new(3);
        let two = Meter::new(2);
        let one = three - two;
        assert_eq!(1, one.into_inner());
    }
    #[test]
    fn mul_num() {
        let two = Meter::new(2);
        let three = 3;
        let six = two * three;
        assert_eq!(6, six.into_inner());
    }
    #[test]
    fn div_num() {
        let six = KiloMeter::new(6);
        let three = 3;
        let two = six / three;
        assert_eq!(2, two.into_inner());
    }
    #[test]
    fn from() {
        let one = KiloMeter::from(1);
        assert_eq!(1, one.into_inner());
    }
    #[test]
    fn into() {
        let one: KiloMeter<_> = 1.into();
        assert_eq!(1, one.into_inner());
    }
    #[test]
    fn kilo_try_from_scale_one() {
        let meter = Meter::new(1000.0);
        let kilo_meter = KiloMeter::<f32>::try_from_scale(meter).unwrap();
        assert_eq!(1.0, kilo_meter.into_inner());
    }
    #[test]
    fn one_try_from_scale_kilo() {
        let kilo_meter = KiloMeter::new(1.0);
        let meter = Meter::<f32>::try_from_scale(kilo_meter).unwrap();
        assert_eq!(1000.0, meter.into_inner());
    }
    #[test]
    fn milli_try_into_scale_one() {
        let milli_meter = MilliMeter::new(1000.0);
        let meter: Meter<_> = milli_meter.try_into_scale().unwrap();
        assert_eq!(1.0, meter.into_inner());
    }
    #[test]
    fn one_try_into_scale_milli() {
        let meter = Meter::new(1.0);
        let milli_meter: MilliMeter<_> = meter.try_into_scale().unwrap();
        assert_eq!(1000.0, milli_meter.into_inner());
    }
}
