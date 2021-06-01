use crate::systems::international_system_of_units::{
    quantity::length as quantity,
};
metron_core::def_measure! {
    pub Length   : quantity::Length,
}
pub mod metre{
    use crate::systems::international_system_of_units::{
        units::si_base::metre as unit,
    };
    metron_core::def_measure! {
        pub YottaMetre : unit::YottaMetre,
        pub ZettaMetre : unit::ZettaMetre,
        pub   ExaMetre : unit::  ExaMetre,
        pub  PetaMetre : unit:: PetaMetre,
        pub  TeraMetre : unit:: TeraMetre,
        pub  GigaMetre : unit:: GigaMetre,
        pub  MegaMetre : unit:: MegaMetre,
        pub  KiloMetre : unit:: KiloMetre,
        pub HectoMetre : unit::HectoMetre,
        pub  DekaMetre : unit:: DekaMetre,
        pub      Metre : unit::     Metre,
        pub  DeciMetre : unit:: DeciMetre,
        pub CentiMetre : unit::CentiMetre,
        pub MilliMetre : unit::MilliMetre,
        pub MicroMetre : unit::MicroMetre,
        pub  NanoMetre : unit:: NanoMetre,
        pub  PicoMetre : unit:: PicoMetre,
        pub FemtoMetre : unit::FemtoMetre,
        pub  AttoMetre : unit:: AttoMetre,
        pub ZeptoMetre : unit::ZeptoMetre,
        pub YoctoMetre : unit::YoctoMetre,
    }
}
