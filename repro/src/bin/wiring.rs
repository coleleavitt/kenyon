//! Wiring study: does the real FlyWire v783 PN->KC wiring hash better than
//! random? Preregistration: research/wiring/PREREG.md. Usage:
//! `wiring [seeds]` (default 20). Writes results/wiring_{pooled,right}.json.

use std::collections::{BTreeMap, HashMap, HashSet};

use anyhow::{Result, ensure};
use kenyon::{
    BitCode, BloomParams, FlyBloom, FlyHash, KcCode, Preprocess, Rng, Scratch, SparseProjection,
    TagRule, wta,
};
use kenyon_repro::data::{self, HALLEM_RECEPTORS, PnKc, glomerulus};
use kenyon_repro::metrics::{average_precision, pearson, sq_dist, top_r_by};
use kenyon_repro::stats::{mean, paired, sd};
use kenyon_repro::{Matrix, data_dir, out_dir};
use rayon::prelude::*;

type Edges = Vec<(usize, usize, f32)>;

const ARMS: [&str; 7] = ["R-b", "R-w", "U-b", "U-w", "D-b", "D-w", "S-w"];
const CONTRASTS: [(&str, &str, &str); 5] = [
    ("P1 wiring beyond degrees", "R-b", "D-b"),
    ("P2 degree heterogeneity", "D-b", "U-b"),
    ("P3 weights", "R-w", "S-w"),
    ("real vs uniform (binary)", "R-b", "U-b"),
    ("real vs deg-preserving (weighted)", "R-w", "D-w"),
];

// ---------- graph and nulls ----------

fn degrees(e: &Edges, n_pn: usize, n_kc: usize) -> (Vec<usize>, Vec<usize>) {
    let (mut o, mut i) = (vec![0; n_pn], vec![0; n_kc]);
    for &(p, k, _) in e {
        o[p] += 1;
        i[k] += 1;
    }
    (o, i)
}

fn binary(e: &Edges) -> Edges {
    e.iter().map(|&(p, k, _)| (p, k, 1.0)).collect()
}

fn permuted_weights(e: &Edges, weights: &[f32], rng: &mut Rng) -> Edges {
    let mut w = weights.to_vec();
    rng.shuffle(&mut w);
    e.iter().zip(w).map(|(&(p, k, _), w)| (p, k, w)).collect()
}

/// Each KC keeps its in-degree; its PNs are uniform without replacement.
fn uniform(e: &Edges, n_pn: usize, n_kc: usize, rng: &mut Rng) -> Edges {
    let (_, indeg) = degrees(e, n_pn, n_kc);
    let mut out = Vec::with_capacity(e.len());
    for (k, &d) in indeg.iter().enumerate() {
        for p in rng.sample_distinct(n_pn, d) {
            out.push((p, k, 1.0));
        }
    }
    out
}

/// Degree-preserving rewire by double-edge swaps (no multi-edges),
/// `10 * E` accepted swaps.
fn rewire(e: &Edges, rng: &mut Rng) -> Edges {
    let mut ed: Vec<(usize, usize)> = e.iter().map(|&(p, k, _)| (p, k)).collect();
    let mut set: HashSet<(usize, usize)> = ed.iter().copied().collect();
    let target = 10 * ed.len();
    let (mut accepted, mut tries) = (0, 0usize);
    while accepted < target && tries < 1000 * target {
        tries += 1;
        let (a, b) = (rng.below(ed.len()), rng.below(ed.len()));
        let ((p1, k1), (p2, k2)) = (ed[a], ed[b]);
        if p1 == p2 || k1 == k2 || set.contains(&(p1, k2)) || set.contains(&(p2, k1)) {
            continue;
        }
        set.remove(&(p1, k1));
        set.remove(&(p2, k2));
        set.insert((p1, k2));
        set.insert((p2, k1));
        ed[a] = (p1, k2);
        ed[b] = (p2, k1);
        accepted += 1;
    }
    ed.into_iter().map(|(p, k)| (p, k, 1.0)).collect()
}

