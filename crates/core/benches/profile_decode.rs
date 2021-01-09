use core::profile;
use core::profile::buffer::Decoder;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::Read;
use std::time::Duration;

pub fn profile_bench_cpu(c: &mut Criterion) {
    let r_file_res = std::fs::File::open("tests/CPU.pb.gz");
    match r_file_res {
        Ok(mut file) => {
            let mut buffer = vec![];
            let _ = file.read_to_end(&mut buffer);
            c.bench_function("profile_bench_cpu", |b| {
                b.iter(|| profile::buffer::Buffer::decode(black_box(buffer.as_mut())))
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
                b.iter(|| profile::buffer::Buffer::decode(black_box(buffer.as_mut())))
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
            c.bench_function("profile_bench_encoded", |b| {
                b.iter(|| profile::buffer::Buffer::decode(black_box(buffer.as_mut())))
            });
        }
        Err(err) => panic!(err),
    }
}

pub fn profile_bench_big_1min_13025_lines(c: &mut Criterion) {
    let r_file_res = std::fs::File::open("tests/RR_CPU.pb.gz");
    match r_file_res {
        Ok(mut file) => {
            let mut buffer = vec![];
            let _ = file.read_to_end(&mut buffer);
            c.bench_function("profile_bench_big_1min_13025_lines", |b| {
                b.iter(|| profile::buffer::Buffer::decode(black_box(buffer.as_mut())))
            });
        }
        Err(err) => panic!(err),
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100).nresamples(5000).measurement_time(Duration::from_secs(60)).warm_up_time(Duration::from_secs(1));
    targets = profile_bench_cpu, profile_bench_heap, profile_bench_encoded
}

criterion_group! {
    name = slow_bench;
    config = Criterion::default().sample_size(10).nresamples(5000).measurement_time(Duration::from_secs(60)).warm_up_time(Duration::from_secs(1));
    targets = profile_bench_big_1min_13025_lines
}

criterion_main!(benches, slow_bench);
