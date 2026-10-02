//! R2: fly Bloom filter novelty (Dasgupta et al. 2018 PNAS, Fig. 3).
//! Protocol and bars: RESULTS.md part 1, R2. Usage: `r2_bloom [sift|odors|all]`.

use anyhow::Result;
use kenyon::{
    BloomParams,
    DenseProjection,
    Expansion,
    FlyBloom,
    FlyHash,
    KcCode,
    Preprocess,
    Rng,
    Scratch,
    SparseProjection,
    TagRule,
};
use kenyon_repro::metrics::{pearson, sq_dist};
use kenyon_repro::stats::{mean, sd};
use kenyon_repro::{Matrix, data, data_dir, out_dir};
use rayon::prelude::*;

const KS: [usize; 5] = [5, 10, 20, 40, 50];

/// One evaluation protocol.
#[derive(Clone, Copy)]
struct Proto {
    /// Fly inputs per KC (`None` = 10% of d).
    fan_in: Option<usize>,
    pre: Preprocess,
    /// Ground truth by cosine distance (SI) or Euclidean (pre-SI guess).
    cosine: bool,
    /// Filter cells per stored item (30 primary, 10 secondary).
    m_factor: usize,
}

const SI: Proto = Proto {
    fan_in: Some(6),
    pre: Preprocess::UnitNorm,
    cosine: true,
    m_factor: 30,
};
const SI_10N: Proto = Proto { m_factor: 10, ..SI };
const GUESS: Proto = Proto {
    fan_in: None,
    pre: Preprocess::CenterMean,
    cosine: false,
    m_factor: 30,
};

/// Per item: the top-`kmax` units ordered by activation (desc), with values.
fn ranked_winners(h: &FlyHash, x: &Matrix) -> Result<Vec<Vec<(u32, f32)>>> {
    let v = (0..x.n)
        .into_par_iter()
        .map_init(
            Scratch::default,
            |s, i| -> kenyon::Result<Vec<(u32, f32)>> {
                let c = h.hash_with(x.row(i), s)?;
                let mut p: Vec<(u32, f32)> = c
                    .indices()
                    .iter()
                    .copied()
                    .zip(c.values().iter().copied())
                    .collect();
                p.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
                Ok(p)
            },
        )
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
    (0..nf)
        .map(|f| perm[f * n / nf..(f + 1) * n / nf].to_vec())
        .collect()
}

/// Ground-truth novelty for each fold's test items (min distance to train).
fn cos_dist(a: &[f32], b: &[f32]) -> f64 {
    let (mut ab, mut aa, mut bb) = (0f64, 0f64, 0f64);
    for (x, y) in a.iter().zip(b) {
        ab += f64::from(*x) * f64::from(*y);
        aa += f64::from(*x).powi(2);
        bb += f64::from(*y).powi(2);
    }
    if aa == 0.0 || bb == 0.0 {
        1.0
    } else {
        1.0 - ab / (aa * bb).sqrt()
    }
}

fn truth(x: &Matrix, fs: &[Vec<usize>], cosine: bool) -> Vec<Vec<f64>> {
    fs.iter()
        .enumerate()
        .map(|(f, test)| {
            let train: Vec<usize> = fs
                .iter()
                .enumerate()
                .filter(|(g, _)| *g != f)
                .flat_map(|(_, v)| v.clone())
                .collect();
            test.par_iter()
                .map(|&t| {
                    train
                        .iter()
                        .map(|&s| {
                            if cosine {
                                cos_dist(x.row(t), x.row(s))
                            } else {
                                sq_dist(x.row(t), x.row(s)).sqrt()
                            }
                        })
                        .fold(f64::INFINITY, f64::min)
                })
                .collect()
        })
        .collect()
}