fn arm_edges(arm: &str, real: &Edges, n_pn: usize, n_kc: usize, seed: u64) -> Edges {
    let weights: Vec<f32> = real.iter().map(|e| e.2).collect();
    let mut rng = Rng::derive(
        seed,
        arm.bytes()
            .fold(0u64, |h, b| h.wrapping_mul(131).wrapping_add(u64::from(b))),
    );
    match arm {
        "R-b" => binary(real),
        "R-w" => real.clone(),
        "U-b" => uniform(real, n_pn, n_kc, &mut rng),
        "U-w" => {
            let u = uniform(real, n_pn, n_kc, &mut rng);
            permuted_weights(&u, &weights, &mut rng)
        }
        "D-b" => rewire(real, &mut rng),
        "D-w" => {
            let d = rewire(real, &mut rng);
            permuted_weights(&d, &weights, &mut rng)
        }
        "S-w" => permuted_weights(real, &weights, &mut rng),
        _ => Vec::new(),
    }
}

// ---------- tasks ----------

struct Codes {
    /// k = 5% binary codes as bitsets.
    bio: Vec<BitCode>,
    /// k = 16 valued codes.
    short: Vec<KcCode>,
    /// Ranked winners (desc), up to 40, for the Bloom filter.
    ranked: Vec<Vec<u32>>,
}

fn encode(h: &FlyHash, x: &Matrix, k_bio: usize) -> Result<Codes> {
    let m = h.n_kc();
    let per: Vec<(BitCode, KcCode, Vec<u32>)> = (0..x.n)
        .into_par_iter()
        .map_init(
            || (Scratch::default(), Vec::new()),
            |(s, idx), i| -> kenyon::Result<(BitCode, KcCode, Vec<u32>)> {
                let act = h.activations(x.row(i), s)?;
                let bio = KcCode::new(m, wta::top_k(&act, k_bio, idx), vec![1.0; k_bio.min(m)])?
                    .to_bits();
                let u16 = wta::top_k(&act, 16, idx);
                let v16 = u16.iter().map(|&u| act[u as usize]).collect();
                let mut r = wta::top_k(&act, 40, idx);
                r.sort_by(|a, b| act[*b as usize].total_cmp(&act[*a as usize]).then(a.cmp(b)));
                Ok((bio, KcCode::new(m, u16, v16)?, r))
            },
        )
        .collect::<kenyon::Result<_>>()?;
    let mut c = Codes {
        bio: Vec::new(),
        short: Vec::new(),
        ranked: Vec::new(),
    };
    for (a, b, r) in per {
        c.bio.push(a);
        c.short.push(b);
        c.ranked.push(r);
    }
    Ok(c)
}

/// mAP (AP / R) for both hash settings: (bio k=5% overlap, k=16 valued Euclid).
fn retrieval(
    codes: &Codes,
    queries: &[usize],
    truth: &[Vec<u32>],
    order: &[u32],
    r: usize,
) -> Result<(f64, f64, f64, f64)> {
    let n = codes.bio.len();
    let res: Vec<(f64, f64, f64, f64)> = queries
        .par_iter()
        .zip(truth)
        .map(|(&q, tr)| -> kenyon::Result<(f64, f64, f64, f64)> {
            let mut d1 = vec![0f64; n];
            let mut d2 = vec![0f64; n];
            for i in 0..n {
                d1[i] = -(codes.bio[q].overlap(&codes.bio[i])? as f64);
                d2[i] = codes.short[q].euclidean_sq(&codes.short[i])?;
            }
            let a = average_precision(&top_r_by(&d1, order, q, r), tr, r);
            let b = average_precision(&top_r_by(&d2, order, q, r), tr, r);
            Ok((a.0, b.0, a.1, b.1))
        })
        .collect::<kenyon::Result<_>>()?;
    Ok((
        mean(&res.iter().map(|v| v.0).collect::<Vec<_>>()),
        mean(&res.iter().map(|v| v.1).collect::<Vec<_>>()),
        mean(&res.iter().map(|v| v.2).collect::<Vec<_>>()),
        mean(&res.iter().map(|v| v.3).collect::<Vec<_>>()),
    ))
}

