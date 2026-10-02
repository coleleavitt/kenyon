//! R2: fly Bloom filter novelty (Dasgupta et al. 2018 PNAS, Fig. 3).
//! Protocol and bars: RESULTS.md part 1, R2. Usage: `r2_bloom [sift|odors|all]`.

use anyhow::Result;
use kenyon::{
    BloomParams, DenseProjection, Expansion, FlyBloom, FlyHash, KcCode, Preprocess, Rng, Scratch, SparseProjection,
    TagRule,
};
use kenyon_repro::metrics::{pearson, sq_dist};
use kenyon_repro::stats::{mean, sd};
use kenyon_repro::{Matrix, data, data_dir, out_dir};
use rayon::prelude::*;

const KS: [usize; 10] = [5, 10, 15, 20, 25, 30, 35, 40, 45, 50];
const LSBF_BITS: [usize; 4] = [4, 8, 12, 16];

/// Per item: the top-`kmax` units ordered by activation (desc), with values.
fn ranked_winners(h: &FlyHash, x: &Matrix) -> Result<Vec<Vec<(u32, f32)>>> {
    let v = (0..x.n)
        .into_par_iter()
        .map_init(Scratch::default, |s, i| -> kenyon::Result<Vec<(u32, f32)>> {
            let c = h.hash_with(x.row(i), s)?;
            let mut p: Vec<(u32, f32)> = c.indices().iter().copied().zip(c.values().iter().copied()).collect();
            p.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
            Ok(p)
        })
        .collect::<kenyon::Result<_>>()?;
    Ok(v)
}

fn code_k(m: usize, w: &[(u32, f32)], k: usize) -> kenyon::Result<KcCode> {
    let (i, v): (Vec<u32>, Vec<f32>) = w.iter().take(k).copied().unzip();
    KcCode::new(m, i, v)
}

fn folds(n: usize, nf: usize, seed: u64) -> Vec<Vec<usize>> {
    let mut perm: Vec<usize> = (0..n).collect();
    Rng::new(seed).shuffle(&mut perm);
    (0..nf).map(|f| perm[f * n / nf..(f + 1) * n / nf].to_vec()).collect()
}

/// Ground-truth novelty for each fold's test items (min distance to train).
fn truth(x: &Matrix, fs: &[Vec<usize>]) -> Vec<Vec<f64>> {
    fs.iter()
        .enumerate()
        .map(|(f, test)| {
            let train: Vec<usize> = fs.iter().enumerate().filter(|(g, _)| *g != f).flat_map(|(_, v)| v.clone()).collect();
            test.par_iter()
                .map(|&t| train.iter().map(|&s| sq_dist(x.row(t), x.row(s))).fold(f64::INFINITY, f64::min).sqrt())
                .collect()
        })
        .collect()
}

/// Correlation per fold for a winner-based filter (fly and its ablations).
fn fly_corr(m: usize, winners: &[Vec<(u32, f32)>], fs: &[Vec<usize>], gt: &[Vec<f64>], k: usize, h: &FlyHash) -> Result<Vec<f64>> {
    let mut out = Vec::new();
    for (f, test) in fs.iter().enumerate() {
        let mut bf = FlyBloom::new(h.clone(), BloomParams::STATIC)?;
        for (g, tr) in fs.iter().enumerate() {
            if g != f {
                for &i in tr {
                    bf.insert_code(&code_k(m, &winners[i], k)?)?;
                }
            }
        }
        let pred: Vec<f64> = test.iter().map(|&i| bf.novelty_code(&code_k(m, &winners[i], k)?)).collect::<kenyon::Result<_>>()?;
        out.push(pearson(&pred, &gt[f]));
    }
    Ok(out)
}

fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Classical (inverted) Bloom filter: k uniform hashes of the exact item.
fn classic_corr(x: &Matrix, fs: &[Vec<usize>], gt: &[Vec<f64>], k: usize, m: usize, seed: u64) -> Vec<f64> {
    let item_hash = |i: usize| x.row(i).iter().fold(seed, |h, v| mix(h ^ u64::from(v.to_bits())));
    let pos = |i: usize, j: usize| (mix(item_hash(i) ^ (j as u64).wrapping_mul(0x9E37)) % m as u64) as usize;
    fs.iter()
        .enumerate()
        .map(|(f, test)| {
            let mut bits = vec![true; m];
            for (g, tr) in fs.iter().enumerate() {
                if g != f {
                    for &i in tr {
                        for j in 0..k {
                            bits[pos(i, j)] = false;
                        }
                    }
                }
            }
            let pred: Vec<f64> = test.iter().map(|&i| (0..k).filter(|&j| bits[pos(i, j)]).count() as f64 / k as f64).collect();
            pearson(&pred, &gt[f])
        })
        .collect()
}

/// LSBF (Kirsch & Mitzenmacher 2006 form): k LSH functions, each b sign bits
/// of Gaussian projections, hashed into its own block of m/k cells.
fn lsbf_corr(x: &Matrix, fs: &[Vec<usize>], gt: &[Vec<f64>], k: usize, m: usize, b: usize, seed: u64) -> Result<Vec<f64>> {
    let block = m / k;
    let proj = DenseProjection::gaussian(x.d, k * b, seed)?;
    let pos: Vec<Vec<usize>> = (0..x.n)
        .into_par_iter()
        .map(|i| -> kenyon::Result<Vec<usize>> {
            let mut pre = Vec::new();
            Preprocess::CenterMean.apply_into(x.row(i), &mut pre);
            let mut a = vec![0f32; k * b];
            proj.project_into(&pre, &mut a)?;
            Ok((0..k)
                .map(|j| {
                    let key = a[j * b..(j + 1) * b].iter().fold(0u64, |acc, v| (acc << 1) | u64::from(*v > 0.0));
                    j * block + (mix(key ^ seed ^ (j as u64) << 40) % block as u64) as usize
                })
                .collect())
        })
        .collect::<kenyon::Result<_>>()?;
    Ok(fs
        .iter()
        .enumerate()
        .map(|(f, test)| {
            let mut bits = vec![true; m];
            for (g, tr) in fs.iter().enumerate() {
                if g != f {
                    for &i in tr {
                        for &p in &pos[i] {
                            bits[p] = false;
                        }
                    }
                }
            }
            let pred: Vec<f64> = test.iter().map(|&i| pos[i].iter().filter(|&&p| bits[p]).count() as f64 / k as f64).collect();
            pearson(&pred, &gt[f])
        })
        .collect())
}

