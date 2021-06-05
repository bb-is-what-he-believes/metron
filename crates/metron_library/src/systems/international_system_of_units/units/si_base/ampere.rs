use metron_core::def_unit;
def_unit! {
    pub Ampere{
        exp( One ),
        sym( "A" ),
    }
}
use crate::systems::international_system_of_units::units::si_prefix::{
    One,
    Yotta, Zetta, Exa,  Peta,  Tera, Giga, Mega,  Kilo,  Hecto, Deka,
    Yocto, Zepto, Atto, Femto, Pico, Nano, Micro, Milli, Centi, Deci,
};
def_unit!{ pub Yotta |: Ampere => YottaAmpere{ sym(  "YA" ), } }
def_unit!{ pub Zetta |: Ampere => ZettaAmpere{ sym(  "ZA" ), } }
def_unit!{ pub   Exa |: Ampere =>   ExaAmpere{ sym(  "EA" ), } }
def_unit!{ pub  Peta |: Ampere =>  PetaAmpere{ sym(  "PA" ), } }
def_unit!{ pub  Tera |: Ampere =>  TeraAmpere{ sym(  "TA" ), } }
def_unit!{ pub  Giga |: Ampere =>  GigaAmpere{ sym(  "GA" ), } }
def_unit!{ pub  Mega |: Ampere =>  MegaAmpere{ sym(  "MA" ), } }
def_unit!{ pub  Kilo |: Ampere =>  KiloAmpere{ sym(  "kA" ), } }
def_unit!{ pub Hecto |: Ampere => HectoAmpere{ sym(  "hA" ), } }
def_unit!{ pub  Deka |: Ampere =>  DekaAmpere{ sym( "daA" ), } }
def_unit!{ pub  Deci |: Ampere =>  DeciAmpere{ sym(  "dA" ), } }
def_unit!{ pub Centi |: Ampere => CentiAmpere{ sym(  "CA" ), } }
def_unit!{ pub Milli |: Ampere => MilliAmpere{ sym(  "mA" ), } }
def_unit!{ pub Micro |: Ampere => MicroAmpere{ sym(  "μA" ), } }
def_unit!{ pub  Nano |: Ampere =>  NanoAmpere{ sym(  "nA" ), } }
def_unit!{ pub  Pico |: Ampere =>  PicoAmpere{ sym(  "pA" ), } }
def_unit!{ pub Femto |: Ampere => FemtoAmpere{ sym(  "fA" ), } }
def_unit!{ pub  Atto |: Ampere =>  AttoAmpere{ sym(  "aA" ), } }
def_unit!{ pub Zepto |: Ampere => ZeptoAmpere{ sym(  "zA" ), } }
def_unit!{ pub Yocto |: Ampere => YoctoAmpere{ sym(  "yA" ), } }
