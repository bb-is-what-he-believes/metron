use crate::convert::FromUnit;
use crate::{unit::Unit};
use num_traits::Pow;
use std::cmp::Ordering;
use std::ops::Mul;
use crate::unit::scale::Scale;


pub trait ExponentialScaledUnit {
    type ExponentialScale: ExponentialScale;
    type BaseUnit: Unit;
}
pub trait ExponentialScale {
    type ScaleBase: ScaleBase;
    type ScaleExponent: ScaleExponent;
}
pub trait ScaleBase {
    type Base;
    const BASE: Self::Base;
}
pub trait ScaleExponent {
    type Exponent;
    const EXPONENT: Self::Exponent;
}
impl<UFrom, UInto, N> FromUnit<N, UFrom> for UInto
where
    UFrom: Unit + ExponentialScaledUnit<
        ExponentialScale: Scale + ExponentialScale<
            ScaleBase = <<UInto as ExponentialScaledUnit>::ExponentialScale as ExponentialScale>::ScaleBase,
            ScaleExponent: ScaleExponent<
                Exponent = i8,
            >
        >
    >,
    UInto: Unit + ExponentialScaledUnit<
        ExponentialScale: Scale + ExponentialScale<
            ScaleBase: ScaleBase<
                Base = f64,
            >,
            ScaleExponent: ScaleExponent<
                Exponent = i8,
            >
        >
    >,
    N: Mul<f64, Output =N> + From<f64>,
{
    type Output = N;
    fn from_unit(num: N) -> Self::Output {
        let from_exp =
            <<<UFrom as ExponentialScaledUnit>
                ::ExponentialScale as ExponentialScale>
                    ::ScaleExponent as ScaleExponent>
                        ::EXPONENT;
        let into_exp =
            <<<UInto as ExponentialScaledUnit>
                ::ExponentialScale as ExponentialScale>
                    ::ScaleExponent as ScaleExponent>
                        ::EXPONENT;
        let ordering = {
            let opt = from_exp.partial_cmp(&into_exp);
            if opt.is_none() {
                // anyhow::bail!("Can't compare exponents");
                panic!("Can't compare exponents")
            }
            opt.unwrap()
        };
        let num = match ordering {
            Ordering::Equal => num.into(),
            _ => {
                let base = <<<UInto as ExponentialScaledUnit>
                    ::ExponentialScale as ExponentialScale>
                        ::ScaleBase as ScaleBase>::BASE;
                let exp = from_exp - into_exp;
                let ratio = base.pow(exp);
                num * ratio
            }
        };
        num
    }
}
pub struct ScaleBaseP10;
impl ScaleBase for ScaleBaseP10 {
    type Base = f64;
    const BASE: f64 = 10.0;
}
macro_rules! def_exponent{
    ($name:ident, $num:literal) => {
        pub struct $name;
        impl ScaleExponent for $name {
            type Exponent = i8;
            const EXPONENT: i8 = $num;
        }
    };
    ($name:ident, -$num:literal) => {
        pub struct $name;
        impl ScaleExponent for $name {
            type Exponent = i8;
            const EXPONENT: i8 = -$num;
        }
    };
}
def_exponent!(ScaleExponentP24,  24);
def_exponent!(ScaleExponentP21,  21);
def_exponent!(ScaleExponentP18,  18);
def_exponent!(ScaleExponentP15,  15);
def_exponent!(ScaleExponentP12,  12);
def_exponent!(ScaleExponentP9 ,   9);
def_exponent!(ScaleExponentP6 ,   6);
def_exponent!(ScaleExponentP3 ,   3);
def_exponent!(ScaleExponentP2 ,   2);
def_exponent!(ScaleExponentP1 ,   1);
def_exponent!(ScaleExponentZ0 ,   0);
def_exponent!(ScaleExponentM1 ,  -1);
def_exponent!(ScaleExponentM2 ,  -2);
def_exponent!(ScaleExponentM3 ,  -3);
def_exponent!(ScaleExponentM6 ,  -6);
def_exponent!(ScaleExponentM9 ,  -9);
def_exponent!(ScaleExponentM12, -12);
def_exponent!(ScaleExponentM15, -15);
def_exponent!(ScaleExponentM18, -18);
def_exponent!(ScaleExponentM21, -21);
def_exponent!(ScaleExponentM24, -24);

#[cfg(test)]
mod test {
    use crate::fmt::Formatter;
    use crate::unit::scale::exp_old::{
        ExponentialScale, ExponentialScaledUnit, Scale, ScaleBase, ScaleBaseP10, ScaleExponent,
        ScaleExponentZ0, ScaleExponentM3, ScaleExponentP3, ScaleExponentP6,
    };
    use crate::unit::Unit;
    use crate::Measure;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    #[test]
    fn empty() {
        assert!(true)
    }

