//! The fly Bloom filter: a distance- and time-sensitive novelty detector.
//!
//! Dasgupta, Sheehan, Stevens & Navlakha, "A neural data structure for novelty
//! detection", *PNAS* 115(51):13093, 2018 (PMC6304992). The KC->MBON-alpha'3
//! synapses are the filter's `m` cells, all starting at weight 1. Storing an
//! item multiplies the weights of its `k` active KCs by `delta` and adds
//! `epsilon` to every other weight (Eq. 2). The novelty of a query is the mean
//! weight of its active KCs (1 = never seen anything like it, 0 = familiar).
//! With `delta = epsilon = 0` it is a distance-sensitive (inverted) Bloom
//! filter; `epsilon > 0` makes old memories recover (forgetting).
//!
//! Recovery is applied lazily: a weight that was last touched `t` insertions
//! ago has gained `t * epsilon` (capped at 1), which equals applying Eq. 2
//! eagerly, so an insert costs O(k) instead of O(m). The cap at 1 is INFERRED
//! from the paper's normalisation of novelty to [0, 1].

use crate::code::KcCode;
use crate::error::{KenyonError, Result, invalid};
use crate::hash::{FlyHash, Scratch};

/// Decay and recovery rates (PNAS 2018 Eq. 2).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BloomParams {
    /// Multiplier applied to an active KC's weight on insert, `0 <= delta < 1`.
    pub decay: f32,
    /// Amount every inactive weight recovers per insert, `0 <= epsilon <= 1`.
    pub recovery: f32,
}

impl BloomParams {
    /// `delta = epsilon = 0`: no time sensitivity (the paper's benchmark
    /// setting).
    pub const STATIC: Self = Self {
        decay: 0.0,
        recovery: 0.0,
    };
}

/// A fly Bloom filter over a [`FlyHash`].
#[derive(Debug, Clone)]
pub struct FlyBloom {
    hasher: FlyHash,
    params: BloomParams,
    weights: Vec<f32>,
    touched: Vec<u64>,
    clock: u64,
}

impl FlyBloom {
    /// New filter with every weight at 1. The hasher should use a
    /// winner-take-all tag; its code dimension is the filter size `m`.
    pub fn new(hasher: FlyHash, params: BloomParams) -> Result<Self> {
        if !(0.0..1.0).contains(&params.decay) {
            return Err(invalid(
                "decay",
                format!("must be in [0, 1), got {}", params.decay),
            ));
        }
        if !(0.0..=1.0).contains(&params.recovery) {
            return Err(invalid(
                "recovery",
                format!("must be in [0, 1], got {}", params.recovery),
            ));
        }
        let m = hasher.n_kc();
        // Timestamps are only needed for lazy recovery.
        let touched = if params.recovery > 0.0 {
            vec![0; m]
        } else {
            Vec::new()
        };
        Ok(Self {
            hasher,
            params,
            weights: vec![1.0; m],
            touched,
            clock: 0,
        })
    }

    /// The hash function.
    pub fn hasher(&self) -> &FlyHash {
        &self.hasher
    }

    /// Number of stored insertions.
    pub fn n_inserted(&self) -> u64 {
        self.clock
    }

    fn weight(&self, u: usize) -> f32 {
        let w = self.weights[u];
        if self.params.recovery == 0.0 || w >= 1.0 {
            return w;
        }
        let gained = (self.clock - self.touched[u]) as f64 * f64::from(self.params.recovery);
        (f64::from(w) + gained).min(1.0) as f32
    }

    fn check(&self, code: &KcCode) -> Result<()> {
        if code.dim() == self.weights.len() {
            Ok(())
        } else {
            Err(KenyonError::CodeMismatch {
                left: self.weights.len(),
                right: code.dim(),
            })
        }
    }

    /// Novelty in [0, 1] of an already-hashed item.
    pub fn novelty_code(&self, code: &KcCode) -> Result<f64> {
        self.check(code)?;
        if code.is_empty() {
            return Ok(1.0);
        }
        let s: f64 = code
            .indices()
            .iter()
            .map(|&u| f64::from(self.weight(u as usize)))
            .sum();
        Ok(s / code.len() as f64)
    }

