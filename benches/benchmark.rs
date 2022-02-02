use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, AxisScale, PlotConfiguration};

#[path = "../src/multiplication_table.rs"] mod multiplication_table;
use multiplication_table::generation::{two_d, one_d};
pub fn multiplication_table_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_multiplication_table");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    // generate_multiplication_table
    for n in [9, 1<<4, 1<<6, 1<<8, two_d::SMART_CUTOFF-1, 1<<10, 1<<12, 1<<13].iter() {
        group.bench_with_input(BenchmarkId::new("2D_standard", n), n, |b, n| {
            b.iter(|| {
                two_d::generate_multiplication_table(black_box(*n));
            })
        });
        group.bench_with_input(BenchmarkId::new("2D_parallell", n), n, |b, n| {
            b.iter(|| {
                two_d::generate_multiplication_table_par(black_box(*n));
            })
        });
        group.bench_with_input(BenchmarkId::new("2D_smart", n), n, |b, n| {
            b.iter(|| {
                two_d::generate_multiplication_table_smart(black_box(*n));
            })
        });
        group.bench_with_input(BenchmarkId::new("2D_cached", n), n, |b, n| {
            b.iter(|| {
                two_d::generate_multiplication_table_cached(black_box(*n));
            })
        });
        group.bench_with_input(BenchmarkId::new("2D_split", n), n, |b, n| {
            b.iter(|| {
                two_d::generate_multiplication_table_split(black_box(*n));
            })
        });
        group.bench_with_input(BenchmarkId::new("1D_standard", n), n, |b, n| {
            b.iter(|| {
                one_d::generate_multiplication_table(black_box(*n));
            })
        });
        group.bench_with_input(BenchmarkId::new("1D_split", n), n, |b, n| {
            b.iter(|| {
                one_d::generate_multiplication_table(black_box(*n));
            })
        });

    }
    group.finish();
}

const TEST_FILE: &[u8] = include_bytes!("../assets/test.png");
pub fn hashes(c: &mut Criterion) {

    let mut group = c.benchmark_group("hashes");
    group.bench_function("xxh3", |b| {
        b.iter(|| {
            xxhash_rust::xxh3::xxh3_64(TEST_FILE);
        })
    });
    let hash = xxhash_rust::xxh3::xxh3_64(TEST_FILE);
    println!("xxh3_64: {hash:16x}");
    group.bench_function("crc32fast", |b| {
        b.iter(|| {
            crc32fast::hash(TEST_FILE);
        })
    });
    let hash = crc32fast::hash(TEST_FILE);
    println!("CRC32: {hash:8x}");
}

criterion_group!(benches, multiplication_table_generation, hashes);
criterion_main!(benches);