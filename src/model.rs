//! FlyModel: continual learning by sparse coding + partial freezing.
//!
//! Shen, Dasgupta & Navlakha, "Algorithmic insights on continual learning from
//! fruit flies" (arXiv:2107.07617, 2021; *Neural Computation* 2023).
//!
//! - Layer 1 (PN -> KC, fixed): `psi = Theta x` with a sparse binary `Theta`
//!   (each KC sums `fan_in` random inputs; m ~ 40 d), then winner-take-all
//!   keeps the `l` largest positive entries, then min-max normalisation to
//!   [0, 1] (Eqs. 1-2, p.4).
//! - Layer 2 (KC -> MBON, learned): when `x` is labelled `j`, only the
//!   synapses from the active KCs to MBON `j` change:
//!   `w_ij <- (1 - alpha) w_ij + beta phi_i`, others decay by `(1 - alpha)`;
//!   weights are capped to [0, 1] (Eq. 3, p.5). "Partial freezing".
//! - Predict `argmax_j sum_i w_ij phi_i`.
//!
//! The input `x` is expected to come from a **frozen encoder** (the paper uses
//! a LeNet5 trained on KMNIST for MNIST-20, p.11): the FlyModel learns
//! associations, not representations (p.4, p.11-12).
//!
//! The ablations of the paper are built in: [`Coding::Dense`] (no WTA, p.7)
//! and the perceptron rules v1-v3 (p.8, p.13).

use crate::code::KcCode;
use crate::error::{KenyonError, Result, check_dim, invalid};
use crate::projection::SparseProjection;
use crate::wta;

/// Layer-1 coding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coding {
    /// Keep the `l` most active positive KCs (the fly).
    Sparse,
    /// Keep every KC (the paper's dense-coding ablation).
    Dense,
}

/// Layer-2 learning rule (paper p.13 pseudocode).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LearningRule {
    /// Perceptron v4 = FlyModel: always strengthen the target, never weaken.
    Associative,
    /// v1, the classic perceptron: on a mistake, `+target`, `-predicted`.
    PerceptronV1,
    /// v2: on a mistake, `+target` only.
    PerceptronV2,
    /// v3: always `+target`; on a mistake also `-predicted`.
    PerceptronV3,
}

/// FlyModel hyper-parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlyModelParams {
    /// Input dimension `d` (encoder feature size).
    pub n_inputs: usize,
    /// Kenyon cells `m`.
    pub n_kc: usize,
    /// Inputs per KC.
    pub fan_in: usize,
    /// Active KCs `l` after WTA.
    pub n_active: usize,
    /// Output classes (MBONs).
    pub n_classes: usize,
    /// Learning rate `beta`.
    pub learning_rate: f32,
    /// Forgetting `alpha` (0 in the paper's experiments).
    pub forgetting: f32,
    /// Coding.
    pub coding: Coding,
    /// Learning rule.
    pub rule: LearningRule,
    /// Projection seed.
    pub seed: u64,
}

impl FlyModelParams {
    /// The paper's MNIST-20 setting (p.12): d = 84, m = 3200, 10 inputs per
    /// KC, l = m / 20 = 160, 20 classes, beta = 0.01, alpha = 0.
    pub fn mnist20(seed: u64) -> Self {
        Self {
            n_inputs: 84,
            n_kc: 3200,
            fan_in: 10,
            n_active: 160,
            n_classes: 20,
            learning_rate: 0.01,
            forgetting: 0.0,
            coding: Coding::Sparse,
            rule: LearningRule::Associative,
            seed,
        }
    }
}

/// The FlyModel continual learner.
#[derive(Debug, Clone)]
pub struct FlyModel {
    p: FlyModelParams,
    theta: SparseProjection,
    /// `n_classes x n_kc`, row-major.
    w: Vec<f32>,
    act: Vec<f32>,
    idx: Vec<u32>,
}

