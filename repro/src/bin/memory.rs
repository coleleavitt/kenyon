//! Memory footprint (bytes, from `memory_bytes()`) of the benchmarked
//! configurations, plus per-item code sizes. Usage: `memory`.

use anyhow::Result;
use kenyon::{
    BloomParams, DenseFly, DenseFlyIndex, FlyBloom, FlyHash, FlyHashParams, FlyIndex, FlyModel,
    FlyModelParams, Preprocess, Rng,
};

fn data(n: usize, d: usize, seed: u64) -> Vec<Vec<f32>> {
    let mut r = Rng::new(seed);
    (0..n)
        .map(|_| (0..d).map(|_| r.next_f64() as f32).collect())
        .collect()
}

fn main() -> Result<()> {
    println!("| structure | bytes | note |");
    println!("|---|---|---|");
    for (name, d, k) in [
        ("FlyHash MNIST d=784 m=7840 fan-in 78", 784, 4),
        ("FlyHash SIFT d=128 m=1280 fan-in 12", 128, 16),
    ] {
        let h = FlyHash::new(FlyHashParams::paper(d, k, 1))?;
        println!(
            "| {name} | {} | projection (compact: 1, 2 or 4-byte indices; no offsets at fixed fan-in) |",
            h.memory_bytes()
        );
    }
    let lsh = FlyHash::lsh(784, 4, Preprocess::CenterMean, 1)?;
    println!(
        "| LSH dense d=784 k=4 | {} | f32 Gaussian matrix |",
        lsh.memory_bytes()
    );
    let h = FlyHash::new(FlyHashParams::paper(128, 16, 1))?;
    let xs = data(10_000, 128, 4);
    let mut idx = FlyIndex::new(h.n_kc());
    for x in &xs {
        idx.insert(h.hash(x)?)?;
    }
    println!(
        "| FlyIndex 10k SIFT-like items, k=16 | {} | {:.1} bytes/item incl. postings |",
        idx.memory_bytes(),
        idx.memory_bytes() as f64 / 1e4
    );
    let df = DenseFly::new(128, 1280, 12, 20, Preprocess::CenterMean, 1)?;
    let mut di = DenseFlyIndex::new();
    for x in &xs {
        let (a, b) = df.hash(x)?;
        di.insert(a, b)?;
    }
    println!(
        "| DenseFlyIndex 10k items, 1280-bit h1, 64-bit h2 | {} | {} bins |",
        di.memory_bytes(),
        di.n_bins()
    );
    for (label, params) in [
        ("static (delta = epsilon = 0)", BloomParams::STATIC),
        (
            "with recovery (epsilon > 0)",
            BloomParams {
                decay: 0.5,
                recovery: 1e-4,
            },
        ),
    ] {
        let bh = FlyHash::new(FlyHashParams {
            n_inputs: 128,
            n_kc: 150_000,
            fan_in: 6,
            k: 20,
            preprocess: Preprocess::UnitNorm,
            seed: 1,
        })?;
        let proj = bh.memory_bytes();
        let bf = FlyBloom::new(bh, params)?;
        println!(
            "| FlyBloom m=150k, {label} | {} | of which projection {proj}; filter cells {} bytes (bit-packed when delta = epsilon = 0; classical BF: 18,750 bytes) |",
            bf.memory_bytes(),
            bf.memory_bytes() - proj
        );
    }
    let m = FlyModel::new(FlyModelParams::mnist20(1))?;
    println!(
        "| FlyModel MNIST-20 (84 -> 3200 -> 20) | {} | projection + 20x3200 f32 weights |",
        m.memory_bytes()
    );
    println!(
        "\nPer stored item: a FlyHash code costs 8 bytes per active unit (u32 index + f32 value); binary codes cost m/8 bytes as bitsets or 4k bytes as index lists."
    );
    Ok(())
}
