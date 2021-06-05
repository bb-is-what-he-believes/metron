use metron_core::def_unit;
use super::super::si_derived::par_second::ParSecond;
def_unit! {
    pub Becquerel{
        from(ParSecond =* 1.0),
        exp( One ),
        sym( "Bq" ),
    }
}
use crate::systems::international_system_of_units::units::si_prefix::{
    One,
    Yotta, Zetta, Exa,  Peta,  Tera, Giga, Mega,  Kilo,  Hecto, Deka,
    Yocto, Zepto, Atto, Femto, Pico, Nano, Micro, Milli, Centi, Deci,
};
def_unit! { pub  Yotta |: Becquerel => YottaBecquerel{ sym(  "YBq" ), } }
def_unit! { pub  Zetta |: Becquerel => ZettaBecquerel{ sym(  "ZBq" ), } }
def_unit! { pub    Exa |: Becquerel =>   ExaBecquerel{ sym(  "EBq" ), } }
def_unit! { pub   Peta |: Becquerel =>  PetaBecquerel{ sym(  "PBq" ), } }
def_unit! { pub   Tera |: Becquerel =>  TeraBecquerel{ sym(  "TBq" ), } }
def_unit! { pub   Giga |: Becquerel =>  GigaBecquerel{ sym(  "GBq" ), } }
def_unit! { pub   Mega |: Becquerel =>  MegaBecquerel{ sym(  "MBq" ), } }
def_unit! { pub   Kilo |: Becquerel =>  KiloBecquerel{ sym(  "kBq" ), } }
def_unit! { pub  Hecto |: Becquerel => HectoBecquerel{ sym(  "hBq" ), } }
def_unit! { pub   Deka |: Becquerel =>  DekaBecquerel{ sym( "daBq" ), } }
def_unit! { pub   Deci |: Becquerel =>  DeciBecquerel{ sym(  "dBq" ), } }
def_unit! { pub  Centi |: Becquerel => CentiBecquerel{ sym(  "CBq" ), } }
def_unit! { pub  Milli |: Becquerel => MilliBecquerel{ sym(  "mBq" ), } }
def_unit! { pub  Micro |: Becquerel => MicroBecquerel{ sym(  "μBq" ), } }
def_unit! { pub   Nano |: Becquerel =>  NanoBecquerel{ sym(  "nBq" ), } }
def_unit! { pub   Pico |: Becquerel =>  PicoBecquerel{ sym(  "pBq" ), } }
def_unit! { pub  Femto |: Becquerel => FemtoBecquerel{ sym(  "fBq" ), } }
def_unit! { pub   Atto |: Becquerel =>  AttoBecquerel{ sym(  "aBq" ), } }
def_unit! { pub  Zepto |: Becquerel => ZeptoBecquerel{ sym(  "zBq" ), } }
def_unit! { pub  Yocto |: Becquerel => YoctoBecquerel{ sym(  "yBq" ), } }