impl FlyModel {
    /// New model with zero associative weights.
    pub fn new(p: FlyModelParams) -> Result<Self> {
        if p.n_classes == 0 {
            return Err(invalid("n_classes", "must be > 0"));
        }
        if p.n_active == 0 || p.n_active > p.n_kc {
            return Err(invalid("n_active", format!("must be in 1..={}", p.n_kc)));
        }
        if !(p.learning_rate > 0.0 && p.learning_rate.is_finite()) {
            return Err(invalid("learning_rate", "must be positive"));
        }
        if !(0.0..1.0).contains(&p.forgetting) {
            return Err(invalid("forgetting", "must be in [0, 1)"));
        }
        let theta = SparseProjection::fixed_fan_in(p.n_inputs, p.n_kc, p.fan_in, p.seed)?;
        Ok(Self {
            p,
            theta,
            w: vec![0.0; p.n_classes * p.n_kc],
            act: Vec::new(),
            idx: Vec::new(),
        })
    }

    /// Parameters.
    pub fn params(&self) -> &FlyModelParams {
        &self.p
    }

    /// The layer-1 code `phi(x)` (min-max normalised to [0, 1]).
    pub fn encode(&mut self, x: &[f32]) -> Result<KcCode> {
        check_dim(self.p.n_inputs, x.len())?;
        self.act.resize(self.p.n_kc, 0.0);
        self.theta.project_into(x, &mut self.act)?;
        let (idx, raw): (Vec<u32>, Vec<f32>) = match self.p.coding {
            Coding::Sparse => {
                let idx = wta::top_k_positive(&self.act, self.p.n_active, &mut self.idx);
                let vals = idx.iter().map(|&i| self.act[i as usize]).collect();
                (idx, vals)
            }
            Coding::Dense => ((0..self.p.n_kc as u32).collect(), self.act.clone()),
        };
        // Min-max over the whole m-vector: silent KCs are 0, so for the
        // sparse code min = 0 and this divides by the largest winner.
        let silent_zero = idx.len() < self.p.n_kc;
        let mut lo = raw.iter().copied().fold(f32::INFINITY, f32::min);
        let hi = raw.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        if silent_zero {
            lo = lo.min(0.0);
        }
        let span = hi - lo;
        let vals = raw
            .iter()
            .map(|&v| if span > 0.0 { (v - lo) / span } else { 0.0 })
            .collect();
        KcCode::new(self.p.n_kc, idx, vals)
    }

    fn class_scores(&self, code: &KcCode) -> Vec<f64> {
        let m = self.p.n_kc;
        (0..self.p.n_classes)
            .map(|j| {
                let row = &self.w[j * m..(j + 1) * m];
                code.indices()
                    .iter()
                    .zip(code.values())
                    .map(|(&i, &v)| f64::from(row[i as usize]) * f64::from(v))
                    .sum()
            })
            .collect()
    }

    fn argmax(scores: &[f64]) -> usize {
        let mut best = 0;
        for (j, &s) in scores.iter().enumerate() {
            if s > scores[best] {
                best = j;
            }
        }
        best
    }

    /// MBON outputs for `x`.
    pub fn scores(&mut self, x: &[f32]) -> Result<Vec<f64>> {
        let code = self.encode(x)?;
        Ok(self.class_scores(&code))
    }

    /// Predicted class (ties -> lowest class index).
    pub fn predict(&mut self, x: &[f32]) -> Result<usize> {
        Ok(Self::argmax(&self.scores(x)?))
    }

    fn update(&mut self, j: usize, code: &KcCode, sign: f32) {
        let m = self.p.n_kc;
        let beta = self.p.learning_rate * sign;
        let row = &mut self.w[j * m..(j + 1) * m];
        for (&i, &v) in code.indices().iter().zip(code.values()) {
            let w = &mut row[i as usize];
            *w = (*w + beta * v).clamp(0.0, 1.0);
        }
    }

