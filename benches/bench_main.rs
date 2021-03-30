use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::trivial_scenes::benches,
    benchmarks::complex_scenes::benches,
    benchmarks::bounding_boxes::benches,
}
