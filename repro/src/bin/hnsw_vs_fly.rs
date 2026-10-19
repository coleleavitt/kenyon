//! FlyHash vs HNSW head-to-head, exactly as research/hnsw/PREREG.md.
//!
//! Writes `results/hnsw_vs_fly.json`; prints the markdown report.

use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result, bail};
use instant_distance::{Builder, Point, Search};
use kenyon::{FlyHash, FlyHashParams, FlyIndex};
use kenyon_repro::metrics::sq_dist;
use kenyon_repro::{Matrix, data, data_dir, out_dir, stats};

const K_TOP: usize = 10;
const RERANK: usize = 100;
const FLY_KS: [usize; 3] = [16, 32, 64];
const FLY_SEEDS: [u64; 5] = [1, 2, 3, 4, 5];

#[derive(Clone)]
struct P(Vec<f32>);

impl Point for P {
    fn distance(&self, other: &Self) -> f32 {
        sq_dist(&self.0, &other.0).sqrt() as f32
    }
}

/// TEXMEX `.ivecs` (i32 dim, then dim i32, per vector).
fn ivecs(path: &Path) -> Result<Vec<Vec<usize>>> {
    let b = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    let mut out = Vec::new();
    let mut at = 0;
    while at < b.len() {
        let d = i32::from_le_bytes(b[at..at + 4].try_into()?) as usize;
        let row = (0..d)
            .map(|j| {
                let s = at + 4 + 4 * j;
                i32::from_le_bytes([b[s], b[s + 1], b[s + 2], b[s + 3]]) as usize
            })
            .collect();
        out.push(row);
        at += 4 + 4 * d;
    }
    Ok(out)
}

fn exact_top(base: &Matrix, q: &[f32], k: usize) -> Vec<usize> {
    let mut d: Vec<(f64, usize)> = (0..base.n).map(|i| (sq_dist(base.row(i), q), i)).collect();
    d.select_nth_unstable_by(k - 1, |a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut top: Vec<(f64, usize)> = d[..k].to_vec();
    top.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    top.into_iter().map(|t| t.1).collect()
}

fn recall(found: &[usize], truth: &[usize]) -> f64 {
    found.iter().filter(|f| truth[..K_TOP].contains(f)).count() as f64 / K_TOP as f64
}

#[derive(Clone)]
struct Row {
    method: String,
    recall: f64,
    latency_us: f64,
    memory: Option<usize>,
}

fn run(name: &str, base: &Matrix, queries: &Matrix, truth: &[Vec<usize>]) -> Result<Vec<Row>> {
    let mut rows = Vec::new();
    let nq = queries.n;
    // Exact.
    let t = Instant::now();
    let mut r = 0.0;
    for (i, gt) in truth.iter().enumerate().take(nq) {
        r += recall(&exact_top(base, queries.row(i), K_TOP), gt);
    }
    rows.push(Row {
        method: "exact".into(),
        recall: r / nq as f64,
        latency_us: t.elapsed().as_secs_f64() * 1e6 / nq as f64,
        memory: Some(base.data.len() * 4),
    });
    // HNSW.
    let points: Vec<P> = (0..base.n).map(|i| P(base.row(i).to_vec())).collect();
    let values: Vec<usize> = (0..base.n).collect();
    let tb = Instant::now();
    let map = Builder::default()
        .ef_construction(100)
        .ef_search(100)
        .seed(0)
        .build(points, values);
    let build_s = tb.elapsed().as_secs_f64();
    let mut search = Search::default();
    let t = Instant::now();
    let mut r = 0.0;
    for (i, gt) in truth.iter().enumerate().take(nq) {
        let q = P(queries.row(i).to_vec());
        let found: Vec<usize> = map
            .search(&q, &mut search)
            .take(K_TOP)
            .map(|m| *m.value)
            .collect();
        r += recall(&found, gt);
    }
    rows.push(Row {
        method: format!("hnsw (ef 100; build {build_s:.1} s, multi-threaded)"),
        recall: r / nq as f64,
        latency_us: t.elapsed().as_secs_f64() * 1e6 / nq as f64,
        memory: Some(base.data.len() * 4),
    });
    // FlyIndex, with and without re-rank, averaged over seeds.
    for k in FLY_KS {
        let (mut rec, mut lat, mut rec_rr, mut lat_rr, mut mem) =
            (vec![], vec![], vec![], vec![], 0);
        for seed in FLY_SEEDS {
            let h = FlyHash::new(FlyHashParams::paper(base.d, k, seed))?;
            let mut idx = FlyIndex::new(h.n_kc());
            for i in 0..base.n {
                idx.insert(h.hash(base.row(i))?)?;
            }
            mem = idx.memory_bytes() + h.memory_bytes();
            let (mut r1, mut r2) = (0.0, 0.0);
            let t = Instant::now();
            for (i, gt) in truth.iter().enumerate().take(nq) {
                let hits = idx.query_overlap(&h.hash(queries.row(i))?, K_TOP)?;
                let found: Vec<usize> = hits.iter().map(|x| x.id as usize).collect();
                r1 += recall(&found, gt);
            }
            lat.push(t.elapsed().as_secs_f64() * 1e6 / nq as f64);
            let t = Instant::now();
            for (i, gt) in truth.iter().enumerate().take(nq) {
                let q = queries.row(i);
                let hits = idx.query_overlap(&h.hash(q)?, RERANK)?;
                let mut c: Vec<(f64, usize)> = hits
                    .iter()
                    .map(|x| (sq_dist(base.row(x.id as usize), q), x.id as usize))
                    .collect();
                c.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
                let found: Vec<usize> = c.iter().take(K_TOP).map(|x| x.1).collect();
                r2 += recall(&found, gt);
            }
            lat_rr.push(t.elapsed().as_secs_f64() * 1e6 / nq as f64);
            rec.push(r1 / nq as f64);
            rec_rr.push(r2 / nq as f64);
        }
        rows.push(Row {
            method: format!("FlyIndex k={k}"),
            recall: stats::mean(&rec),
            latency_us: stats::mean(&lat),
            memory: Some(mem),
        });
        rows.push(Row {
            method: format!("FlyIndex k={k} + re-rank top {RERANK}"),
            recall: stats::mean(&rec_rr),
            latency_us: stats::mean(&lat_rr),
            memory: Some(mem + base.data.len() * 4),
        });
    }
    eprintln!("{name}: done");
    Ok(rows)
}

fn verdict(rows: &[Row]) -> &'static str {
    let Some(h) = rows.iter().find(|r| r.method.starts_with("hnsw")) else {
        return "no hnsw row";
    };
    let fly: Vec<&Row> = rows
        .iter()
        .filter(|r| r.method.starts_with("FlyIndex"))
        .collect();
    if fly
        .iter()
        .any(|f| f.recall >= h.recall - 0.02 && f.latency_us <= h.latency_us)
    {
        return "refuted";
    }
    let best = fly.iter().max_by(|a, b| a.recall.total_cmp(&b.recall));
    match best {
        Some(b) if h.recall >= b.recall + 0.10 && h.latency_us <= b.latency_us => "supported",
        _ => "mixed",
    }
}

