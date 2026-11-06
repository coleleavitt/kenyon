//! Serde round-trips for the state-bearing types.
//!
//! The crate's point is that the same seed produces the same hashes on every
//! platform and future version. Serialising a `FlyHash` and its filter after
//! some inserts must round-trip to a byte-identical structure that hashes
//! the same inputs the same way; a `FlyIndex` tombstone must survive the
//! round-trip too.

#![cfg(feature = "serde")]

use kenyon::{
    BloomParams, FlyBloom, FlyHash, FlyHashParams, FlyIndex, FlyModel, FlyModelParams, Rng,
};

fn sample(seed: u64, n: usize, d: usize) -> Vec<Vec<f32>> {
    let mut r = Rng::new(seed);
    (0..n)
        .map(|_| (0..d).map(|_| r.next_f64() as f32).collect())
        .collect()
}

#[test]
fn flyhash_roundtrips_and_hashes_the_same_inputs() {
    let h = FlyHash::new(FlyHashParams::paper(32, 8, 11)).unwrap();
    let json = serde_json::to_string(&h).unwrap();
    let back: FlyHash = serde_json::from_str(&json).unwrap();
    for x in sample(1, 10, 32) {
        assert_eq!(h.hash(&x).unwrap(), back.hash(&x).unwrap());
    }
}

#[test]
fn flybloom_roundtrips_in_both_cell_modes() {
    let h = FlyHash::new(FlyHashParams::paper(16, 4, 7)).unwrap();
    for params in [
        BloomParams::STATIC,
        BloomParams {
            decay: 0.5,
            recovery: 0.01,
        },
    ] {
        let mut f = FlyBloom::new(h.clone(), params).unwrap();
        for x in sample(9, 5, 16) {
            f.insert(&x).unwrap();
        }
        let bytes = serde_json::to_vec(&f).unwrap();
        let back: FlyBloom = serde_json::from_slice(&bytes).unwrap();
        for x in sample(9, 5, 16) {
            assert_eq!(f.novelty(&x).unwrap(), back.novelty(&x).unwrap());
        }
        assert_eq!(f.is_bit_packed(), back.is_bit_packed());
        assert_eq!(f.n_cells(), back.n_cells());
        assert_eq!(f.n_inserted(), back.n_inserted());
    }
}

#[test]
fn flyindex_roundtrips_with_tombstones() {
    let h = FlyHash::new(FlyHashParams::paper(32, 8, 3)).unwrap();
    let mut idx = FlyIndex::new(h.n_kc());
    let xs = sample(5, 8, 32);
    let mut ids = Vec::new();
    for x in &xs {
        ids.push(idx.insert(h.hash(x).unwrap()).unwrap());
    }
    assert!(idx.remove(ids[2]).unwrap());
    // Idempotency on a second removal.
    assert!(!idx.remove(ids[2]).unwrap());
    // Off-the-end never errors.
    assert!(!idx.remove(9999).unwrap());
    let before = idx.query_overlap(&h.hash(&xs[2]).unwrap(), 4).unwrap();
    assert!(before.iter().all(|hit| hit.id != ids[2]));

    let json = serde_json::to_string(&idx).unwrap();
    let back: FlyIndex = serde_json::from_str(&json).unwrap();
    assert_eq!(idx.len(), back.len());
    assert_eq!(idx.live_len(), back.live_len());
    assert_eq!(back.live_len(), xs.len() - 1);
    let after = back.query_overlap(&h.hash(&xs[2]).unwrap(), 4).unwrap();
    assert_eq!(after, before);
    assert!(back.get(ids[2]).is_none());
}

#[test]
fn flymodel_roundtrips_and_predicts_the_same() {
    let mut model = FlyModel::new(FlyModelParams {
        n_inputs: 16,
        n_kc: 160,
        fan_in: 4,
        n_active: 8,
        n_classes: 3,
        learning_rate: 0.05,
        forgetting: 0.0,
        coding: kenyon::Coding::Sparse,
        rule: kenyon::LearningRule::Associative,
        seed: 42,
    })
    .unwrap();
    for (label, x) in sample(2, 24, 16).into_iter().enumerate() {
        model.train_one(&x, label % 3).unwrap();
    }
    let bytes = serde_json::to_vec(&model).unwrap();
    let mut back: FlyModel = serde_json::from_slice(&bytes).unwrap();
    for x in sample(2, 8, 16) {
        assert_eq!(model.predict(&x).unwrap(), back.predict(&x).unwrap());
    }
}
