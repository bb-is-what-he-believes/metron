// use crate::convert::FromUnit;
use crate::{Measure, Unit};
use num_traits::Pow;
use std::cmp::Ordering;
use std::ops::{Mul, Sub};

pub trait ExponentialScaledUnit {
    type ExponentialScale;
    type BaseUnit: Unit;
}
pub trait ExponentialScaleDefault
where
    Self: ExponentialScale<Self::ScaleBase>,
    Self::ScaleBase: ScaleBase<Self::Base>,
    <Self as ExponentialScale<Self::ScaleBase>>::ScaleExponent: ScaleExponent<Self::Exponent>,
    Self::Base: Pow<<Self::Exponent as Sub<Self::Exponent>>::Output>,
    Self::Exponent: PartialOrd<Self::Exponent> + Sub<Self::Exponent>,
{
    type ScaleBase;
    type Base;
    type Exponent;
}
pub trait ExponentialScale<SB> {
    type ScaleExponent;
}
pub trait ScaleBase<B> {
    const BASE: B;
}
pub trait ScaleExponent<E> {
    const EXPONENT: E;
}

macro_rules! exp_scale_def {
    ($unit:ident, $name:ident) => {
        <<$unit as ExponentialScaledUnit>::ExponentialScale as ExponentialScaleDefault>::$name
    };
}

impl<N, USelf> Measure<N, USelf>
where
    USelf
        : Unit
        + ExponentialScaledUnit,
    <USelf as ExponentialScaledUnit>::ExponentialScale
        : ExponentialScale<exp_scale_def!(USelf, ScaleBase)>
        + ExponentialScaleDefault,
    <<USelf as ExponentialScaledUnit>
            ::ExponentialScale as ExponentialScale<exp_scale_def!(USelf, ScaleBase)>>
            ::ScaleExponent
        : ScaleExponent<exp_scale_def!(USelf, Exponent)>,
    exp_scale_def!(USelf, ScaleBase)
        : ScaleBase<exp_scale_def!(USelf, Base)>,
    exp_scale_def!(USelf, Base)
        : Pow<<exp_scale_def!(USelf, Exponent) as Sub<exp_scale_def!(USelf, Exponent)>>::Output>,
    exp_scale_def!(USelf, Exponent)
        : PartialOrd<exp_scale_def!(USelf, Exponent)>
        + Sub<exp_scale_def!(USelf, Exponent)>,
    N   : Mul<
            <exp_scale_def!(USelf, Base) as Pow<
                <exp_scale_def!(USelf, Exponent) as Sub>::Output,
            >>::Output,
            Output = N,
        >,
{
    pub fn from_scale<UFrom>(from: Measure<N, UFrom>) -> Self
    where
        UFrom
            : Unit
            + ExponentialScaledUnit<
                BaseUnit
                    = <USelf as ExponentialScaledUnit>::BaseUnit,
                ExponentialScale
                    : ExponentialScale<
                        exp_scale_def!(USelf, ScaleBase),
                        ScaleExponent: ScaleExponent<exp_scale_def!(USelf, Exponent)>,
                    >
                    + ExponentialScaleDefault<
                        ScaleBase = exp_scale_def!(USelf, ScaleBase),
                        Base = exp_scale_def!(USelf, Base),
                        Exponent = exp_scale_def!(USelf, Exponent),
                    >,
            >,
    {
        Self::from_scale_spec::<
            UFrom,
            exp_scale_def!(UFrom, ScaleBase),
            exp_scale_def!(UFrom, Base),
            exp_scale_def!(UFrom, Exponent),
        >(from)
    }
}
impl<N, USelf> Measure<N, USelf>
where
    USelf: Unit + ExponentialScaledUnit,
{
    fn from_scale_spec<UFrom, SB, B, E>(from: Measure<N, UFrom>) -> Self
    where
        USelf
            : Unit
            + ExponentialScaledUnit<
                ExponentialScale: ExponentialScale<SB, ScaleExponent: ScaleExponent<E>>,
            >,
        UFrom
            : Unit
            + ExponentialScaledUnit<
                BaseUnit = <USelf as ExponentialScaledUnit>::BaseUnit,
                ExponentialScale: ExponentialScale<SB, ScaleExponent: ScaleExponent<E>>,
            >,
        SB  : ScaleBase<B>,
        E   : PartialOrd<E> + Sub<E>,
        B   : Pow<<E as Sub<E>>::Output>,
        N   : Mul<<B as Pow<<E as Sub>::Output>>::Output, Output = N>,
    {
        let from_exp = <<<UFrom as ExponentialScaledUnit>::ExponentialScale as ExponentialScale<
            SB,
        >>::ScaleExponent as ScaleExponent<E>>::EXPONENT;
        let into_exp = <<<USelf as ExponentialScaledUnit>::ExponentialScale as ExponentialScale<
            SB,
        >>::ScaleExponent as ScaleExponent<E>>::EXPONENT;
        let ordering = {
            let opt = from_exp.partial_cmp(&into_exp);
            if opt.is_none() {
                // anyhow::bail!("Can't compare exponents");
                panic!("Can't compare exponents")
            }
            opt.unwrap()
        };
        let num = match ordering {
            Ordering::Equal => from.num.into(),
            _ => {
                let base = <SB as ScaleBase<B>>::BASE;
                let exp = from_exp - into_exp;
                let ratio = base.pow(exp);
                from.num * ratio
            }
        };
        Self::new(num)
    }
}
#[cfg(test)]
mod test {
    mod units {
        use crate::unit::scale::exp::{
            ExponentialScale, ExponentialScaleDefault, ExponentialScaledUnit, Measure,
        };
        use crate::Unit;
        use crate::{def_exp_scale, def_exp_scale_base, def_exp_scale_exp};

