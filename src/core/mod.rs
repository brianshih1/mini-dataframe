use once_cell::sync::Lazy;
use rayon::{ThreadPool, ThreadPoolBuilder};

pub mod field;
pub mod iterator;
pub mod schema;
pub mod sync_ptr;

pub static POOL: Lazy<ThreadPool> = Lazy::new(|| {
    ThreadPoolBuilder::new()
        .num_threads(std::thread::available_parallelism().unwrap().get())
        .build()
        .unwrap()
});
