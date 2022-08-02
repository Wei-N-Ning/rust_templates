// TODO: replace these tests with the real benchmarking tests

#![feature(test)]
extern crate test;
use test::Bencher;

// IMPORTANT!
// `cargo bench` requires the nightly build `rustup default nightly`

// reference:
// https://github.com/cgcardona/benchmarks/blob/master/src/main.rs
// https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/bench.html
// https://doc.rust-lang.org/beta/unstable-book/library-features/test.html

#[bench]
fn bench_xor_1000_ints(b: &mut Bencher) {
    b.iter(|| {
        (0..1000).fold(0, |old, new| old ^ new);
    });
}
