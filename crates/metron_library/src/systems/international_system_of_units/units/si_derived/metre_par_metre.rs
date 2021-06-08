use metron_core::def_unit;
use super::super::si_base::metre::Metre;
def_unit! {
    Metre |/ Metre => pub MetreParMetre{
        exp( One ),
        sym( "m/m" ),
    }
}
use crate::systems::international_system_of_units::units::si_prefix::{
    One,
};