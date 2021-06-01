use metron_core::def_unit;
def_unit! {
    pub Kelvin{
        exp( One ),
        sym( "K" ),
    }
}
use crate::systems::international_system_of_units::units::si_prefix::{
    One,
    Yotta, Zetta, Exa,  Peta,  Tera, Giga, Mega,  Kilo,  Hecto, Deka,
    Yocto, Zepto, Atto, Femto, Pico, Nano, Micro, Milli, Centi, Deci,
};
def_unit! { pub  Yotta |: Kelvin => YottaKelvin{ sym(  "YK" ), } }
def_unit! { pub  Zetta |: Kelvin => ZettaKelvin{ sym(  "ZK" ), } }
def_unit! { pub    Exa |: Kelvin =>   ExaKelvin{ sym(  "EK" ), } }
def_unit! { pub   Peta |: Kelvin =>  PetaKelvin{ sym(  "PK" ), } }
def_unit! { pub   Tera |: Kelvin =>  TeraKelvin{ sym(  "TK" ), } }
def_unit! { pub   Giga |: Kelvin =>  GigaKelvin{ sym(  "GK" ), } }
def_unit! { pub   Mega |: Kelvin =>  MegaKelvin{ sym(  "MK" ), } }
def_unit! { pub   Kilo |: Kelvin =>  KiloKelvin{ sym(  "kK" ), } }
def_unit! { pub  Hecto |: Kelvin => HectoKelvin{ sym(  "hK" ), } }
def_unit! { pub   Deka |: Kelvin =>  DekaKelvin{ sym( "daK" ), } }
def_unit! { pub   Deci |: Kelvin =>  DeciKelvin{ sym(  "dK" ), } }
def_unit! { pub  Centi |: Kelvin => CentiKelvin{ sym(  "CK" ), } }
def_unit! { pub  Milli |: Kelvin => MilliKelvin{ sym(  "mK" ), } }
def_unit! { pub  Micro |: Kelvin => MicroKelvin{ sym(  "μK" ), } }
def_unit! { pub   Nano |: Kelvin =>  NanoKelvin{ sym(  "nK" ), } }
def_unit! { pub   Pico |: Kelvin =>  PicoKelvin{ sym(  "pK" ), } }
def_unit! { pub  Femto |: Kelvin => FemtoKelvin{ sym(  "fK" ), } }
def_unit! { pub   Atto |: Kelvin =>  AttoKelvin{ sym(  "aK" ), } }
def_unit! { pub  Zepto |: Kelvin => ZeptoKelvin{ sym(  "zK" ), } }
def_unit! { pub  Yocto |: Kelvin => YoctoKelvin{ sym(  "yK" ), } }