/// Correlation per fold for a winner-based filter (fly and its ablations).
fn fly_corr(
    m: usize,
    winners: &[Vec<(u32, f32)>],
    fs: &[Vec<usize>],
    gt: &[Vec<f64>],
    k: usize,
    h: &FlyHash,
) -> Result<Vec<f64>> {
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
        let pred: Vec<f64> = test
            .iter()
            .map(|&i| bf.novelty_code(&code_k(m, &winners[i], k)?))
            .collect::<kenyon::Result<_>>()?;
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
fn classic_corr(
    x: &Matrix,
    fs: &[Vec<usize>],
    gt: &[Vec<f64>],
    k: usize,
    m: usize,
    seed: u64,
) -> Vec<f64> {
    let item_hash = |i: usize| {
        x.row(i)
            .iter()
            .fold(seed, |h, v| mix(h ^ u64::from(v.to_bits())))
    };
    let pos = |i: usize, j: usize| {
        (mix(item_hash(i) ^ (j as u64).wrapping_mul(0x9E37)) % m as u64) as usize
    };
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
            let pred: Vec<f64> = test
                .iter()
                .map(|&i| (0..k).filter(|&j| bits[pos(i, j)]).count() as f64 / k as f64)
                .collect();
            pearson(&pred, &gt[f])
        })
        .collect()
}