/// Fly Bloom filter novelty, 10-fold CV: mean Pearson r per k.
fn novelty(
    h: &FlyHash,
    ranked: &[Vec<u32>],
    gt_space: &Matrix,
    ks: &[usize],
    seed: u64,
) -> Result<Vec<f64>> {
    let n = ranked.len();
    let m = h.n_kc();
    let mut perm: Vec<usize> = (0..n).collect();
    Rng::derive(seed, 0xf01d).shuffle(&mut perm);
    let folds: Vec<Vec<usize>> = (0..10)
        .map(|f| perm[f * n / 10..(f + 1) * n / 10].to_vec())
        .collect();
    let mut out = Vec::new();
    for &k in ks {
        let mut rs = Vec::new();
        for (f, test) in folds.iter().enumerate() {
            let mut bf = FlyBloom::new(h.clone(), BloomParams::STATIC)?;
            let train: Vec<usize> = folds
                .iter()
                .enumerate()
                .filter(|(g, _)| *g != f)
                .flat_map(|(_, v)| v.clone())
                .collect();
            let code = |i: usize| {
                let u: Vec<u32> = ranked[i].iter().take(k).copied().collect();
                let n = u.len();
                KcCode::new(m, u, vec![1.0; n])
            };
            for &i in &train {
                bf.insert_code(&code(i)?)?;
            }
            let pred: Vec<f64> = test
                .iter()
                .map(|&i| bf.novelty_code(&code(i)?))
                .collect::<kenyon::Result<_>>()?;
            let gt: Vec<f64> = test
                .iter()
                .map(|&t| {
                    train
                        .iter()
                        .map(|&s| sq_dist(gt_space.row(t), gt_space.row(s)))
                        .fold(f64::INFINITY, f64::min)
                        .sqrt()
                })
                .collect();
            rs.push(pearson(&pred, &gt));
        }
        out.push(mean(&rs));
    }
    Ok(out)
}

const NOV_KS: [usize; 4] = [20, 5, 10, 40];

// ---------- main ----------

fn subgraph(g: &PnKc, right_only: bool) -> (Vec<usize>, Vec<usize>, Edges) {
    let keep_pn: Vec<usize> = (0..g.pns.len())
        .filter(|&i| !right_only || g.pns[i].side == "right")
        .collect();
    let keep_kc: Vec<usize> = (0..g.kcs.len())
        .filter(|&i| !right_only || g.kcs[i].side == "right")
        .collect();
    let pmap: HashMap<usize, usize> = keep_pn.iter().enumerate().map(|(a, &b)| (b, a)).collect();
    let kmap: HashMap<usize, usize> = keep_kc.iter().enumerate().map(|(a, &b)| (b, a)).collect();
    let mut e: Edges = g
        .edges
        .iter()
        .filter_map(|&(p, k, w)| Some((*pmap.get(&p)?, *kmap.get(&k)?, w)))
        .collect();
    // drop PNs / KCs left without edges
    let used_p: std::collections::BTreeSet<usize> = e.iter().map(|x| x.0).collect();
    let used_k: std::collections::BTreeSet<usize> = e.iter().map(|x| x.1).collect();
    let rp: HashMap<usize, usize> = used_p.iter().enumerate().map(|(a, &b)| (b, a)).collect();
    let rk: HashMap<usize, usize> = used_k.iter().enumerate().map(|(a, &b)| (b, a)).collect();
    for x in &mut e {
        *x = (rp[&x.0], rk[&x.1], x.2);
    }
    let pns = used_p.iter().map(|&i| keep_pn[i]).collect();
    let kcs = used_k.iter().map(|&i| keep_kc[i]).collect();
    (pns, kcs, e)
}

fn pct(v: &mut [usize], q: f64) -> usize {
    v.sort_unstable();
    v[((v.len() - 1) as f64 * q).round() as usize]
}

