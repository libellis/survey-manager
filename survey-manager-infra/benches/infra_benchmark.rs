#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn benchmark_reading_no_cache(c: &mut Criterion) {
    c.bench_function("Mysql repo reading with no cache layer."{
        b.iter(|| "placeholder.  put computation here")
    });
}

fn benchmark_reading_with_redis_cache(c: &mut Criterion) {
    c.bench_function("Mysql repo reading with redis cache layer."{
        b.iter(|| "placeholder.  put computation here")
    });
}

fn benchmark_reading_with_memecached_layer(c: &mut Criterion) {
    c.bench_function("Mysql repo reading with memecached cache layer."{
        b.iter(|| "placeholder.  put computation here")
    });
}

fn benchmark_reading_with_evhashmap_layer(c: &mut Criterion) {
    c.bench_function("Mysql repo reading with evhashmap cache layer."{
        b.iter(|| "placeholder.  put computation here")
    });
}

criterion_group!(benches, benchmark_reading_no_cache, benchmark_reading_with_redis_cache,);

criterion_main!(benches);
