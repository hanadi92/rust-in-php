#![feature(test)]

use test::Bencher;
use extension::find_departing_trains_php;

extern crate test;

#[bench]
fn bench_workload(b: &mut Bencher) {
    b.iter(|| find_departing_trains_php("14:54:20", "Tokoyo"));
}
