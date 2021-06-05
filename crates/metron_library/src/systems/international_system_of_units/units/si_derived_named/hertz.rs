use metron_core::def_unit;
use super::super::si_derived::par_second::ParSecond;
def_unit! {
    pub Hertz{
        from(ParSecond =* 1.0),
        exp( One ),
        sym( "Hz" ),
    }
}
use crate::systems::international_system_of_units::units::si_prefix::{
    One,
    Yotta, Zetta, Exa,  Peta,  Tera, Giga, Mega,  Kilo,  Hecto, Deka,
    Yocto, Zepto, Atto, Femto, Pico, Nano, Micro, Milli, Centi, Deci,
};
def_unit! { pub  Yotta |: Hertz => YottaHertz{ sym(  "YHz" ), } }
def_unit! { pub  Zetta |: Hertz => ZettaHertz{ sym(  "ZHz" ), } }
def_unit! { pub    Exa |: Hertz =>   ExaHertz{ sym(  "EHz" ), } }
def_unit! { pub   Peta |: Hertz =>  PetaHertz{ sym(  "PHz" ), } }
def_unit! { pub   Tera |: Hertz =>  TeraHertz{ sym(  "THz" ), } }
def_unit! { pub   Giga |: Hertz =>  GigaHertz{ sym(  "GHz" ), } }
def_unit! { pub   Mega |: Hertz =>  MegaHertz{ sym(  "MHz" ), } }
def_unit! { pub   Kilo |: Hertz =>  KiloHertz{ sym(  "kHz" ), } }
def_unit! { pub  Hecto |: Hertz => HectoHertz{ sym(  "hHz" ), } }
def_unit! { pub   Deka |: Hertz =>  DekaHertz{ sym( "daHz" ), } }
def_unit! { pub   Deci |: Hertz =>  DeciHertz{ sym(  "dHz" ), } }
def_unit! { pub  Centi |: Hertz => CentiHertz{ sym(  "CHz" ), } }
def_unit! { pub  Milli |: Hertz => MilliHertz{ sym(  "mHz" ), } }
def_unit! { pub  Micro |: Hertz => MicroHertz{ sym(  "μHz" ), } }
def_unit! { pub   Nano |: Hertz =>  NanoHertz{ sym(  "nHz" ), } }
def_unit! { pub   Pico |: Hertz =>  PicoHertz{ sym(  "pHz" ), } }
def_unit! { pub  Femto |: Hertz => FemtoHertz{ sym(  "fHz" ), } }
def_unit! { pub   Atto |: Hertz =>  AttoHertz{ sym(  "aHz" ), } }
def_unit! { pub  Zepto |: Hertz => ZeptoHertz{ sym(  "zHz" ), } }
def_unit! { pub  Yocto |: Hertz => YoctoHertz{ sym(  "yHz" ), } }