    /// Learn one labelled example (one online step).
    pub fn train_one(&mut self, x: &[f32], class: usize) -> Result<()> {
        if class >= self.p.n_classes {
            return Err(KenyonError::ClassOutOfRange {
                class,
                n_classes: self.p.n_classes,
            });
        }
        let code = self.encode(x)?;
        if self.p.forgetting > 0.0 {
            let keep = 1.0 - self.p.forgetting;
            self.w.iter_mut().for_each(|w| *w *= keep);
        }
        let predicted = match self.p.rule {
            LearningRule::Associative => None,
            _ => Some(Self::argmax(&self.class_scores(&code))),
        };
        let wrong = predicted.filter(|&p| p != class);
        match self.p.rule {
            LearningRule::Associative => self.update(class, &code, 1.0),
            LearningRule::PerceptronV1 => {
                if let Some(p) = wrong {
                    self.update(class, &code, 1.0);
                    self.update(p, &code, -1.0);
                }
            }
            LearningRule::PerceptronV2 => {
                if wrong.is_some() {
                    self.update(class, &code, 1.0);
                }
            }
            LearningRule::PerceptronV3 => {
                self.update(class, &code, 1.0);
                if let Some(p) = wrong {
                    self.update(p, &code, -1.0);
                }
            }
        }
        Ok(())
    }

    /// The associative weights of class `j` (`n_kc` values).
    pub fn class_weights(&self, j: usize) -> Option<&[f32]> {
        let m = self.p.n_kc;
        (j < self.p.n_classes).then(|| &self.w[j * m..(j + 1) * m])
    }

    /// Fraction of associative weights at the cap (saturation, paper p.5).
    pub fn saturation(&self) -> f64 {
        self.w.iter().filter(|&&w| w >= 1.0).count() as f64 / self.w.len() as f64
    }

    /// Heap bytes held (projection + weights).
    pub fn memory_bytes(&self) -> usize {
        self.theta.memory_bytes()
            + self.w.capacity() * 4
            + self.act.capacity() * 4
            + self.idx.capacity() * 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rng;

    fn params(coding: Coding, rule: LearningRule) -> FlyModelParams {
        FlyModelParams {
            n_inputs: 20,
            n_kc: 800,
            fan_in: 4,
            n_active: 40,
            n_classes: 4,
            learning_rate: 0.05,
            forgetting: 0.0,
            coding,
            rule,
            seed: 3,
        }
    }

    /// Four noisy prototypes learned one class at a time.
    fn run(coding: Coding, rule: LearningRule) -> f64 {
        let mut r = Rng::new(9);
        let protos: Vec<Vec<f32>> = (0..4)
            .map(|_| (0..20).map(|_| r.next_f64() as f32).collect())
            .collect();
        let noisy = |c: usize, r: &mut Rng| -> Vec<f32> {
            protos[c]
                .iter()
                .map(|&v| (v + 0.15 * r.normal() as f32).max(0.0))
                .collect()
        };
        let mut m = FlyModel::new(params(coding, rule)).unwrap();
        for c in 0..4 {
            for _ in 0..50 {
                let x = noisy(c, &mut r);
                m.train_one(&x, c).unwrap();
            }
        }
        let mut ok = 0;
        for i in 0..400 {
            let c = i % 4;
            let x = noisy(c, &mut r);
            if m.predict(&x).unwrap() == c {
                ok += 1;
            }
        }
        f64::from(ok) / 400.0
    }

    #[test]
    fn sparse_associative_learns_sequential_classes() {
        let acc = run(Coding::Sparse, LearningRule::Associative);
        assert!(acc > 0.9, "{acc}");
    }

    #[test]
    fn every_rule_and_coding_runs() {
        for coding in [Coding::Sparse, Coding::Dense] {
            for rule in [
                LearningRule::Associative,
                LearningRule::PerceptronV1,
                LearningRule::PerceptronV2,
                LearningRule::PerceptronV3,
            ] {
                let acc = run(coding, rule);
                assert!((0.0..=1.0).contains(&acc));
            }
        }
    }

    #[test]
    fn code_is_sparse_and_normalised() {
        let mut m = FlyModel::new(params(Coding::Sparse, LearningRule::Associative)).unwrap();
        let c = m.encode(&[0.5; 20]).unwrap();
        assert!(c.len() <= 40);
        assert!(c.values().iter().all(|v| (0.0..=1.0).contains(v)));
        assert!(m.train_one(&[0.5; 20], 4).is_err());
        let mut d = FlyModel::new(params(Coding::Dense, LearningRule::Associative)).unwrap();
        assert_eq!(d.encode(&[0.5; 20]).unwrap().len(), 800);
    }
}
