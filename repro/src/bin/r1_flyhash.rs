//! R1: FlyHash vs LSH on MNIST (Dasgupta, Stevens & Navlakha 2017, Fig. 2-3).
//! Protocol and bars: RESULTS.md part 1, R1. Usage: `r1_flyhash [trials]`.

use anyhow::Result;
use kenyon::{
    DenseProjection,
    Expansion,
    FlyHash,
    KcCode,
    Preprocess,
    Rng,
    Scratch,
    SparseProjection,
    TagRule,
    wta,
};
use kenyon_repro::metrics::{average_precision, sq_dist, top_r_by};
use kenyon_repro::stats::{mean, sd};
use kenyon_repro::{Matrix, data, data_dir, out_dir};
use rayon::prelude::*;

const N_QUERIES: usize = 1000;
const R: usize = 200;
const KS: [usize; 4] = [4, 8, 16, 32];

#[derive(Clone, Copy, PartialEq)]
enum Rank {
    Euclid,
    Overlap,
}

struct Method {
    name: &'static str,
    rank: Rank,
}

const METHODS: [Method; 10] = [
    Method {
        name: "fly",
        rank: Rank::Euclid,
    },
    Method {
        name: "lsh",
        rank: Rank::Euclid,
    },
    Method {
        name: "abl_dense_expansion_wta",
        rank: Rank::Euclid,
    },
    Method {
        name: "abl_no_wta_random_tag",
        rank: Rank::Euclid,
    },
    Method {
        name: "abl_no_expansion_sparse",
        rank: Rank::Euclid,
    },
    Method {
        name: "fly_binary_overlap",
        rank: Rank::Overlap,
    },
    // Exploratory (not predeclared): divisive "same mean" pre-processing.
    Method {
        name: "x_fly_divisive",
        rank: Rank::Euclid,
    },
    Method {
        name: "x_lsh_divisive",
        rank: Rank::Euclid,
    },
    Method {
        name: "x_fly_binary_divisive",
        rank: Rank::Overlap,
    },
    Method {
        name: "x_lsh_binary_simhash",
        rank: Rank::Overlap,
    },
];

/// Codes for every item for each k, for one method in one trial.
fn codes_for(method: &str, x: &Matrix, seed: u64) -> Result<Vec<Vec<KcCode>>> {
    let d = x.d;
    let m = 10 * d;
    let fan = d / 10;
    let pre = if method.contains("divisive") {
        Preprocess::DivisiveMean(100.0)
    } else {
        Preprocess::CenterMean
    };
    let method = match method {
        "x_fly_divisive" | "x_fly_binary_divisive" => "fly",
        "x_lsh_divisive" => "lsh",
        m => m,
    };
    // One hasher per k when the expansion itself depends on k (LSH, no-expansion);
    // otherwise one expansion and WTA / fixed tags for every k.
    match method {
        "x_lsh_binary_simhash" => KS
            .iter()
            .map(|&k| {
                // SimHash: k sign bits of dense Gaussian projections, as an index code.
                let h = FlyHash::lsh(d, k, pre, seed)?;
                (0..x.n)
                    .map(|i| {
                        let c = h.hash(x.row(i))?;
                        let on: Vec<u32> = c
                            .values()
                            .iter()
                            .enumerate()
                            .filter(|(_, v)| **v > 0.0)
                            .map(|(j, _)| j as u32)
                            .collect();
                        let n = on.len();
                        KcCode::new(k, on, vec![1.0; n])
                    })
                    .collect::<kenyon::Result<Vec<_>>>()
            })
            .collect::<kenyon::Result<Vec<_>>>()
            .map_err(Into::into),
        "lsh" | "abl_no_expansion_sparse" => KS
            .iter()
            .map(|&k| {
                let h = if method == "lsh" {
                    FlyHash::lsh(d, k, pre, seed)?
                } else {
                    let p = SparseProjection::fixed_fan_in(d, k, fan, seed)?;
                    FlyHash::with_expansion(p.into(), TagRule::All, pre)?
                };
                Ok((0..x.n)
                    .into_par_iter()
                    .map_init(Scratch::default, |s, i| h.hash_with(x.row(i), s))
                    .collect::<kenyon::Result<Vec<_>>>()?)
            })
            .collect(),
        _ => {
            let exp: Expansion = if method == "abl_dense_expansion_wta" {
                DenseProjection::gaussian(d, m, seed)?.into()
            } else {
                SparseProjection::fixed_fan_in(d, m, fan, seed)?.into()
            };
            let h = FlyHash::with_expansion(exp, TagRule::All, pre)?;
            let mut tag_rng = Rng::derive(seed, 77);
            let fixed: Vec<Vec<u32>> = KS
                .iter()
                .map(|&k| {
                    let mut u: Vec<u32> = tag_rng
                        .sample_distinct(m, k)
                        .into_iter()
                        .map(|v| v as u32)
                        .collect();
                    u.sort_unstable();
                    u
                })
                .collect();
            let per_item: Vec<Vec<KcCode>> = (0..x.n)
                .into_par_iter()
                .map_init(
                    || (Scratch::default(), Vec::new()),
                    |(s, idx), i| -> kenyon::Result<Vec<KcCode>> {
                        let act = h.activations(x.row(i), s)?;
                        KS.iter()
                            .zip(&fixed)
                            .map(|(&k, fx)| {
                                let units = if method == "abl_no_wta_random_tag" {
                                    fx.clone()
                                } else {
                                    wta::top_k(&act, k, idx)
                                };
                                let vals = units.iter().map(|&u| act[u as usize]).collect();
                                KcCode::new(m, units, vals)
                            })
                            .collect()
                    },
                )
                .collect::<kenyon::Result<_>>()?;
            Ok((0..KS.len())
                .map(|ki| per_item.iter().map(|c| c[ki].clone()).collect())
                .collect())
        }
    }
}

