use super::super::units::si_base::metre::{
    KiloMetre,
    Metre,
};

metron_core::def_quantity!{
    /// Length is a measure of distance.
    pub Length
    /// basic unit for Length is Meter
    < Metre > {
        from( KiloMetre ),
    }
}