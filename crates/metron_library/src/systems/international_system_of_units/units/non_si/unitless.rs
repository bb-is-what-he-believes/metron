#[cfg(test)]
mod test {
    use metron_core::Unit;
    use metron_core::{def_unit, impl_unit_from};
    use std::fmt::Formatter;
    def_unit! {
        Metre{
            // syn( "m" ),
        }
    }
    impl metron_core::fmt::Symbol<()> for Metre{
        fn fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "m")
        }
    }
    struct Feet;
    impl ::metron_core::unit::Unit for Feet {}
    impl<N> ::metron_core::convert::FromUnit<N, Feet> for Feet {
        type Output = N;
        fn from_unit(num: N) -> <Self as ::metron_core::convert::FromUnit<N, Feet>>::Output {
            num
        }
    }
    impl<N> ::metron_core::convert::FromUnit<N, Metre> for Feet
    where
        N: std::ops::Mul<f32, Output = N>,
    {
        type Output = N;
        fn from_unit(num: N) -> N {
            (num * 1.0)
        }
    }
    def_unit! {
        Metre |* Metre => SquareMeter{
            // div( Self |/ Metre => Metre),
        }
    }
    impl metron_core::fmt::Symbol<()> for SquareMeter{
        fn fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "m²")
        }
    }
    metron_core::impl_unit_div!(SquareMeter |/ Metre => Metre);
    def_unit! {
        Metre |/ Metre => Radian;
    }
    impl metron_core::fmt::Symbol<()> for Radian{
        fn fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "rad")
        }
    }
    def_unit!{
        SquareMeter |/ SquareMeter => Steradian;
    }
    impl metron_core::fmt::Symbol<()> for Steradian{
        fn fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "strad")
        }
    }


    pub struct Unitless<D> {
        _derive: std::marker::PhantomData<D>,
    }
    impl<D> Unit for Unitless<D> {}
    impl<N, D> ::metron_core::convert::FromUnit<N, Unitless<D>> for Unitless<D> {
        type Output = N;
        fn from_unit(num: N) -> <Self as ::metron_core::convert::FromUnit<N, Unitless<D>>>::Output {
            num
        }
    }

    impl<N> ::metron_core::convert::FromUnit<N, Radian> for Unitless<Metre> {
        type Output = N;
        fn from_unit(num: N) -> N {
            num
        }
    }

    pub struct Percent<M> {
        _derive: std::marker::PhantomData<M>,
    }
    pub struct PercentPoint<M> {
        _derive: std::marker::PhantomData<M>,
    }
    mod measure {
        use metron_core::def_measure;
        use std::ops::Mul;
        use std::fmt::{Debug, Formatter};
        def_measure!(Metre  = super::Metre,);
        def_measure!(Radian = super::Radian,);
        def_measure!(Feet   = super::Feet,);
        type Percent<N, D> = metron_core::Measure<N, super::Percent<D>>;

        #[test]
        fn test() {
            {
                // struct U<const D: i8> {
                //     num : i64,
                // }
                //
                // impl<const D: i8> Debug for U<D>{
                //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                //         f.debug_struct("U")
                //             .field("const D:i8 ", &D)
                //             .finish()
                //     }
                // }
                //
                // impl<const DLhs: i8, const DRhs: i8> std::ops::Mul<U<DRhs>> for U<DLhs>
                // where
                //     U<{ DLhs + DRhs }>: Sized,
                // {
                //     type Output = U<{ DLhs + DRhs }>;
                //     fn mul(self, rhs: U<DRhs>) -> Self::Output {
                //         U {
                //             num: self.num * rhs.num,
                //         }
                //     }
                // }
                // let lhs = U::<1>{num: 2};
                // let rhs = U::<2>{num: 3};
                // let ans = lhs * rhs;
                // println!("{:?}", ans)


                let hen_a = Metre::new(10.0f32);
                let hen_b = Metre::new(3.0f32);
                let hen_c = Metre::new(5.0f32);
                // let t1 = (hen_a * hen_a + hen_b * hen_b + hen_c * hen_c);
                // println!("{}",&t1);
                // let t2 = t1 / 2.0f32;
                // println!("{}",&t2);
                // let t3 = t2 / hen_b;
                // println!("{}",&t3);
                // let cos_a = t3 / hen_c;
                let cos_a = (hen_a * hen_a + hen_b * hen_b + hen_c * hen_c) / 2.0f32 / hen_b / hen_c;
                println!("{}",&cos_a);
            }
        }
    }
}
