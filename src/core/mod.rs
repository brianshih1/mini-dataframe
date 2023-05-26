use once_cell::sync::Lazy;
use rayon::{ThreadPool, ThreadPoolBuilder};

pub static POOL: Lazy<ThreadPool> = Lazy::new(|| {
    ThreadPoolBuilder::new()
        .num_threads(std::thread::available_parallelism().unwrap().get())
        .build()
        .unwrap()
});