        def_exp_scale! {
            pub    One[ BP10 =^ E0, BP1000 =^ E0, ],
        }
        def_exp_scale! {
            pub   Kilo[ BP10 =^  EP3, BP1000 =^ EP1, ],
            pub  Milli[ BP10 =^  EM3, BP1000 =^ EM1, ],
        }
        def_exp_scale_base! {
            pub BP1000[f32 = 1000.0, f64 = 1000.0,],
            pub   BP10[f32 =   10.0, f64 =   10.0,],
        }
        def_exp_scale_exp! {
            pub    EP3[f32 =    3.0, f64 =    3.0,],
            pub    EP1[f32 =    1.0, f64 =    1.0,],
            pub     E0[f32 =    0.0, f64 =    0.0,],
            pub    EM1[f32 = -  1.0, f64 = -  1.0,],
            pub    EM3[f32 = -  3.0, f64 = -  3.0,],
        }
        impl ExponentialScaleDefault for Milli {
            type ScaleBase = BP1000;
            type Base = f32;
            type Exponent = f32;
        }
        impl ExponentialScaleDefault for Kilo {
            type ScaleBase = BP1000;
            type Base = f32;
            type Exponent = f32;
        }
        pub struct Meter;
        impl Unit for Meter {}
        pub struct MilliMeter;
        impl Unit for MilliMeter{}
        impl ExponentialScaledUnit for MilliMeter {
            type ExponentialScale = Milli;
            type BaseUnit = Meter;
        }
        pub struct KiloMeter;
        impl Unit for KiloMeter{}
        impl ExponentialScaledUnit for KiloMeter {
            type ExponentialScale = Kilo;
            type BaseUnit = Meter;
        }
    }
    use crate::unit::scale::exp::Measure;
    type MilliMeter<N> = Measure<N, units::MilliMeter>;
    type KiloMeter<N> = Measure<N, units::KiloMeter>;

    #[test]
    fn test() {
        let milli_meter = MilliMeter::new(1000.0);
        let kilo_meter: KiloMeter<_> = KiloMeter::from_scale(milli_meter);
        println!("{:?}", kilo_meter);
    }
}
