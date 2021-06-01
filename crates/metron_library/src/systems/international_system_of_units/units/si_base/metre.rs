use metron_core::def_unit;
def_unit! {
    pub Metre{
        exp( One ),
        sym( "m" ),
    }
}
use crate::systems::international_system_of_units::units::si_prefix::{
    One,
    Yotta, Zetta, Exa,  Peta,  Tera, Giga, Mega,  Kilo,  Hecto, Deka,
    Yocto, Zepto, Atto, Femto, Pico, Nano, Micro, Milli, Centi, Deci,
};
def_unit! { pub  Yotta |: Metre => YottaMetre{ sym(  "Ym" ), } }
def_unit! { pub  Zetta |: Metre => ZettaMetre{ sym(  "Zm" ), } }
def_unit! { pub    Exa |: Metre =>   ExaMetre{ sym(  "Em" ), } }
def_unit! { pub   Peta |: Metre =>  PetaMetre{ sym(  "Pm" ), } }
def_unit! { pub   Tera |: Metre =>  TeraMetre{ sym(  "Tm" ), } }
def_unit! { pub   Giga |: Metre =>  GigaMetre{ sym(  "Gm" ), } }
def_unit! { pub   Mega |: Metre =>  MegaMetre{ sym(  "Mm" ), } }
def_unit! { pub   Kilo |: Metre =>  KiloMetre{ sym(  "km" ), } }
def_unit! { pub  Hecto |: Metre => HectoMetre{ sym(  "hm" ), } }
def_unit! { pub   Deka |: Metre =>  DekaMetre{ sym( "dam" ), } }
def_unit! { pub   Deci |: Metre =>  DeciMetre{ sym(  "dm" ), } }
def_unit! { pub  Centi |: Metre => CentiMetre{ sym(  "Cm" ), } }
def_unit! { pub  Milli |: Metre => MilliMetre{ sym(  "mm" ), } }
def_unit! { pub  Micro |: Metre => MicroMetre{ sym(  "μm" ), } }
def_unit! { pub   Nano |: Metre =>  NanoMetre{ sym(  "nm" ), } }
def_unit! { pub   Pico |: Metre =>  PicoMetre{ sym(  "pm" ), } }
def_unit! { pub  Femto |: Metre => FemtoMetre{ sym(  "fm" ), } }
def_unit! { pub   Atto |: Metre =>  AttoMetre{ sym(  "am" ), } }
def_unit! { pub  Zepto |: Metre => ZeptoMetre{ sym(  "zm" ), } }
def_unit! { pub  Yocto |: Metre => YoctoMetre{ sym(  "ym" ), } }