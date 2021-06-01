use metron_core::def_unit;
def_unit! {
    pub Kilo |: Gram => KiloGram{
        sym( "kg" ),
    }
}
use crate::systems::international_system_of_units::units::si_prefix::{
    One,
    Yotta, Zetta, Exa,  Peta,  Tera, Giga, Mega,  Kilo,  Hecto, Deka,
    Yocto, Zepto, Atto, Femto, Pico, Nano, Micro, Milli, Centi, Deci,
};
def_unit! {
    pub Gram{
        exp( One ),
        sym( "g" ),
    }
}
def_unit! { pub  Yotta |: Gram => YottaGram{ sym(  "Yg" ), } }
def_unit! { pub  Zetta |: Gram => ZettaGram{ sym(  "Zg" ), } }
def_unit! { pub    Exa |: Gram =>   ExaGram{ sym(  "Eg" ), } }
def_unit! { pub   Peta |: Gram =>  PetaGram{ sym(  "Pg" ), } }
def_unit! { pub   Tera |: Gram =>  TeraGram{ sym(  "Tg" ), } }
def_unit! { pub   Giga |: Gram =>  GigaGram{ sym(  "Gg" ), } }
def_unit! { pub   Mega |: Gram =>  MegaGram{ sym(  "Mg" ), } }
def_unit! { pub  Hecto |: Gram => HectoGram{ sym(  "hg" ), } }
def_unit! { pub   Deka |: Gram =>  DekaGram{ sym( "dag" ), } }
def_unit! { pub   Deci |: Gram =>  DeciGram{ sym(  "dg" ), } }
def_unit! { pub  Centi |: Gram => CentiGram{ sym(  "Cg" ), } }
def_unit! { pub  Milli |: Gram => MilliGram{ sym(  "mg" ), } }
def_unit! { pub  Micro |: Gram => MicroGram{ sym(  "μg" ), } }
def_unit! { pub   Nano |: Gram =>  NanoGram{ sym(  "ng" ), } }
def_unit! { pub   Pico |: Gram =>  PicoGram{ sym(  "pg" ), } }
def_unit! { pub  Femto |: Gram => FemtoGram{ sym(  "fg" ), } }
def_unit! { pub   Atto |: Gram =>  AttoGram{ sym(  "ag" ), } }
def_unit! { pub  Zepto |: Gram => ZeptoGram{ sym(  "zg" ), } }
def_unit! { pub  Yocto |: Gram => YoctoGram{ sym(  "yg" ), } }