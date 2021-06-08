#[macro_export]
macro_rules! def_unit_measure{
    ( $name_mod_measure:ident :: $name_measure_and_unit:ident ) => {
        pub mod $name_mod_measure{
            pub mod unit{
                $crate::def_unit!(pub $name_measure_and_unit;);
            }
            $crate::def_measure!(pub $name_measure_and_unit = self::unit::$name_measure_and_unit);
        }
    };
}