fn main() -> Result<()> {
    let dir = data_dir();
    let head = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_default();
    eprintln!("hnsw_vs_fly: HEAD {head}");
    // SIFT10K with the shipped ground truth.
    let s = dir.join("siftsmall/siftsmall");
    let sift_base = data::fvecs(&s.join("siftsmall_base.fvecs"))?;
    let sift_q = data::fvecs(&s.join("siftsmall_query.fvecs"))?;
    let sift_gt = ivecs(&s.join("siftsmall_groundtruth.ivecs"))?;
    if sift_gt.len() != sift_q.n {
        bail!(
            "sift ground truth rows {} != queries {}",
            sift_gt.len(),
            sift_q.n
        );
    }
    // Sanity: our exact search agrees with the shipped ground truth.
    let agree = (0..sift_q.n)
        .map(|i| recall(&exact_top(&sift_base, sift_q.row(i), K_TOP), &sift_gt[i]))
        .sum::<f64>()
        / sift_q.n as f64;
    eprintln!("sift: exact vs shipped ground truth recall@10 = {agree:.4}");
    let sift = run("sift10k", &sift_base, &sift_q, &sift_gt)?;
    // MNIST: 60k train base, first 1,000 test queries, pixels / 255.
    let scale = |mut m: Matrix| {
        m.data.iter_mut().for_each(|v| *v /= 255.0);
        m
    };
    let (mb, _) = data::mnist_split(&dir.join("mnist"), true)?;
    let (mq, _) = data::mnist_split(&dir.join("mnist"), false)?;
    let mb = scale(mb);
    let mq = scale(mq).select(&(0..1000).collect::<Vec<_>>());
    let mnist_gt: Vec<Vec<usize>> = (0..mq.n)
        .map(|i| exact_top(&mb, mq.row(i), K_TOP))
        .collect();
    let mnist = run("mnist", &mb, &mq, &mnist_gt)?;

    let mut md = format!(
        "# FlyHash vs HNSW (research/hnsw/PREREG.md)\n\nHEAD `{head}`. Single-threaded query latency; FlyIndex averaged over seeds 1–5. SIFT exact-vs-shipped ground truth agreement: {agree:.4}.\n"
    );
    let mut json = format!("{{\"head\":\"{head}\",\"sift_gt_agreement\":{agree}");
    for (name, rows) in [("SIFT10K", &sift), ("MNIST (60k base, 1k queries)", &mnist)] {
        md.push_str(&format!(
            "\n## {name}\n\n| method | recall@10 | latency (µs/query) | memory (bytes) |\n|---|---|---|---|\n"
        ));
        for r in rows {
            md.push_str(&format!(
                "| {} | {:.3} | {:.1} | {} |\n",
                r.method,
                r.recall,
                r.latency_us,
                r.memory.map_or("—".into(), |m| m.to_string())
            ));
        }
        md.push_str(&format!("\nVerdict: **{}**\n", verdict(rows)));
        json.push_str(&format!(
            ",\"{}\":{{\"verdict\":\"{}\",\"rows\":[{}]}}",
            name.split_whitespace().next().unwrap_or(name),
            verdict(rows),
            rows.iter()
                .map(|r| format!(
                    "{{\"method\":\"{}\",\"recall\":{},\"latency_us\":{},\"memory\":{}}}",
                    r.method,
                    r.recall,
                    r.latency_us,
                    r.memory.unwrap_or(0)
                ))
                .collect::<Vec<_>>()
                .join(",")
        ));
    }
    md.push_str("\nHNSW memory counts the raw vectors only (the crate does not expose its graph size): a lower bound.\n");
    json.push_str("}\n");
    println!("{md}");
    std::fs::write(out_dir().join("hnsw_vs_fly.json"), json)?;
    Ok(())
}