fn main() -> Result<()> {
    let trials: usize = std::env::args()
        .nth(1)
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(50);
    let (x, _) = data::mnist_split(&data_dir().join("mnist"), false)?;
    eprintln!("MNIST test: {} x {}", x.n, x.d);
    // results[method][k] = per-trial (ap_r, ap_hits)
    let mut res: Vec<Vec<Vec<(f64, f64)>>> = vec![vec![Vec::new(); KS.len()]; METHODS.len()];
    for t in 0..trials {
        let seed = 0x5231_0000 + t as u64;
        let mut rng = Rng::new(seed);
        let mut order: Vec<u32> = (0..x.n as u32).collect();
        rng.shuffle(&mut order); // order[i] = random rank of item i (tie-break)
        let queries = rng.sample_distinct(x.n, N_QUERIES);
        let truth: Vec<Vec<u32>> = queries
            .par_iter()
            .map(|&q| {
                let dist: Vec<f64> = (0..x.n).map(|i| sq_dist(x.row(q), x.row(i))).collect();
                top_r_by(&dist, &order, q, R)
            })
            .collect();
        for (mi, m) in METHODS.iter().enumerate() {
            let codes = codes_for(m.name, &x, seed.wrapping_mul(31).wrapping_add(mi as u64))?;
            for (ki, ck) in codes.iter().enumerate() {
                let aps: Vec<(f64, f64)> = queries
                    .par_iter()
                    .zip(&truth)
                    .map(|(&q, tr)| -> kenyon::Result<(f64, f64)> {
                        let mut dist = vec![0f64; x.n];
                        for (i, c) in ck.iter().enumerate() {
                            dist[i] = match m.rank {
                                Rank::Euclid => ck[q].euclidean_sq(c)?,
                                Rank::Overlap => -(ck[q].overlap(c)? as f64),
                            };
                        }
                        let pred = top_r_by(&dist, &order, q, R);
                        Ok(average_precision(&pred, tr, R))
                    })
                    .collect::<kenyon::Result<_>>()?;
                let a = mean(&aps.iter().map(|p| p.0).collect::<Vec<_>>());
                let b = mean(&aps.iter().map(|p| p.1).collect::<Vec<_>>());
                res[mi][ki].push((a, b));
            }
        }
        eprintln!(
            "trial {t}: fly k4 {:.1} lsh k4 {:.1}",
            100.0 * res[0][0].last().map_or(0.0, |p| p.0),
            100.0 * res[1][0].last().map_or(0.0, |p| p.0)
        );
    }
    let mut json = String::from("{\n  \"experiment\": \"r1_flyhash_mnist\",\n");
    json += &format!(
        "  \"trials\": {trials},\n  \"n\": {}, \"queries\": {N_QUERIES}, \"r\": {R},\n  \"rows\": [\n",
        x.n
    );
    println!("| method | k | mAP (AP@200 / 200) % | sd | mAP (/ hits) % |");
    println!("|---|---|---|---|---|");
    let mut rows = Vec::new();
    for (mi, m) in METHODS.iter().enumerate() {
        for (ki, &k) in KS.iter().enumerate() {
            let a: Vec<f64> = res[mi][ki].iter().map(|p| 100.0 * p.0).collect();
            let b: Vec<f64> = res[mi][ki].iter().map(|p| 100.0 * p.1).collect();
            println!(
                "| {} | {k} | {:.1} | {:.2} | {:.1} |",
                m.name,
                mean(&a),
                sd(&a),
                mean(&b)
            );
            rows.push(format!(
                "    {{\"method\": \"{}\", \"k\": {k}, \"map_r\": {:.4}, \"map_r_sd\": {:.4}, \"map_hits\": {:.4}, \"map_hits_sd\": {:.4}}}",
                m.name,
                mean(&a),
                sd(&a),
                mean(&b),
                sd(&b)
            ));
        }
    }
    json += &rows.join(",\n");
    json += "\n  ]\n}\n";
    std::fs::write(out_dir().join("r1_flyhash.json"), json)?;
    Ok(())
}