    struct SMilli;
    impl SMilli {
        const SYMBOL: Lazy<&'static str> = Lazy::new(|| "m");
        const SYMBOL_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
            let mut map = HashMap::new();
            map.insert("short", "m");
            map.insert("long", "milli");
            map
        });
    }
    impl Scale for SMilli {
        fn symbol(_fmt: &Formatter) -> &'static str {
            &Self::SYMBOL_MAP.get("short").unwrap()
        }
    }
    impl ExponentialScale for SMilli {
        type ScaleBase = ScaleBaseP10;
        type ScaleExponent = ScaleExponentM3;
    }

    struct SOne;
    impl Scale for SOne {
        fn symbol(_fmt: &Formatter) -> &'static str {
            ""
        }
    }
    impl ExponentialScale for SOne {
        type ScaleBase = ScaleBaseP10;
        type ScaleExponent = ScaleExponentZ0;
    }

    struct SKilo;
    impl SKilo {
        const SYMBOL: Lazy<&'static str> = Lazy::new(|| "k");
    }
    impl Scale for SKilo {
        fn symbol(_fmt: &Formatter) -> &'static str {
            "k"
        }
    }

    impl ExponentialScale for SKilo {
        type ScaleBase = ScaleBaseP10;
        type ScaleExponent = ScaleExponentP3;
    }

    struct SMega;
    impl Scale for SMega {
        fn symbol(_fmt: &Formatter) -> &'static str {
            "M"
        }
    }
    impl ExponentialScale for SMega {
        type ScaleBase = ScaleBaseP10;
        type ScaleExponent = ScaleExponentP6;
    }

    struct UMeter;
    impl UMeter {
        const SYMBOL: Lazy<&'static str> = Lazy::new(|| "m");
    }
    impl Unit for UMeter {
        fn symbol(_fmt: &Formatter) -> &'static str {
            "m"
        }
    }
    impl ExponentialScaledUnit for UMeter {
        type ExponentialScale = SOne;
        type BaseUnit = UMeter;
    }
    struct UKiloMeter;
    impl UKiloMeter {
        const SYMBOL: Lazy<&'static str> = Lazy::new(|| {
            static SYMBOL: Lazy<String> =
                Lazy::new(|| format!("{}{}", &*SKilo::SYMBOL, &*UMeter::SYMBOL));
            &SYMBOL
        });
    }
    impl Unit for UKiloMeter {
        fn symbol(_fmt: &Formatter) -> &'static str {
            &*Self::SYMBOL
        }
    }
    impl ExponentialScaledUnit for UKiloMeter {
        type ExponentialScale = SKilo;
        type BaseUnit = UMeter;
    }
    struct UMilliMeter;
    impl UMilliMeter {
        const SYMBOL: Lazy<&'static str> = Lazy::new(|| {
            static SYMBOL: Lazy<String> =
                Lazy::new(|| format!("{}{}", &*SMilli::SYMBOL, &*UMeter::SYMBOL));
            &SYMBOL
        });
    }
    impl Unit for UMilliMeter {
        fn symbol(_fmt: &Formatter) -> &'static str {
            &Self::SYMBOL
        }
    }
    impl ExponentialScaledUnit for UMilliMeter {
        type ExponentialScale = SMilli;
        type BaseUnit = UMeter;
    }

    type MilliMeter<N> = Measure<N, UMilliMeter>;
    type Meter<N> = Measure<N, UMeter>;
    type KiloMeter<N> = Measure<N, UKiloMeter>;
    #[test]
    fn kilo_from_scale_one() {
        let meter = Meter::new(1000.0);
        let kilo_meter = KiloMeter::from_unit(meter);
        assert_eq!(1.0, kilo_meter.into_inner());
    }
    #[test]
    fn one_from_scale_kilo() {
        let kilo_meter = KiloMeter::new(1.0);
        let meter = Meter::from_unit(kilo_meter);
        assert_eq!(1000.0, meter.into_inner());
    }
    #[test]
    fn milli_into_scale_one() {
        let milli_meter = MilliMeter::new(1000.0);
        let meter: Meter<_> = milli_meter.into_unit();
        assert_eq!(1.0, meter.into_inner());
    }
    #[test]
    fn one_into_scale_milli() {
        let meter = Meter::new(1.0);
        let milli_meter: MilliMeter<_> = meter.into_unit();
        assert_eq!(1000.0, milli_meter.into_inner());
    }
}
