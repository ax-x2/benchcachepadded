use cachepadded::{CounterArray, NonPaddedCounter, PaddedCounter, benchmark_counters};
use clap::Parser;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "cachepadded")]
#[command(about = "benchmark cache-padded vs non-padded data structures")]
struct Args {
    #[arg(short, long, default_value = "1,2,4,8,16,32")]
    threads: String,
    
    #[arg(short, long, default_value = "1000000")]
    iterations: usize,
}

fn main() {
    let args = Args::parse();
    
    println!("cache-padded performance benchmark");
    println!("==================================");
    
    let thread_counts: Vec<usize> = args.threads
        .split(',')
        .map(|s| s.trim().parse().expect("invalid thread count"))
        .collect();
    
    println!("testing {} iterations per thread", args.iterations);
    println!();
    
    for &threads in &thread_counts {
        println!("threads: {}", threads);
        
        // benchmark non-padded
        let non_padded = Arc::new(CounterArray::<NonPaddedCounter>::new(threads));
        let non_padded_time = benchmark_counters(non_padded, threads, args.iterations);
        
        // benchmark padded
        let padded = Arc::new(CounterArray::<PaddedCounter>::new(threads));
        let padded_time = benchmark_counters(padded, threads, args.iterations);
        
        println!("  non-padded: {:?}", non_padded_time);
        println!("  padded:     {:?}", padded_time);
        
        if non_padded_time > padded_time {
            let speedup = non_padded_time.as_nanos() as f64 / padded_time.as_nanos() as f64;
            println!("  speedup:    {:.2}x", speedup);
        }
        println!();
    }
    
    println!("run 'cargo bench' for detailed criterion benchmarks");
}
