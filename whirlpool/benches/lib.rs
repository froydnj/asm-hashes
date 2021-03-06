#![no_std]
#![feature(test)]
extern crate test;
extern crate whirlpool_asm;

use test::Bencher;

#[bench]
fn bench_compress(b: &mut Bencher) {
    let mut state = Default::default();
    let data = Default::default();

    b.iter(|| {
        whirlpool_asm::compress(&mut state, &data);
    });

    b.bytes = data.len() as u64;
}