/// LSBF as in the PNAS SI, Alg. S2: k Gaussian d x 5 projections of the
/// unit-norm input, quantised by floor((s + OFFSET) / WIDTH) with OFFSET = 0,
/// WIDTH = 1, the integer vector hashed mod m.
fn lsbf_corr(
    x: &Matrix,
    fs: &[Vec<usize>],
    gt: &[Vec<f64>],
    k: usize,
    m: usize,
    seed: u64,
) -> Result<Vec<f64>> {
    const D_LSH: usize = 5;
    let proj = DenseProjection::gaussian(x.d, k * D_LSH, seed)?;
    let pos: Vec<Vec<usize>> = (0..x.n)
        .into_par_iter()
        .map(|i| -> kenyon::Result<Vec<usize>> {
            let mut pre = Vec::new();
            Preprocess::UnitNorm.apply_into(x.row(i), &mut pre);
            let mut a = vec![0f32; k * D_LSH];
            proj.project_into(&pre, &mut a)?;
            Ok((0..k)
                .map(|j| {
                    let key = a[j * D_LSH..(j + 1) * D_LSH]
                        .iter()
                        .fold(seed ^ ((j as u64) << 48), |acc, v| {
                            mix(acc ^ (v.floor() as i64 as u64))
                        });
                    (key % m as u64) as usize
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
            let pred: Vec<f64> = test
                .iter()
                .map(|&i| pos[i].iter().filter(|&&p| bits[p]).count() as f64 / k as f64)
                .collect();
            pearson(&pred, &gt[f])
        })
        .collect())
}

struct Row {
    dataset: String,
    method: String,
    k: usize,
    mean: f64,
    /// SD over all (seed, fold) correlations.
    sd: f64,
    /// SD of the per-seed fold-means (NaN with one seed).
    sd_seed: f64,
}

fn run(name: &str, x: &Matrix, p: Proto, seeds: &[u64], rows: &mut Vec<Row>) -> Result<()> {
    let n_train = x.n - x.n / 10;
    let m = p.m_factor * n_train;
    let fan_in = p.fan_in.unwrap_or((x.d / 10).max(1));
    let kmax = *KS.iter().max().unwrap_or(&50);
    eprintln!("{name}: n={} d={} m={m} fan_in={fan_in}", x.n, x.d);
    // method -> k -> per-(seed, fold) correlations
    let mut acc: std::collections::BTreeMap<(String, usize), Vec<f64>> = Default::default();
    let mut seed_means: std::collections::BTreeMap<(String, usize), Vec<f64>> = Default::default();
    for &seed in seeds {
        let fs = folds(x.n, 10, seed);
        let gt = truth(x, &fs, p.cosine);
        let fly = FlyHash::with_expansion(
            SparseProjection::fixed_fan_in(x.d, m, fan_in, seed)?.into(),
            TagRule::WinnerTakeAll(kmax),
            p.pre,
        )?;
        let dense: Expansion = DenseProjection::gaussian(x.d, m, seed ^ 0xD)?.into();
        let dense = FlyHash::with_expansion(dense, TagRule::WinnerTakeAll(kmax), p.pre)?;
        let w_fly = ranked_winners(&fly, x)?;
        let w_dense = ranked_winners(&dense, x)?;
        for &k in &KS {
            acc.entry(("fly".into(), k))
                .or_default()
                .extend(fly_corr(m, &w_fly, &fs, &gt, k, &fly)?);
            acc.entry(("abl_dense_expansion".into(), k))
                .or_default()
                .extend(fly_corr(m, &w_dense, &fs, &gt, k, &dense)?);
            // No WTA: the same k units for every item (random tag).
            let tag: Vec<(u32, f32)> = Rng::derive(seed, k as u64)
                .sample_distinct(m, k)
                .into_iter()
                .map(|u| (u as u32, 1.0))
                .collect();
            let w_tag = vec![tag; x.n];
            acc.entry(("abl_no_wta_random_tag".into(), k))
                .or_default()
                .extend(fly_corr(m, &w_tag, &fs, &gt, k, &fly)?);
            acc.entry(("classic_bloom".into(), k))
                .or_default()
                .extend(classic_corr(x, &fs, &gt, k, m, seed));
            acc.entry(("lsbf".into(), k)).or_default().extend(lsbf_corr(
                x,
                &fs,
                &gt,
                k,
                m,
                seed ^ 0x15bf,
            )?);
        }
        let per_seed = |meth: &str, k: usize| {
            let v = &acc[&(meth.to_string(), k)];
            mean(&v[v.len() - fs.len()..])
        };
        for &k in &KS {
            for meth in ["fly", "lsbf", "classic_bloom", "abl_dense_expansion"] {
                seed_means
                    .entry((meth.to_string(), k))
                    .or_default()
                    .push(per_seed(meth, k));
            }
        }
        eprintln!(
            "  seed {seed} done: fly k20 {:.3} k40 {:.3}",
            per_seed("fly", 20),
            per_seed("fly", 40)
        );
    }
    for ((method, k), v) in acc {
        let sm = seed_means
            .get(&(method.clone(), k))
            .map_or(f64::NAN, |s| sd(s));
        rows.push(Row {
            dataset: name.into(),
            method,
            k,
            mean: mean(&v),
            sd: sd(&v),
            sd_seed: sm,
        });
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
        let seeds: Vec<u64> = (0..3).collect();
        run("sift5000", &x, SI, &seeds, &mut rows)?;
        run("sift5000_m10n", &x, SI_10N, &seeds[..1], &mut rows)?;
        run("sift5000_prereg_guess", &x, GUESS, &seeds[..1], &mut rows)?;
    }
    if which == "odors" || which == "all" {
        let seeds: Vec<u64> = (0..20).collect();
        let (_, raw) = data::hallem(&data_dir().join("door"), false)?;
        run("hallem110", &raw, SI, &seeds, &mut rows)?;
        run("hallem110_m10n", &raw, SI_10N, &seeds, &mut rows)?;
        let (_, abs) = data::hallem(&data_dir().join("door"), true)?;
        run("hallem110_with_sfr", &abs, SI, &seeds, &mut rows)?;
        run(
            "hallem110_with_sfr_prereg_guess",
            &abs,
            GUESS,
            &seeds,
            &mut rows,
        )?;
    }
    println!("| dataset | method | k | r (mean) | sd (folds) | sd (seed means) |");
    println!("|---|---|---|---|---|---|");
    let mut json = vec![];
    for r in &rows {
        println!(
            "| {} | {} | {} | {:.3} | {:.3} | {:.3} |",
            r.dataset, r.method, r.k, r.mean, r.sd, r.sd_seed
        );
        json.push(format!(
            "    {{\"dataset\": \"{}\", \"method\": \"{}\", \"k\": {}, \"r\": {:.4}, \"sd\": {:.4}, \"sd_seed\": {:.4}}}",
            r.dataset, r.method, r.k, r.mean, r.sd, r.sd_seed
        ));
    }
    std::fs::write(
        out_dir().join(format!("r2_bloom_{which}.json")),
        format!(
            "{{\n  \"experiment\": \"r2_bloom\",\n  \"rows\": [\n{}\n  ]\n}}\n",
            json.join(",\n")
        ),
    )?;
    Ok(())
}
