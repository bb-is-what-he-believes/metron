use metron_library::systems::international_system_of_units::measure as si;
use si::length::metre::{KiloMetre, Metre};
fn main() {
    let metre1000 = Metre::from(1000.0);
    let metre2000 = metre1000 * 2.0;
    let kilo_metre2 = KiloMetre::from_scale(metre2000);
    println!("{:.1} -> {:#}", metre2000, kilo_metre2);
}
