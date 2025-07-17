use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const CACHE_LINE_SIZE: usize = 64;

// non-padded counter - will cause false sharing
#[derive(Default)]
pub struct NonPaddedCounter {
    pub value: AtomicU64,
}

// cache-line padded counter - prevents false sharing
#[repr(align(64))]
pub struct PaddedCounter {
    pub value: AtomicU64,
    _padding: [u8; CACHE_LINE_SIZE - 8], // 64 - size_of::<AtomicU64>()
}

impl Default for PaddedCounter {
    fn default() -> Self {
        Self {
            value: AtomicU64::new(0),
            _padding: [0; CACHE_LINE_SIZE - 8],
        }
    }
}

pub struct CounterArray<T> {
    pub counters: Vec<T>,
}

impl<T: Default> CounterArray<T> {
    pub fn new(size: usize) -> Self {
        let mut counters = Vec::with_capacity(size);
        for _ in 0..size {
            counters.push(T::default());
        }
        Self { counters }
    }
}

pub fn benchmark_counters<T>(
    counters: Arc<CounterArray<T>>,
    thread_count: usize,
    iterations: usize,
) -> Duration
where
    T: Default + Send + Sync + 'static,
    T: AsRef<AtomicU64>,
{
    let start = std::time::Instant::now();
    
    let handles: Vec<_> = (0..thread_count)
        .map(|thread_id| {
            let counters = Arc::clone(&counters);
            thread::spawn(move || {
                let counter_idx = thread_id % counters.counters.len();
                let counter = counters.counters[counter_idx].as_ref();
                
                for _ in 0..iterations {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    start.elapsed()
}

impl AsRef<AtomicU64> for NonPaddedCounter {
    fn as_ref(&self) -> &AtomicU64 {
        &self.value
    }
}

impl AsRef<AtomicU64> for PaddedCounter {
    fn as_ref(&self) -> &AtomicU64 {
        &self.value
    }
}
