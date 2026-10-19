//! Throughput benchmarks (criterion). Memory is reported by the
//! `memory_bytes()` methods; `cargo run -p kenyon-repro --release --bin memory`
//! prints them for the same configurations.

use criterion::{BatchSize, Criterion, Throughput, black_box, criterion_group, criterion_main};
use kenyon::{
    BloomParams, DenseFly, FlyBloom, FlyHash, FlyHashParams, FlyIndex, FlyModel, FlyModelParams,
    Preprocess, Rng, Scratch,
};

fn data(n: usize, d: usize, seed: u64) -> Vec<Vec<f32>> {
    let mut r = Rng::new(seed);
    (0..n)
        .map(|_| (0..d).map(|_| r.next_f64() as f32).collect())
        .collect()
}

fn hashing(c: &mut Criterion) {
    let mut g = c.benchmark_group("hash");
    g.throughput(Throughput::Elements(1));
    for (name, d, k) in [
        ("mnist_d784_m7840_k4", 784, 4),
        ("sift_d128_m1280_k16", 128, 16),
    ] {
        let h = FlyHash::new(FlyHashParams::paper(d, k, 1)).expect("hasher");
        let xs = data(256, d, 2);
        let mut s = Scratch::default();
        let mut i = 0;
        g.bench_function(format!("flyhash/{name}"), |b| {
            b.iter(|| {
                i = (i + 1) % xs.len();
                black_box(h.hash_with(&xs[i], &mut s).expect("hash"))
            })
        });
        let lsh = FlyHash::lsh(d, k, Preprocess::CenterMean, 1).expect("lsh");
        g.bench_function(format!("lsh_dense/{name}"), |b| {
            b.iter(|| {
                i = (i + 1) % xs.len();
                black_box(lsh.hash_with(&xs[i], &mut s).expect("hash"))
            })
        });
    }
    let df = DenseFly::new(128, 1280, 12, 20, Preprocess::CenterMean, 1).expect("densefly");
    let xs = data(256, 128, 3);
    let mut s = Scratch::default();
    let mut i = 0;
    g.bench_function("densefly/sift_d128_m1280", |b| {
        b.iter(|| {
            i = (i + 1) % xs.len();
            black_box(df.hash_with(&xs[i], &mut s).expect("hash"))
        })
    });
    g.finish();
}

fn index(c: &mut Criterion) {
    let h = FlyHash::new(FlyHashParams::paper(128, 16, 1)).expect("hasher");
    let xs = data(10_000, 128, 4);
    let mut idx = FlyIndex::new(h.n_kc());
    for x in &xs {
        idx.insert(h.hash(x).expect("hash")).expect("insert");
    }
    let q = h.hash(&xs[5]).expect("hash");
    let mut g = c.benchmark_group("index_10k_sift_k16");
    g.throughput(Throughput::Elements(1));
    g.bench_function("query_overlap_top100", |b| {
        b.iter(|| black_box(idx.query_overlap(&q, 100).expect("q")))
    });
    g.bench_function("query_euclidean_top100", |b| {
        b.iter(|| black_box(idx.query_euclidean(&q, 100).expect("q")))
    });
    g.finish();
}

fn bloom(c: &mut Criterion) {
    let xs = data(512, 128, 5);
    let h = FlyHash::new(FlyHashParams {
        n_inputs: 128,
        n_kc: 150_000,
        fan_in: 12,
        k: 20,
        preprocess: Preprocess::CenterMean,
        seed: 1,
    })
    .expect("h");
    let mut g = c.benchmark_group("bloom_sift_m150k_k20");
    g.throughput(Throughput::Elements(1));
    let mut f = FlyBloom::new(
        h,
        BloomParams {
            decay: 0.5,
            recovery: 1e-4,
        },
    )
    .expect("f");
    let codes: Vec<_> = xs.iter().map(|x| f.hasher().hash(x).expect("h")).collect();
    let mut i = 0;
    g.bench_function("insert_code", |b| {
        b.iter(|| {
            i = (i + 1) % codes.len();
            f.insert_code(&codes[i]).expect("ins")
        })
    });
    g.bench_function("novelty_code", |b| {
        b.iter(|| {
            i = (i + 1) % codes.len();
            black_box(f.novelty_code(&codes[i]).expect("nov"))
        })
    });
    let mut s = Scratch::default();
    g.bench_function("observe_raw_vector", |b| {
        b.iter(|| {
            i = (i + 1) % xs.len();
            black_box(f.observe(&xs[i], &mut s).expect("obs"))
        })
    });
    g.finish();
}

fn model(c: &mut Criterion) {
    let xs = data(512, 84, 6);
    let mut g = c.benchmark_group("flymodel_mnist20");
    g.throughput(Throughput::Elements(1));
    let base = FlyModel::new(FlyModelParams::mnist20(1)).expect("m");
    let mut i = 0;
    g.bench_function("train_one", |b| {
        b.iter_batched_ref(
            || base.clone(),
            |m| {
                i = (i + 1) % xs.len();
                m.train_one(&xs[i], i % 20).expect("t")
            },
            BatchSize::LargeInput,
        )
    });
    let mut m = base.clone();
    for (j, x) in xs.iter().enumerate() {
        m.train_one(x, j % 20).expect("t");
    }
    g.bench_function("predict", |b| {
        b.iter(|| {
            i = (i + 1) % xs.len();
            black_box(m.predict(&xs[i]).expect("p"))
        })
    });
    g.finish();
}

criterion_group!(benches, hashing, index, bloom, model);
criterion_main!(benches);
