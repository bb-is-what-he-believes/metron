use metron_core::def_unit;
def_unit! {
    pub Candela{
        exp( One ),
        sym( "cd" ),
    }
}
use crate::systems::international_system_of_units::units::si_prefix::{
    One,
    Yotta, Zetta, Exa,  Peta,  Tera, Giga, Mega,  Kilo,  Hecto, Deka,
    Yocto, Zepto, Atto, Femto, Pico, Nano, Micro, Milli, Centi, Deci,
};
def_unit! { pub  Yotta |: Candela => YottaCandela{ sym(  "Ycd" ), } }
def_unit! { pub  Zetta |: Candela => ZettaCandela{ sym(  "Zcd" ), } }
def_unit! { pub    Exa |: Candela =>   ExaCandela{ sym(  "Ecd" ), } }
def_unit! { pub   Peta |: Candela =>  PetaCandela{ sym(  "Pcd" ), } }
def_unit! { pub   Tera |: Candela =>  TeraCandela{ sym(  "Tcd" ), } }
def_unit! { pub   Giga |: Candela =>  GigaCandela{ sym(  "Gcd" ), } }
def_unit! { pub   Mega |: Candela =>  MegaCandela{ sym(  "Mcd" ), } }
def_unit! { pub   Kilo |: Candela =>  KiloCandela{ sym(  "kcd" ), } }
def_unit! { pub  Hecto |: Candela => HectoCandela{ sym(  "hcd" ), } }
def_unit! { pub   Deka |: Candela =>  DekaCandela{ sym( "dacd" ), } }
def_unit! { pub   Deci |: Candela =>  DeciCandela{ sym(  "dcd" ), } }
def_unit! { pub  Centi |: Candela => CentiCandela{ sym(  "Ccd" ), } }
def_unit! { pub  Milli |: Candela => MilliCandela{ sym(  "mcd" ), } }
def_unit! { pub  Micro |: Candela => MicroCandela{ sym(  "μcd" ), } }
def_unit! { pub   Nano |: Candela =>  NanoCandela{ sym(  "ncd" ), } }
def_unit! { pub   Pico |: Candela =>  PicoCandela{ sym(  "pcd" ), } }
def_unit! { pub  Femto |: Candela => FemtoCandela{ sym(  "fcd" ), } }
def_unit! { pub   Atto |: Candela =>  AttoCandela{ sym(  "acd" ), } }
def_unit! { pub  Zepto |: Candela => ZeptoCandela{ sym(  "zcd" ), } }
def_unit! { pub  Yocto |: Candela => YoctoCandela{ sym(  "ycd" ), } }