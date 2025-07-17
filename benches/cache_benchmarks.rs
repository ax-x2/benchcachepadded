use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cachepadded::{CounterArray, NonPaddedCounter, PaddedCounter, benchmark_counters};
use std::sync::Arc;

fn bench_false_sharing(c: &mut Criterion) {
    let mut group = c.benchmark_group("false_sharing");
    
    // test with 2 threads sharing adjacent counters
    let thread_count = 2;
    let iterations = 1_000_000;
    
    group.bench_function("non_padded", |b| {
        b.iter(|| {
            let counters = Arc::new(CounterArray::<NonPaddedCounter>::new(thread_count));
            let duration = benchmark_counters(counters, thread_count, iterations);
            black_box(duration);
        })
    });
    
    group.bench_function("padded", |b| {
        b.iter(|| {
            let counters = Arc::new(CounterArray::<PaddedCounter>::new(thread_count));
            let duration = benchmark_counters(counters, thread_count, iterations);
            black_box(duration);
        })
    });
    
    group.finish();
}

fn bench_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling");
    let iterations = 500_000;
    
    for thread_count in [1, 2, 4, 8].iter() {
        group.bench_with_input(
            format!("non_padded_{}_threads", thread_count),
            thread_count,
            |b, &thread_count| {
                b.iter(|| {
                    let counters = Arc::new(CounterArray::<NonPaddedCounter>::new(thread_count));
                    let duration = benchmark_counters(counters, thread_count, iterations);
                    black_box(duration);
                })
            },
        );
        
        group.bench_with_input(
            format!("padded_{}_threads", thread_count),
            thread_count,
            |b, &thread_count| {
                b.iter(|| {
                    let counters = Arc::new(CounterArray::<PaddedCounter>::new(thread_count));
                    let duration = benchmark_counters(counters, thread_count, iterations);
                    black_box(duration);
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_false_sharing, bench_scaling);
criterion_main!(benches);