    /// Store an already-hashed item.
    pub fn insert_code(&mut self, code: &KcCode) -> Result<()> {
        self.check(code)?;
        for &u in code.indices() {
            let u = u as usize;
            let w = self.weight(u) * self.params.decay;
            self.weights[u] = w;
        }
        self.clock += 1;
        // Active units were updated "at" this insertion: they do not recover
        // from it (Eq. 2 adds epsilon only to inactive KCs).
        if !self.touched.is_empty() {
            for &u in code.indices() {
                self.touched[u as usize] = self.clock;
            }
        }
        Ok(())
    }

    /// Novelty of `x` without storing it.
    pub fn novelty(&self, x: &[f32]) -> Result<f64> {
        let code = self.hasher.hash(x)?;
        self.novelty_code(&code)
    }

    /// Store `x`.
    pub fn insert(&mut self, x: &[f32]) -> Result<()> {
        let code = self.hasher.hash(x)?;
        self.insert_code(&code)
    }

    /// Score then store (the streaming use: "how new was this?").
    pub fn observe(&mut self, x: &[f32], s: &mut Scratch) -> Result<f64> {
        let code = self.hasher.hash_with(x, s)?;
        let n = self.novelty_code(&code)?;
        self.insert_code(&code)?;
        Ok(n)
    }

    /// Reset every weight to 1.
    pub fn clear(&mut self) {
        self.weights.iter_mut().for_each(|w| *w = 1.0);
        self.touched.iter_mut().for_each(|t| *t = 0);
        self.clock = 0;
    }

    /// Heap bytes held (weights + timestamps + hasher). Each cell is an `f32`
    /// (plus a `u64` timestamp when `recovery > 0`), not the single bit of a
    /// classical Bloom filter: 32-96x the paper's "m bits" storage.
    pub fn memory_bytes(&self) -> usize {
        self.weights.capacity() * 4 + self.touched.capacity() * 8 + self.hasher.memory_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FlyHashParams, Rng};

    fn filter(p: BloomParams) -> FlyBloom {
        FlyBloom::new(FlyHash::new(FlyHashParams::paper(32, 8, 7)).unwrap(), p).unwrap()
    }

    fn x(seed: u64) -> Vec<f32> {
        let mut r = Rng::new(seed);
        (0..32).map(|_| r.next_f64() as f32).collect()
    }

    #[test]
    fn seen_items_are_familiar() {
        let mut f = filter(BloomParams::STATIC);
        assert_eq!(f.novelty(&x(1)).unwrap(), 1.0);
        f.insert(&x(1)).unwrap();
        assert_eq!(f.novelty(&x(1)).unwrap(), 0.0);
        assert!(f.novelty(&x(2)).unwrap() > 0.5);
    }

    #[test]
    fn lazy_recovery_matches_eager_eq2() {
        let p = BloomParams {
            decay: 0.5,
            recovery: 0.01,
        };
        let mut f = filter(p);
        let h = f.hasher().clone();
        let mut eager = vec![1.0f64; h.n_kc()];
        for s in 0..50 {
            let c = h.hash(&x(s % 7)).unwrap();
            f.insert_code(&c).unwrap();
            let act: std::collections::HashSet<u32> = c.indices().iter().copied().collect();
            for (u, w) in eager.iter_mut().enumerate() {
                *w = if act.contains(&(u as u32)) {
                    *w * 0.5
                } else {
                    (*w + 0.01).min(1.0)
                };
            }
        }
        for (u, &e) in eager.iter().enumerate() {
            assert!((f64::from(f.weight(u)) - e).abs() < 1e-4, "unit {u}");
        }
    }

    #[test]
    fn recovery_makes_old_items_novel_again() {
        let mut f = filter(BloomParams {
            decay: 0.0,
            recovery: 0.1,
        });
        f.insert(&x(1)).unwrap();
        let fresh = f.novelty(&x(1)).unwrap();
        for s in 100..120 {
            f.insert(&x(s)).unwrap();
        }
        assert!(f.novelty(&x(1)).unwrap() > fresh);
        assert!(
            FlyBloom::new(
                f.hasher().clone(),
                BloomParams {
                    decay: 1.0,
                    recovery: 0.0
                }
            )
            .is_err()
        );
    }
}
