#![feature(test)]
extern crate test;

#[path = "../src/flipper.rs"]
mod flipper;
#[path = "../src/fusion.rs"]
mod fusion;
#[path = "../src/graph.rs"]
mod graph;
#[path = "../src/reducibility.rs"]
mod reducibility;
#[path = "../src/set_cover.rs"]
mod set_cover;

use graph::read_configurations;
use test::Bencher;

#[bench]
fn bench_find_fusion_candidate_pairs(b: &mut Bencher) {
    let configs = read_configurations("unavoidable_629.conf").unwrap();
    b.iter(|| {
        fusion::find_fusion_candidate_pairs(&configs);
    });
}
