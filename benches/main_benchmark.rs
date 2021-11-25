use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
#[path = "../src/main.rs"] mod main;
use main::*;
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_multiplication_table");
    group.warm_up_time(Duration::from_secs(2));
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(50);

    for n in [9, 1<<4, 1<<6, 1<<8, 1<<9, 1<<10, 1<<11, 1<<12, 1<<13, 3<<12, 1<<14].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, a| {
            b.iter(|| {
                generate_multiplication_table_smart(black_box(*a));
            })
        });
    }
    group.finish();
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);