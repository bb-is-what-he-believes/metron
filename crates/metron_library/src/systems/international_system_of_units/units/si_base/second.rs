use metron_core::def_unit;
def_unit! {
    pub Second{
        exp( One ),
        sym( "s" ),
    }
}
use crate::systems::international_system_of_units::units::si_prefix::{
    One,
    Yotta, Zetta, Exa,  Peta,  Tera, Giga, Mega,  Kilo,  Hecto, Deka,
    Yocto, Zepto, Atto, Femto, Pico, Nano, Micro, Milli, Centi, Deci,
};
def_unit! { pub  Yotta |: Second => YottaSecond{ sym(  "Ys" ), } }
def_unit! { pub  Zetta |: Second => ZettaSecond{ sym(  "Zs" ), } }
def_unit! { pub    Exa |: Second =>   ExaSecond{ sym(  "Es" ), } }
def_unit! { pub   Peta |: Second =>  PetaSecond{ sym(  "Ps" ), } }
def_unit! { pub   Tera |: Second =>  TeraSecond{ sym(  "Ts" ), } }
def_unit! { pub   Giga |: Second =>  GigaSecond{ sym(  "Gs" ), } }
def_unit! { pub   Mega |: Second =>  MegaSecond{ sym(  "Ms" ), } }
def_unit! { pub   Kilo |: Second =>  KiloSecond{ sym(  "ks" ), } }
def_unit! { pub  Hecto |: Second => HectoSecond{ sym(  "hs" ), } }
def_unit! { pub   Deka |: Second =>  DekaSecond{ sym( "das" ), } }
def_unit! { pub   Deci |: Second =>  DeciSecond{ sym(  "ds" ), } }
def_unit! { pub  Centi |: Second => CentiSecond{ sym(  "Cs" ), } }
def_unit! { pub  Milli |: Second => MilliSecond{ sym(  "ms" ), } }
def_unit! { pub  Micro |: Second => MicroSecond{ sym(  "μs" ), } }
def_unit! { pub   Nano |: Second =>  NanoSecond{ sym(  "ns" ), } }
def_unit! { pub   Pico |: Second =>  PicoSecond{ sym(  "ps" ), } }
def_unit! { pub  Femto |: Second => FemtoSecond{ sym(  "fs" ), } }
def_unit! { pub   Atto |: Second =>  AttoSecond{ sym(  "as" ), } }
def_unit! { pub  Zepto |: Second => ZeptoSecond{ sym(  "zs" ), } }
def_unit! { pub  Yocto |: Second => YoctoSecond{ sym(  "ys" ), } }