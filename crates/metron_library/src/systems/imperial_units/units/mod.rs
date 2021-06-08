pub mod length {
    use metron_core::def_unit;
    def_unit! {
        pub Thou {
            from(Inch =* 1000.),
            from(Feet =* 12000.),
            sym("th"),
        }
    }
    def_unit! {
        pub Inch {
            from(Thou =/ 1000.),
            sym("in"),
        }
    }
    def_unit! {
        pub Feet {
            from(Thou =/ 12000.),
            from(Inch =/ 12.),
            sym("ft"),
        }
    }
    def_unit! {
        pub Yard {
            from(Feet =/ 3.),
            sym("ft"),
        }
    }
    def_unit! {
        pub Chain {
            from(Yard =/ 22.),
            sym("ch"),
        }
    }
    def_unit! {
        pub Furlong {
            from(Chain =/ 10.),
            sym("fur"),
        }
    }
    def_unit! {
        pub Mile {
            from(Furlong =/ 8.),
            sym("mi"),
        }
    }
    def_unit! {
        pub League {
            from(Mile =/ 3.),
            sym("lea"),
        }
    }
    #[cfg(test)]
    mod test {
        use metron_core::Measure;
        #[test]
        fn thou2feet() {
            type Thou = Measure<f32, super::Thou>;
            type Feet = Measure<f32, super::Feet>;
            assert_eq!(Feet::new(1.), Feet::from_unit(Thou::new(12000.)))
        }
    }
}