fn study(g: &PnKc, right_only: bool, n_seeds: u64, tag: &str) -> Result<String> {
    let label = format!("{tag}{}", if right_only { "right" } else { "pooled" });
    let (pn_idx, _kc_idx, real) = subgraph(g, right_only);
    let (n_pn, n_kc) = (
        pn_idx.len(),
        real.iter().map(|e| e.1).max().map_or(0, |m| m + 1),
    );
    let (mut od, mut id) = degrees(&real, n_pn, n_kc);
    let odm = od.iter().sum::<usize>() as f64 / n_pn as f64;
    let stats = format!(
        "{label}: {} pairs, {n_pn} PNs -> {n_kc} KCs; KC fan-in median {} (p10 {}, p90 {}); PN out-degree mean {:.1}, median {}, max {}; synapses {}",
        real.len(),
        pct(&mut id, 0.5),
        pct(&mut id, 0.1),
        pct(&mut id, 0.9),
        odm,
        pct(&mut od, 0.5),
        od.iter().max().copied().unwrap_or(0),
        real.iter().map(|e| e.2).sum::<f32>()
    );
    eprintln!("{stats}");
    let k_bio = (n_kc as f64 * 0.05).round() as usize;

    // Odour inputs on PN channels.
    let (_, hal) = data::hallem(&data_dir().join("door"), true)?;
    let mut glom_rate: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (j, (_, gl)) in HALLEM_RECEPTORS.iter().enumerate() {
        for g in *gl {
            glom_rate.entry(g).or_default().push(j);
        }
    }
    let mut odor_pn = vec![0f32; hal.n * n_pn];
    let mut n_driven = 0;
    for (pi, &orig) in pn_idx.iter().enumerate() {
        if let Some(gl) = glomerulus(&g.pns[orig])
            && let Some(rs) = glom_rate.get(gl)
        {
            n_driven += 1;
            for o in 0..hal.n {
                odor_pn[o * n_pn + pi] =
                    rs.iter().map(|&j| hal.row(o)[j]).sum::<f32>() / rs.len() as f32;
            }
        }
    }
    let odor_x = Matrix {
        n: hal.n,
        d: n_pn,
        data: odor_pn,
    };
    eprintln!(
        "{label}: odour input drives {n_driven} of {n_pn} PNs ({} glomeruli)",
        glom_rate.len()
    );
    let odor_order: Vec<u32> = (0..hal.n as u32).collect();
    let odor_truth: Vec<Vec<u32>> = (0..hal.n)
        .map(|q| {
            top_r_by(
                &(0..hal.n)
                    .map(|i| sq_dist(hal.row(q), hal.row(i)))
                    .collect::<Vec<_>>(),
                &odor_order,
                q,
                5,
            )
        })
        .collect();
    let odor_q: Vec<usize> = (0..hal.n).collect();

    let (mnist, _) = data::mnist_split(&data_dir().join("mnist"), false)?;
    // metric name -> arm -> per-seed values
    let mut vals: BTreeMap<String, BTreeMap<&str, Vec<f64>>> = BTreeMap::new();
    for seed in 0..n_seeds {
        let mut rng = Rng::derive(0x317e + seed, 1);
        let mut perm: Vec<usize> = (0..mnist.n).collect();
        rng.shuffle(&mut perm);
        let x = mnist.select(&perm);
        let mut gr = Rng::derive(0x317e + seed, 2);
        let gm: Vec<f32> = (0..n_pn * x.d)
            .map(|_| (gr.normal() / (x.d as f64).sqrt()) as f32)
            .collect();
        let xpn = Matrix {
            n: x.n,
            d: n_pn,
            data: (0..x.n)
                .into_par_iter()
                .flat_map_iter(|i| {
                    let r = x.row(i);
                    let gm = &gm;
                    (0..n_pn).map(move |p| {
                        gm[p * 784..(p + 1) * 784]
                            .iter()
                            .zip(r)
                            .map(|(a, b)| a * b)
                            .sum::<f32>()
                    })
                })
                .collect(),
        };
        let order: Vec<u32> = (0..x.n as u32).collect(); // x is already in random order
        let queries = rng.sample_distinct(x.n, 1000);
        let mut pre = Vec::new();
        let xc = Matrix {
            n: xpn.n,
            d: n_pn,
            data: (0..xpn.n)
                .flat_map(|i| {
                    Preprocess::CenterMean.apply_into(xpn.row(i), &mut pre);
                    pre.clone()
                })
                .collect(),
        };
        let truth: Vec<Vec<u32>> = queries
            .par_iter()
            .map(|&q| {
                top_r_by(
                    &(0..xc.n)
                        .map(|i| sq_dist(xc.row(q), xc.row(i)))
                        .collect::<Vec<_>>(),
                    &order,
                    q,
                    200,
                )
            })
            .collect();
        let nov_items: Vec<usize> = (0..200).collect();
        let nov_x = xc.select(&nov_items);
        for arm in ARMS {
            let e = arm_edges(arm, &real, n_pn, n_kc, seed);
            let (o2, i2) = degrees(&e, n_pn, n_kc);
            let (o1, i1) = degrees(&real, n_pn, n_kc);
            ensure!(i2 == i1, "{arm}: KC in-degrees changed");
            if arm.starts_with('D') || arm.starts_with('R') || arm.starts_with('S') {
                ensure!(o2 == o1, "{arm}: PN out-degrees changed");
            }
            let proj = SparseProjection::from_edges(n_pn, n_kc, &e)?;
            let h = FlyHash::with_expansion(proj.into(), TagRule::All, Preprocess::CenterMean)?;
            let codes = encode(&h, &xpn, k_bio)?;
            let (a, b, ah, bh) = retrieval(&codes, &queries, &truth, &order, 200)?;
            let oc = encode(&h, &odor_x, k_bio)?;
            let (oa, ob, oah, obh) = retrieval(&oc, &odor_q, &odor_truth, &odor_order, 5)?;
            let nov_codes = encode(&h, &nov_x, k_bio)?;
            let nm = novelty(&h, &nov_codes.ranked, &nov_x, &NOV_KS, seed)?;
            let no = novelty(&h, &oc.ranked, &hal, &NOV_KS, seed)?;
            let mut put = |name: String, v: f64| {
                vals.entry(name)
                    .or_default()
                    .entry(arm)
                    .or_default()
                    .push(v)
            };
            put("T1 mAP bio k=5% (AP/R)".into(), a);
            put("T1 mAP k=16 valued (AP/R)".into(), b);
            put("T1 mAP bio k=5% (AP/hits)".into(), ah);
            put("T1 mAP k=16 valued (AP/hits)".into(), bh);
            put("T2 odour mAP bio k=5% (AP/R)".into(), oa);
            put("T2 odour mAP k=16 valued (AP/R)".into(), ob);
            put("T2 odour mAP bio k=5% (AP/hits)".into(), oah);
            put("T2 odour mAP k=16 valued (AP/hits)".into(), obh);
            for (j, &k) in NOV_KS.iter().enumerate() {
                put(format!("T3 novelty MNIST-PN r (k={k})"), nm[j]);
                put(format!("T3 novelty odour r (k={k})"), no[j]);
            }
        }
        eprintln!(
            "{label} seed {seed}: T1 bio R-b {:.4} D-b {:.4} U-b {:.4}",
            vals["T1 mAP bio k=5% (AP/R)"]["R-b"][seed as usize],
            vals["T1 mAP bio k=5% (AP/R)"]["D-b"][seed as usize],
            vals["T1 mAP bio k=5% (AP/R)"]["U-b"][seed as usize]
        );
    }
    // Report.
    let mut md = format!(
        "## {label}\n\n{stats}\n\nodour input drives {n_driven} of {n_pn} PNs; k_bio = {k_bio}\n\n"
    );
    md += "### Arm means (sd over seeds)\n\n| metric | ";
    md += &ARMS.join(" | ");
    md += " |\n|---|";
    md += &"---|".repeat(ARMS.len());
    md += "\n";
    let mut json_rows = Vec::new();
    for (metric, arms) in &vals {
        md += &format!("| {metric} |");
        for arm in ARMS {
            let v = &arms[arm];
            md += &format!(" {:.4} ({:.4}) |", mean(v), sd(v));
        }
        md += "\n";
    }
    md += "\n### Contrasts (paired over seeds)\n\n| metric | contrast | mean diff | 95% t-CI | 95% boot CI | d_z | null sd | detected | > null sd |\n|---|---|---|---|---|---|---|---|---|\n";
    for (metric, arms) in &vals {
        for (ci, (cname, a, b)) in CONTRASTS.iter().enumerate() {
            let p = paired(&arms[a], &arms[b], 0xc1 + ci as u64);
            let null_sd = sd(&arms[b]);
            let detected = p.t_lo > 0.0 || p.t_hi < 0.0;
            let relevant = detected && p.mean_diff.abs() > null_sd;
            md += &format!(
                "| {metric} | {cname} ({a} - {b}) | {:+.4} | [{:+.4}, {:+.4}] | [{:+.4}, {:+.4}] | {:.2} | {:.4} | {} | {} |\n",
                p.mean_diff,
                p.t_lo,
                p.t_hi,
                p.boot_lo,
                p.boot_hi,
                p.dz,
                null_sd,
                detected,
                relevant
            );
            let rank = if metric.starts_with("T2") {
                // real is a single value: its rank among null seeds (1 = best)
                let r = arms[a][0];
                1 + arms[b].iter().filter(|&&v| v > r).count()
            } else {
                0
            };
            json_rows.push(format!(
                "    {{\"metric\": \"{metric}\", \"contrast\": \"{cname}\", \"a\": \"{a}\", \"b\": \"{b}\", \"mean_a\": {:.6}, \"mean_b\": {:.6}, \"diff\": {:.6}, \"t_lo\": {:.6}, \"t_hi\": {:.6}, \"boot_lo\": {:.6}, \"boot_hi\": {:.6}, \"dz\": {:.4}, \"null_sd\": {:.6}, \"detected\": {detected}, \"relevant\": {relevant}, \"t2_rank_of_a_among_b\": {rank}}}",
                mean(&arms[a]), mean(&arms[b]), p.mean_diff, p.t_lo, p.t_hi, p.boot_lo, p.boot_hi, p.dz, null_sd
            ));
        }
    }
    std::fs::write(
        out_dir().join(format!("wiring_{label}.json")),
        format!(
            "{{\n  \"study\": \"wiring_{label}\",\n  \"graph\": \"{stats}\",\n  \"seeds\": {n_seeds},\n  \"rows\": [\n{}\n  ]\n}}\n",
            json_rows.join(",\n")
        ),
    )?;
    Ok(md)
}

fn main() -> Result<()> {
    let n_seeds: u64 = std::env::args()
        .nth(1)
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(20);
    // KENYON_CONNECTOME=mcns runs the male-CNS replication (PREREG A2).
    let mcns = std::env::var("KENYON_CONNECTOME").is_ok_and(|v| v == "mcns");
    let home = std::path::PathBuf::from(std::env::var("HOME").unwrap_or_default());
    let (g, tag, title) = if mcns {
        let dir = std::env::var("KENYON_MCNS")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| home.join("flywire-data/mcns/1.0"));
        (
            data::mcns_pn_kc(&dir)?,
            "mcns_",
            "Wiring study results: male CNS v1.0 replication",
        )
    } else {
        let fw = std::env::var("KENYON_FLYWIRE")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| home.join("flywire-data/fafb/783"));
        (data::flywire_pn_kc(&fw)?, "", "Wiring study results")
    };
    let mut md = format!("# {title}\n\n");
    md += &study(&g, false, n_seeds, tag)?;
    md += "\n";
    md += &study(&g, true, n_seeds, tag)?;
    println!("{md}");
    Ok(())
}
