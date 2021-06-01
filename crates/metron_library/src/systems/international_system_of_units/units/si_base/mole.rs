use metron_core::def_unit;
def_unit! {
    pub Mole{
        exp( One ),
        sym( "mol" ),
    }
}
use crate::systems::international_system_of_units::units::si_prefix::{
    One,
    Yotta, Zetta, Exa,  Peta,  Tera, Giga, Mega,  Kilo,  Hecto, Deka,
    Yocto, Zepto, Atto, Femto, Pico, Nano, Micro, Milli, Centi, Deci,
};
def_unit! { pub  Yotta |: Mole => YottaMole{ sym(  "Ymol" ), } }
def_unit! { pub  Zetta |: Mole => ZettaMole{ sym(  "Zmol" ), } }
def_unit! { pub    Exa |: Mole =>   ExaMole{ sym(  "Emol" ), } }
def_unit! { pub   Peta |: Mole =>  PetaMole{ sym(  "Pmol" ), } }
def_unit! { pub   Tera |: Mole =>  TeraMole{ sym(  "Tmol" ), } }
def_unit! { pub   Giga |: Mole =>  GigaMole{ sym(  "Gmol" ), } }
def_unit! { pub   Mega |: Mole =>  MegaMole{ sym(  "Mmol" ), } }
def_unit! { pub   Kilo |: Mole =>  KiloMole{ sym(  "kmol" ), } }
def_unit! { pub  Hecto |: Mole => HectoMole{ sym(  "hmol" ), } }
def_unit! { pub   Deka |: Mole =>  DekaMole{ sym( "damol" ), } }
def_unit! { pub   Deci |: Mole =>  DeciMole{ sym(  "dmol" ), } }
def_unit! { pub  Centi |: Mole => CentiMole{ sym(  "Cmol" ), } }
def_unit! { pub  Milli |: Mole => MilliMole{ sym(  "mmol" ), } }
def_unit! { pub  Micro |: Mole => MicroMole{ sym(  "μmol" ), } }
def_unit! { pub   Nano |: Mole =>  NanoMole{ sym(  "nmol" ), } }
def_unit! { pub   Pico |: Mole =>  PicoMole{ sym(  "pmol" ), } }
def_unit! { pub  Femto |: Mole => FemtoMole{ sym(  "fmol" ), } }
def_unit! { pub   Atto |: Mole =>  AttoMole{ sym(  "amol" ), } }
def_unit! { pub  Zepto |: Mole => ZeptoMole{ sym(  "zmol" ), } }
def_unit! { pub  Yocto |: Mole => YoctoMole{ sym(  "ymol" ), } }