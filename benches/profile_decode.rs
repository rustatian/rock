use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rock::profile::buffer::{Buffer, ProfileDecoder};
use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;

pub fn profile_bench_cpu(c: &mut Criterion) {
    let r_file_res = std::fs::File::open("tests/CPU.pb.gz");
    match r_file_res {
        Ok(mut file) => {
            let mut buffer = vec![];
            let _ = file.read_to_end(&mut buffer);
            c.bench_function("profile_bench_cpu", |b| {
                b.iter(|| Buffer::decode(black_box(buffer.clone())))
            });
        }
        Err(err) => panic!(err),
    }
}

pub fn profile_bench_heap(c: &mut Criterion) {
    let r_file_res = std::fs::File::open("tests/HEAP.pb.gz");
    match r_file_res {
        Ok(mut file) => {
            let mut buffer = vec![];
            let _ = file.read_to_end(&mut buffer);
            c.bench_function("profile_bench_heap", |b| {
                b.iter(|| Buffer::decode(black_box(buffer.clone())))
            });
        }
        Err(err) => panic!(err),
    }
}

pub fn profile_bench_encoded(c: &mut Criterion) {
    let r_file_res = std::fs::File::open("tests/encoded");
    match r_file_res {
        Ok(mut file) => {
            let mut buffer = vec![];
            let _ = file.read_to_end(&mut buffer);
            c.bench_function("profile_bench_heap", |b| {
                b.iter(|| Buffer::decode(black_box(buffer.clone())))
            });
        }
        Err(err) => panic!(err),
    }
}

criterion_group!(
    benches,
    profile_bench_cpu,
    profile_bench_heap,
    profile_bench_encoded
);
criterion_main!(benches);
