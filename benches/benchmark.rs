#![allow(unused)]
use base16384::{Base16384, Base16384Utf8};
use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};

fn small_encode(b: &mut Bencher) {
    let data = [0u8; 32];
    let mut buf = vec![0u16; Base16384::encode_len(data.len())];
    b.iter(|| {
        black_box(Base16384::encode_to_slice(
            black_box(&data),
            black_box(&mut buf),
        ));
    });
}

fn small_decode(b: &mut Bencher) {
    let mut data = [0x4e00u16; 20];
    data[19] = 0x3d04;
    let mut buf = vec![0u8; Base16384::decode_len(data.len(), Some(0x3d04))];
    b.iter(|| {
        black_box(Base16384::decode_to_slice(
            black_box(&data),
            black_box(&mut buf),
        ));
    });
}

fn large_encode(b: &mut Bencher) {
    let data = vec![0u8; 1024000];
    let mut buf = vec![0u16; Base16384::encode_len(data.len())];
    b.iter(|| {
        black_box(Base16384::encode_to_slice(
            black_box(&data),
            black_box(&mut buf),
        ));
    });
}

fn large_decode(b: &mut Bencher) {
    let mut data = vec![0x4e00u16; 585144];
    data[585143] = 0x3d05;
    let mut buf = vec![0u8; Base16384::decode_len(data.len(), Some(0x3d05))];
    b.iter(|| {
        black_box(Base16384::decode_to_slice(
            black_box(&data),
            black_box(&mut buf),
        ));
    });
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("small encode", small_encode);
    c.bench_function("small decode", small_decode);
    c.bench_function("large encode", large_encode);
    c.bench_function("large decode", large_decode);
}

fn small_encode_utf8(b: &mut Bencher) {
    let data = [0u8; 32];
    let mut buf = "A".repeat(Base16384Utf8::encode_len(data.len()));
    b.iter(|| {
        black_box(Base16384Utf8::encode_to_slice(
            black_box(&data),
            black_box(&mut buf),
        ));
    });
}

fn small_decode_utf8(b: &mut Bencher) {
    let mut data = std::iter::repeat([0xE4, 0xB8, 0x80])
        .take(19)
        .flatten()
        .collect::<Vec<_>>();
    data.extend([0xE3, 0xB4, 0x84]);
    let data = String::from_utf8(data).unwrap();
    let mut buf = vec![0u8; Base16384Utf8::decode_len(data.len(), Some(0x3d04))];
    b.iter(|| {
        black_box(Base16384Utf8::decode_to_slice(
            black_box(&data),
            black_box(&mut buf),
        ));
    });
}

fn large_encode_utf8(b: &mut Bencher) {
    let data = vec![0u8; 1024000];
    let mut buf = "A".repeat(Base16384Utf8::encode_len(data.len()));
    b.iter(|| {
        black_box(Base16384Utf8::encode_to_slice(
            black_box(&data),
            black_box(&mut buf),
        ));
    });
}

fn large_decode_utf8(b: &mut Bencher) {
    let mut data = std::iter::repeat([0xE4, 0xB8, 0x80])
        .take(585143)
        .flatten()
        .collect::<Vec<_>>();
    data.extend([0xE3, 0xB4, 0x85]);
    let data = String::from_utf8(data).unwrap();
    let mut buf = vec![0u8; Base16384Utf8::decode_len(data.len(), Some(0x3d05))];
    b.iter(|| {
        black_box(Base16384Utf8::decode_to_slice(
            black_box(&data),
            black_box(&mut buf),
        ));
    });
}

pub fn criterion_benchmark_utf8(c: &mut Criterion) {
    c.bench_function("small encode", small_encode);
    c.bench_function("small decode", small_decode);
    c.bench_function("large encode", large_encode);
    c.bench_function("large decode", large_decode);
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_utf8);
criterion_main!(benches);
