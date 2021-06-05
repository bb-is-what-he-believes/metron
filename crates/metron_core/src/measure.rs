use crate::convert::{IntoMeasure, MulMeasure, DivMeasure};
use crate::unit::Unit;
use core::fmt;
use std::fmt::{Debug};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// provide any arithmetic operation for number with measurement.
///
///
///
///
pub struct Measure<N, M> {
    pub(crate) num: N,
    _measure: PhantomData<M>,
}

///
/// impl basic func
///
///
impl<N, M> Measure<N, M> {
    ///
    /// new Measure from number.
    ///
    pub fn new(num: N) -> Self {
        Self {
            num,
            _measure: PhantomData,
        }
    }
    pub fn num(&self) -> &N {
        &self.num
    }
    pub fn into_num(self) -> N {
        self.num
    }
}

impl<N, M> Clone for Measure<N, M>
where
    N: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.num.clone())
    }
}
impl<N, M> Copy for Measure<N, M>
where
    N: Copy,
{

}

impl<N, U: Unit> Debug for Measure<N, U>
where
    N: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use core::any::type_name;
        f.debug_struct("Measure")
            .field("num", &self.num)
            .field("type num", &format!("{}", type_name::<N>()))
            .field("type measure", &format!("{}", type_name::<U>()))
            .finish()
    }
}

impl<N, M> From<N> for Measure<N, M> {
    fn from(num: N) -> Self {
        Self::new(num)
    }
}

impl<NSelf, NOther, U: Unit> Add<Measure<NOther, U>> for Measure<NSelf, U>
where
    NSelf: Add<NOther>,
    U: Unit,
{
    type Output = Measure<<NSelf as Add<NOther>>::Output, U>;
    fn add(self, rhs: Measure<NOther, U>) -> Self::Output {
        Measure::new(self.num + rhs.num)
    }
}

impl<NSelf, NOther, U: Unit> AddAssign<Measure<NOther, U>> for Measure<NSelf, U>
where
    NSelf: AddAssign<NOther>,
    U: Unit,
{
    fn add_assign(&mut self, rhs: Measure<NOther, U>) {
        self.num += rhs.num
    }
}

impl<NSelf, NOther, U: Unit> Sub<Measure<NOther, U>> for Measure<NSelf, U>
where
    NSelf: Sub<NOther>,
    U: Unit,
{
    type Output = Measure<<NSelf as Sub<NOther>>::Output, U>;
    fn sub(self, rhs: Measure<NOther, U>) -> Self::Output {
        Measure::new(self.num - rhs.num)
    }
}

impl<NSelf, NOther, U: Unit> SubAssign<Measure<NOther, U>> for Measure<NSelf, U>
where
    NSelf: SubAssign<NOther>,
    U: Unit,
{
    fn sub_assign(&mut self, rhs: Measure<NOther, U>) {
        self.num -= rhs.num
    }
}

impl<NSelf, USelf, Rhs> Mul<Rhs> for Measure<NSelf, USelf>
where
    Rhs: IntoMeasure,
    NSelf: Mul<<Rhs as IntoMeasure>::Num>,
    USelf: Unit + MulMeasure<<Rhs as IntoMeasure>::Unit, NSelf, <Rhs as IntoMeasure>::Num>,
{
    type Output = Measure<
        <NSelf as Mul<<Rhs as IntoMeasure>::Num>>::Output,
        <USelf as MulMeasure<<Rhs as IntoMeasure>::Unit, NSelf, <Rhs as IntoMeasure>::Num>>::Output,
    >;

    fn mul(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.into_measure();
        let num = self.num * rhs.num;
        Self::Output::new(num)
    }
}

impl<NSelf, MSelf, Rhs> MulAssign<Rhs> for Measure<NSelf, MSelf>
    where
        Rhs: IntoMeasure,
        NSelf
            : Mul<<Rhs as IntoMeasure>::Num, Output = NSelf>
            + MulAssign<<Rhs as IntoMeasure>::Num,>,
        MSelf
            : Unit
            + MulMeasure<
                <Rhs as IntoMeasure>::Unit,
                NSelf,
                <Rhs as IntoMeasure>::Num,
                Output = NSelf,
            >,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        let rhs = rhs.into_measure();
        self.num *= rhs.num;
    }
}

impl<NSelf, USelf, Rhs> Div<Rhs> for Measure<NSelf, USelf>
    where
        Rhs: IntoMeasure,
        NSelf: Div<<Rhs as IntoMeasure>::Num>,
        USelf: Unit + DivMeasure<<Rhs as IntoMeasure>::Unit, NSelf, <Rhs as IntoMeasure>::Num>,
{
    type Output = Measure<
        <NSelf as Div<<Rhs as IntoMeasure>::Num>>::Output,
        <USelf as DivMeasure<<Rhs as IntoMeasure>::Unit, NSelf, <Rhs as IntoMeasure>::Num>>::Output,
    >;

    fn div(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.into_measure();
        let num = self.num / rhs.num;
        Self::Output::new(num)
    }
}

impl<NSelf, MSelf, Rhs> DivAssign<Rhs> for Measure<NSelf, MSelf>
    where
        Rhs: IntoMeasure,
        NSelf
        : Div<<Rhs as IntoMeasure>::Num, Output = NSelf>
        + DivAssign<<Rhs as IntoMeasure>::Num,>,
        MSelf
        : Unit
        + DivMeasure<
            <Rhs as IntoMeasure>::Unit,
            NSelf,
            <Rhs as IntoMeasure>::Num,
            Output = NSelf,
        >,
{
    fn div_assign(&mut self, rhs: Rhs) {
        let rhs = rhs.into_measure();
        self.num /= rhs.num;
    }
}

#[cfg(test)]
mod tests {
    use crate::unit::Unit;
    use crate::Measure;
    use std::fmt::Formatter;

    struct USome;
    impl Unit for USome {}
    type SomeUnit<N> = Measure<N, USome>;

    #[test]
    fn debug() {
        let some = SomeUnit::new(1);
        println!("{:?}", &some)
    }
    #[test]
    fn format_default() {
        impl crate::fmt::Symbol<()> for USome {
            fn fmt(f: &mut Formatter<'_>) -> core::fmt::Result {
                f.write_str("unit")
            }
        }
        let some = SomeUnit::new(1);
        assert_eq!(format!("{}", &some), "1 unit");
    }
    #[test]
    fn add() {
        let one = SomeUnit::new(1);
        let two = SomeUnit::new(2);
        let three = one + two;
        assert_eq!(3, three.into_num());
    }
    #[test]
    fn sub() {
        let three = SomeUnit::new(3);
        let two = SomeUnit::new(2);
        let one = three - two;
        assert_eq!(1, one.into_num());
    }
    #[test]
    fn mul_num() {
        let two = SomeUnit::new(2);
        let three = 3;
        let six = two * three;
        assert_eq!(6, six.into_num());
    }
    #[test]
    fn div_num() {
        let six = SomeUnit::new(6);
        let three = 3;
        let two = six / three;
        assert_eq!(2, two.into_num());
    }
    #[test]
    fn from() {
        let one = SomeUnit::from(1);
        assert_eq!(1, one.into_num());
    }
    #[test]
    fn into() {
        let one: SomeUnit<_> = 1.into();
        assert_eq!(1, one.into_num());
    }
}
