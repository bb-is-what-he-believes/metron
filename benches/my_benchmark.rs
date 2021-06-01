// #[macro_use]
// extern crate criterion;
//
// use criterion::black_box;
// use criterion::Criterion;
// use metron_library::units::length::si::Meter;
// fn mul_1(num: f32, num2: f32) {
//     num * num2;
// }
// fn mul_2(num: Meter<f32>, num2: f32) {
//     num * num2;
// }
// fn add_2(num: Meter<f32>, num2: Meter<f32>) {
//     num + num2;
// }
//
// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| {
//         b.iter(|| mul_1(black_box(20.0), black_box(20.0)))
//     });
//     c.bench_function("fib 20", |b| {
//         b.iter(|| mul_2(black_box(Meter::from(20.0)), black_box(20.0)))
//     });
//     c.bench_function("fib 20", |b| {
//         b.iter(|| add_2(black_box(Meter::from(20.0)), black_box(Meter::from(20.0))))
//     });
// }
//
// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);
