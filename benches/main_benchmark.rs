use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, AxisScale, PlotConfiguration};
#[path = "../src/multiplication_table.rs"] mod multiplication_table;
use multiplication_table::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_multiplication_table");
    group.warm_up_time(Duration::from_secs(2));
    group.measurement_time(Duration::from_secs(3));
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    group.sample_size(50);

    // generate_multiplication_table
    for n in [9, 1<<4, 1<<5, 1<<6, 1<<7, 3<<6, 1<<8, 1<<9, 3<<8, 1<<10, 3<<9, 1<<11, 3<<10, 1<<12, 1<<13, 3<<12].iter() {
        group.bench_with_input(BenchmarkId::new("2D_standard", n), n, |b, n| {
            b.iter(|| {
                generation::two_d::generate_multiplication_table(black_box(*n));
            })
        });
        group.bench_with_input(BenchmarkId::new("2D_parallell", n), n, |b, n| {
            b.iter(|| {
                generation::two_d::generate_multiplication_table_async(black_box(*n));
            })
        });

    }
    group.finish();
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);