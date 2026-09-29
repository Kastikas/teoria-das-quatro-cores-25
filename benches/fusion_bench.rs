#![allow(dead_code, unused_imports)]

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
use std::path::Path;
use std::time::Instant;

fn main() {
    let conf_path = if Path::new("01_modelo_629_rsst_canonico/unavoidable_629.conf").exists() {
        "01_modelo_629_rsst_canonico/unavoidable_629.conf"
    } else {
        "unavoidable_629.conf"
    };

    let configs = match read_configurations(conf_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load {}: {}", conf_path, e);
            return;
        }
    };

    println!(
        "Benchmarking find_fusion_candidate_pairs with {} configurations...",
        configs.len()
    );
    let iterations = 100;
    let t0 = Instant::now();
    let mut total_pairs = 0;
    for _ in 0..iterations {
        let pairs = fusion::find_fusion_candidate_pairs(&configs);
        total_pairs = pairs.len();
    }
    let elapsed = t0.elapsed();
    let per_iter = elapsed / iterations as u32;
    println!("Candidate pairs found: {}", total_pairs);
    println!("Total time ({} iters): {:.2?}", iterations, elapsed);
    println!("Average time per iteration: {:.2?}", per_iter);
}
