use metron_core::def_unit;
use super::super::si_base::second::Second;
def_unit! {
    () |/ Second => pub ParSecond{
        exp( One ),
        sym( "s⁻¹" ),
    }
}
use crate::systems::international_system_of_units::units::si_prefix::{
    One,
};