use metron_core::convert::IntoMeasure;

// fn main() {
//     use metron_library::systems::international_system_of_units::measure as si;
//     use si::length::metre::{
//         KiloMetre,
//         Metre,
//     };
//     let meter1000 = Metre::from(1000.0);
//     let meter2000 = meter1000 * 2.0f32;
//     let kilo_meter2 = KiloMetre::from_scale(meter2000);
//     println!("{:#}",kilo_meter2);
//     println!("{:?}",kilo_meter2);
//     println!("{:#?}",kilo_meter2);
//     println!("Hello {0} is {1:.5}", "x", kilo_meter2);
// }
fn main() {
    use metron_library::systems::international_system_of_units::measure as si;
    use si::length::metre::{
        KiloMetre,
        Metre,
    };
    let meter1000 = Metre::from(1000.0);
    let meter2000 = meter1000 / 2.0f32.into_measure();
    println!("{:#}",meter2000);
}
