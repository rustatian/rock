#![warn(missing_debug_implementations, rust_2018_idioms)]

pub mod profile;

use mimalloc_rs::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
