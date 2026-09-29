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
        "\nBenchmarking is_subconfiguration with {} configurations...",
        configs.len()
    );
    let iterations_subconf = 100;
    let t0_subconf = Instant::now();
    let mut match_count = 0;
    for _ in 0..iterations_subconf {
        for i in 0..configs.len() {
            for j in 0..configs.len() {
                if fusion::is_subconfiguration(&configs[i], &configs[j]) {
                    match_count += 1;
                }
            }
        }
    }
    let elapsed_subconf = t0_subconf.elapsed();
    let per_iter_subconf = elapsed_subconf / iterations_subconf as u32;
    println!("Matches found: {}", match_count / iterations_subconf as u32);
    println!(
        "Total time ({} iters): {:.2?}",
        iterations_subconf, elapsed_subconf
    );
    println!("Average time per iteration: {:.2?}", per_iter_subconf);
}
