#![warn(missing_debug_implementations, rust_2018_idioms)]

pub mod profile;

#[cfg(all(any(target_os = "linux", )))]
use mi_malloc_rust::MiMalloc;

#[cfg(all(any(target_os = "linux",)))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
