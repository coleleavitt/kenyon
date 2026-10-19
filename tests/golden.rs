//! Cross-version stability and behavioural properties.

use kenyon::{
    BloomParams, FlyBloom, FlyHash, FlyHashParams, FlyModel, FlyModelParams, Preprocess, Rng,
    SparseProjection,
};

fn vecs(seed: u64, n: usize, d: usize) -> Vec<Vec<f32>> {
    let mut r = Rng::new(seed);
    (0..n)
        .map(|_| (0..d).map(|_| r.next_f64() as f32).collect())
        .collect()
}

/// The generator's output for a fixed seed must never change: published
/// hashes depend on it.
#[test]
fn rng_golden() {
    let mut r = Rng::new(42);
    let got: Vec<u64> = (0..3).map(|_| r.next_u64()).collect();
    assert_eq!(got, GOLDEN_RNG);
}

/// The projection and the hash for a fixed seed must never change.
#[test]
fn flyhash_golden() {
    let p = SparseProjection::fixed_fan_in(50, 2000, 6, 7).unwrap();
    assert_eq!(p.row(0), GOLDEN_ROW0);
    let h = FlyHash::new(FlyHashParams {
        n_inputs: 50,
        n_kc: 2000,
        fan_in: 6,
        k: 8,
        preprocess: Preprocess::CenterMean,
        seed: 7,
    })
    .unwrap();
    let x: Vec<f32> = (0..50).map(|i| ((i * 37) % 11) as f32).collect();
    assert_eq!(h.hash(&x).unwrap().indices(), GOLDEN_CODE);
}

const GOLDEN_RNG: [u64; 3] = [
    1546998764402558742,
    6990951692964543102,
    12544586762248559009,
];
const GOLDEN_ROW0: &[u32] = &[7, 14, 22, 28, 36, 47];
const GOLDEN_CODE: &[u32] = &[132, 251, 359, 552, 574, 1066, 1334, 1595];

/// Locality: on average, nearer inputs share more winners (Dasgupta 2017
/// Lemma 1 + WTA).
#[test]
fn closer_inputs_share_more_winners() {
    let h = FlyHash::new(FlyHashParams::paper(64, 32, 3)).unwrap();
    let base = vecs(1, 200, 64);
    let mut r = Rng::new(9);
    let (mut near, mut far) = (0usize, 0usize);
    for x in &base {
        let small: Vec<f32> = x.iter().map(|v| v + 0.05 * r.normal() as f32).collect();
        let big: Vec<f32> = x.iter().map(|v| v + 0.5 * r.normal() as f32).collect();
        let c = h.hash(x).unwrap();
        near += c.overlap(&h.hash(&small).unwrap()).unwrap();
        far += c.overlap(&h.hash(&big).unwrap()).unwrap();
    }
    assert!(near > far, "{near} vs {far}");
}

/// Novelty grows with distance to the stored set (PNAS 2018 Eq. 1).
#[test]
fn novelty_tracks_distance() {
    let h = FlyHash::new(FlyHashParams {
        n_inputs: 32,
        n_kc: 6000,
        fan_in: 6,
        k: 20,
        preprocess: Preprocess::UnitNorm,
        seed: 5,
    })
    .unwrap();
    let mut f = FlyBloom::new(h, BloomParams::STATIC).unwrap();
    let stored = vecs(2, 100, 32);
    for x in &stored {
        f.insert(x).unwrap();
    }
    let mut r = Rng::new(3);
    let (mut near, mut far) = (0.0, 0.0);
    for x in &stored {
        let a: Vec<f32> = x.iter().map(|v| v + 0.02 * r.normal() as f32).collect();
        let b: Vec<f32> = (0..32).map(|_| r.next_f64() as f32).collect();
        near += f.novelty(&a).unwrap();
        far += f.novelty(&b).unwrap();
    }
    assert!(near < far, "{near} vs {far}");
}

/// FlyModel is deterministic for a seed and trains through its public API.
#[test]
fn flymodel_deterministic() {
    let xs = vecs(4, 40, 84);
    let mut a = FlyModel::new(FlyModelParams::mnist20(1)).unwrap();
    let mut b = FlyModel::new(FlyModelParams::mnist20(1)).unwrap();
    for (i, x) in xs.iter().enumerate() {
        a.train_one(x, i % 20).unwrap();
        b.train_one(x, i % 20).unwrap();
    }
    for x in &xs {
        assert_eq!(a.scores(x).unwrap(), b.scores(x).unwrap());
    }
}
