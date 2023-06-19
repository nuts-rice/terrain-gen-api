use criterion::{black_box, criterion_group,
criterion_main, Criterion, async_executor::{AsyncExecutor, AsyncStdExecutor, FuturesExecutor}};
use futures::executor::block_on;
use terrain_gen_api::routes::height_map;
pub async fn midpnt_displacement_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Midpnt Displacemnt");
    group.bench_function("midpnt displacement bench", |b| {
        let mut heightmap_bench = block_on(height_map::Heightmap::new(9, 0.2));
        // b.to_async(FuturesExecutor).iter(|| heightmap_bench.midpnt_displacement());
    });

    group.finish();

    
}

criterion_group!(benches, midpnt_displacement_bench);
criterion_main!(benches);