struct Row {
    dataset: String,
    method: String,
    k: usize,
    mean: f64,
    sd: f64,
}

fn run(name: &str, x: &Matrix, fan_in: usize, seeds: &[u64], rows: &mut Vec<Row>) -> Result<()> {
    let n_train = x.n - x.n / 10;
    let m = 30 * n_train;
    let kmax = *KS.iter().max().unwrap_or(&50);
    eprintln!("{name}: n={} d={} m={m} fan_in={fan_in}", x.n, x.d);
    // method -> k -> per-(seed, fold) correlations
    let mut acc: std::collections::BTreeMap<(String, usize), Vec<f64>> = Default::default();
    for &seed in seeds {
        let fs = folds(x.n, 10, seed);
        let gt = truth(x, &fs);
        let fly = FlyHash::with_expansion(
            SparseProjection::fixed_fan_in(x.d, m, fan_in, seed)?.into(),
            TagRule::WinnerTakeAll(kmax),
            Preprocess::CenterMean,
        )?;
        let dense: Expansion = DenseProjection::gaussian(x.d, m, seed ^ 0xD)?.into();
        let dense = FlyHash::with_expansion(dense, TagRule::WinnerTakeAll(kmax), Preprocess::CenterMean)?;
        let w_fly = ranked_winners(&fly, x)?;
        let w_dense = ranked_winners(&dense, x)?;
        for &k in &KS {
            acc.entry(("fly".into(), k)).or_default().extend(fly_corr(m, &w_fly, &fs, &gt, k, &fly)?);
            acc.entry(("abl_dense_expansion".into(), k)).or_default().extend(fly_corr(m, &w_dense, &fs, &gt, k, &dense)?);
            // No WTA: the same k units for every item (random tag).
            let tag: Vec<(u32, f32)> = Rng::derive(seed, k as u64).sample_distinct(m, k).into_iter().map(|u| (u as u32, 1.0)).collect();
            let w_tag = vec![tag; x.n];
            acc.entry(("abl_no_wta_random_tag".into(), k)).or_default().extend(fly_corr(m, &w_tag, &fs, &gt, k, &fly)?);
            acc.entry(("classic_bloom".into(), k)).or_default().extend(classic_corr(x, &fs, &gt, k, m, seed));
            let mut best: Option<(f64, Vec<f64>, usize)> = None;
            for &b in &LSBF_BITS {
                let c = lsbf_corr(x, &fs, &gt, k, m, b, seed ^ 0x15bf)?;
                let mc = mean(&c);
                if best.as_ref().is_none_or(|bb| mc > bb.0) {
                    best = Some((mc, c, b));
                }
            }
            if let Some((_, c, b)) = best {
                eprintln!("  seed {seed} k {k}: lsbf best b={b}");
                acc.entry(("lsbf_best_b".into(), k)).or_default().extend(c);
            }
        }
        eprintln!("  seed {seed} done: fly k20 {:.3}", mean(&acc[&("fly".to_string(), 20)]));
    }
    for ((method, k), v) in acc {
        rows.push(Row { dataset: name.into(), method, k, mean: mean(&v), sd: sd(&v) });
    }
    Ok(())
}

fn main() -> Result<()> {
    let which = std::env::args().nth(1).unwrap_or_else(|| "all".into());
    let mut rows = Vec::new();
    if which == "sift" || which == "all" {
        let base = data::fvecs(&data_dir().join("siftsmall/siftsmall/siftsmall_base.fvecs"))?;
        let pick = Rng::new(0x5_1F7).sample_distinct(base.n, 5000);
        let x = base.select(&pick);
        run("sift5000", &x, x.d / 10, &[1], &mut rows)?;
    }
    if which == "odors" || which == "all" {
        let (_, x) = data::hallem(&data_dir().join("door"))?;
        let seeds: Vec<u64> = (0..20).collect();
        run("hallem110", &x, (x.d / 10).max(1), &seeds, &mut rows)?;
        // Exploratory: fan-in 6 (the fly's ~6 claws).
        run("hallem110_fanin6", &x, 6, &seeds, &mut rows)?;
    }
    println!("| dataset | method | k | r (mean) | sd |");
    println!("|---|---|---|---|---|");
    let mut json = vec![];
    for r in &rows {
        println!("| {} | {} | {} | {:.3} | {:.3} |", r.dataset, r.method, r.k, r.mean, r.sd);
        json.push(format!(
            "    {{\"dataset\": \"{}\", \"method\": \"{}\", \"k\": {}, \"r\": {:.4}, \"sd\": {:.4}}}",
            r.dataset, r.method, r.k, r.mean, r.sd
        ));
    }
    std::fs::write(
        out_dir().join(format!("r2_bloom_{which}.json")),
        format!("{{\n  \"experiment\": \"r2_bloom\",\n  \"rows\": [\n{}\n  ]\n}}\n", json.join(",\n")),
    )?;
    Ok(())
}
