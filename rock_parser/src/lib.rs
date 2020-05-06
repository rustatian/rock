pub mod profile;

use mimalloc_rs::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
