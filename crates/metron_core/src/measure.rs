use crate::convert::{DivMeasure, IntoMeasure, MulMeasure};
use crate::unit::Unit;
use core::fmt;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

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
    pub fn of(num: N) -> Self {
        Self::new(num)
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
impl<N, M> Copy for Measure<N, M> where N: Copy {}

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

impl<N, M> Eq for Measure<N, M>
where
    Self: PartialEq<Measure<N, M>>,
    N: Eq,
{
}

impl<N, M> PartialEq for Measure<N, M>
where
    N: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl<N, M> Neg for Measure<N, M>
where
    N: Neg,
{
    type Output = Measure<<N as Neg>::Output, M>;

    fn neg(self) -> Self::Output {
        Self::Output::new(-self.num)
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

impl<NSelf, M, NRhs> MulAssign<NRhs> for Measure<NSelf, M>
where
    NSelf: MulAssign<NRhs>,
{
    fn mul_assign(&mut self, rhs: NRhs) {
        self.num *= rhs;
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

impl<NSelf, M, NRhs> DivAssign<NRhs> for Measure<NSelf, M>
where
    NSelf: DivAssign<NRhs>,
{
    fn div_assign(&mut self, rhs: NRhs) {
        self.num /= rhs;
    }
}

impl<NSelf, M, NRhs> Rem<NRhs> for Measure<NSelf, M>
where
    NSelf: Rem<NRhs>,
{
    type Output = Measure<<NSelf as Rem<NRhs>>::Output, M>;

    fn rem(self, rhs: NRhs) -> Self::Output {
        Self::Output::new(self.num % rhs)
    }
}

impl<NSelf, M, NRhs> RemAssign<NRhs> for Measure<NSelf, M>
where
    NSelf: RemAssign<NRhs>,
{
    fn rem_assign(&mut self, rhs: NRhs) {
        self.num %= rhs
    }
}

impl<N, M> PartialOrd for Measure<N, M>
where
    Self: PartialEq<Measure<N, M>>,
    N: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.num.partial_cmp(&other.num)
    }
}
impl<N, M> Ord for Measure<N, M>
where
    Self: Eq + PartialOrd<Measure<N, M>>,
    N: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.num.cmp(&other.num)
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::{DivMeasure, MulMeasure};
    use crate::unit::Unit;
    use crate::Measure;
    use std::cmp::Ordering;
    use std::fmt::Formatter;
    use std::ops::{Div, Mul};

    struct USome;
    impl Unit for USome {}
    type SomeUnit<N> = Measure<N, USome>;

    struct USomeMul;
    impl Unit for USomeMul {}
    type SomeUnitMul<N> = Measure<N, USomeMul>;
    impl<N: Mul<N>> MulMeasure<USome, N> for USome {
        type Output = USomeMul;
    }

    struct USomeDiv;
    impl Unit for USomeDiv {}
    type SomeUnitDiv<N> = Measure<N, USomeDiv>;
    impl<N: Div<N>> DivMeasure<USome, N> for USome {
        type Output = USomeDiv;
    }

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
    fn neg() {
        assert_eq!(SomeUnit::new(-1), -SomeUnit::new(1))
    }
    #[test]
    fn add() {
        let one = SomeUnit::new(1);
        let two = SomeUnit::new(2);
        let three = one + two;
        assert_eq!(3, three.into_num());
    }
    #[test]
    fn add_assign() {
        let mut lhs = SomeUnit::new(1);
        let rhs = SomeUnit::new(2);
        lhs += rhs;
        assert_eq!(3, lhs.into_num());
    }
    #[test]
    fn sub() {
        let three = SomeUnit::new(3);
        let two = SomeUnit::new(2);
        let one = three - two;
        assert_eq!(1, one.into_num());
    }
    #[test]
    fn sub_assign() {
        let mut lhs = SomeUnit::of(1);
        let rhs = SomeUnit::new(2);
        lhs += rhs;
        assert_eq!(3, *lhs.num());
    }
    #[test]
    fn mul_num() {
        let two = SomeUnit::new(2);
        let three = 3;
        let six = two * three;
        assert_eq!(6, six.into_num());
    }
    #[test]
    fn mul_num_assign() {
        let mut lhs = SomeUnit::of(2);
        let rhs = 3;
        lhs *= rhs;
        assert_eq!(6, lhs.into_num());
    }
    #[test]
    fn mul_unit() {
        let lhs = SomeUnit::of(2);
        let rhs = SomeUnit::of(3);
        let mul = lhs * rhs;
        assert_eq!(SomeUnitMul::of(6), mul)
    }
    #[test]
    fn div_num() {
        let six = SomeUnit::new(6);
        let three = 3;
        let two = six / three;
        assert_eq!(2, two.into_num());
    }
    #[test]
    fn div_num_assign() {
        let mut lhs = SomeUnit::of(6);
        let rhs = 3;
        lhs /= rhs;
        assert_eq!(2, lhs.into_num())
    }
    #[test]
    fn div_unit() {
        let lhs = SomeUnit::of(6);
        let rhs = SomeUnit::of(3);
        let div = lhs / rhs;
        assert_eq!(SomeUnitDiv::of(2), div)
    }
    #[test]
    fn partial_ord_eq() {
        let lhs = SomeUnit::of(1.0);
        let rhs = SomeUnit::of(1.0);
        assert_eq!(Some(Ordering::Equal), lhs.partial_cmp(&rhs));
        assert_eq!(false, lhs < rhs);
        assert_eq!(true, lhs <= rhs);
        assert_eq!(true, lhs >= rhs);
        assert_eq!(false, lhs > rhs);
    }
    #[test]
    fn partial_ord_gt() {
        let lhs = SomeUnit::of(2.0);
        let rhs = SomeUnit::of(1.0);
        assert_eq!(Some(Ordering::Greater), lhs.partial_cmp(&rhs));
        assert_eq!(false, lhs < rhs);
        assert_eq!(false, lhs <= rhs);
        assert_eq!(true, lhs >= rhs);
        assert_eq!(true, lhs > rhs);
    }
    #[test]
    fn partial_ord_lt() {
        let lhs = SomeUnit::of(1.0);
        let rhs = SomeUnit::of(2.0);
        assert_eq!(Some(Ordering::Less), lhs.partial_cmp(&rhs));
        assert_eq!(true, lhs < rhs);
        assert_eq!(true, lhs <= rhs);
        assert_eq!(false, lhs >= rhs);
        assert_eq!(false, lhs > rhs);
    }
    #[test]
    fn ord_eq() {
        let lhs = SomeUnit::of(1);
        let rhs = SomeUnit::of(1);
        assert_eq!(Ordering::Equal, lhs.cmp(&rhs));
        assert_eq!(false, lhs < rhs);
        assert_eq!(true, lhs <= rhs);
        assert_eq!(true, lhs >= rhs);
        assert_eq!(false, lhs > rhs);
    }
    #[test]
    fn ord_gt() {
        let lhs = SomeUnit::of(2);
        let rhs = SomeUnit::of(1);
        assert_eq!(Ordering::Greater, lhs.cmp(&rhs));
        assert_eq!(false, lhs < rhs);
        assert_eq!(false, lhs <= rhs);
        assert_eq!(true, lhs >= rhs);
        assert_eq!(true, lhs > rhs);
    }
    #[test]
    fn ord_lt() {
        let lhs = SomeUnit::of(1);
        let rhs = SomeUnit::of(2);
        assert_eq!(Ordering::Less, lhs.cmp(&rhs));
        assert_eq!(true, lhs < rhs);
        assert_eq!(true, lhs <= rhs);
        assert_eq!(false, lhs >= rhs);
        assert_eq!(false, lhs > rhs);
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
