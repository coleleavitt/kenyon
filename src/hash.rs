//! FlyHash and DenseFly.
//!
//! **FlyHash** (Dasgupta, Stevens & Navlakha, *Science* 358:793, 2017; bioRxiv
//! 10.1101/180471): (1) centre the input, (2) expand `d -> m >> d` with a
//! sparse binary random matrix, (3) keep the top-`k` activations
//! (winner-take-all). The tag is those `k` units, with or without their values.
//!
//! **DenseFly** (Sharma & Navlakha 2018, arXiv:1812.01844, Alg. 1): the same
//! expansion, but every unit with activation `>= 0` is set to 1 (a
//! high-dimensional SimHash), plus a low-dimensional *pseudo-hash* (the sign
//! of each block's mean activation) used to pick multi-probe bins.

use crate::code::{BitCode, KcCode};
use crate::error::{Result, check_dim, invalid};
use crate::preprocess::Preprocess;
use crate::projection::{DenseProjection, Expansion, SparseProjection};
use crate::rng::Rng;
use crate::wta;

/// How the tag is selected from the `m` activations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagRule {
    /// The `k` most active units (the fly's APL inhibition).
    WinnerTakeAll(usize),
    /// A fixed set of units chosen at random once, the same for every input:
    /// the "random tag" control of Dasgupta 2017 (preprint p.8-9), which the
    /// paper notes is effectively `k` random projections.
    Fixed(Vec<u32>),
    /// Every unit (no sparsification): the dense-code control.
    All,
}

/// Reusable buffers for allocation-free hashing.
#[derive(Debug, Default, Clone)]
pub struct Scratch {
    pre: Vec<f32>,
    act: Vec<f32>,
    idx: Vec<u32>,
}

/// A FlyHash function. Immutable after construction, so `&FlyHash` can be
/// shared across threads.
#[derive(Debug, Clone, PartialEq)]
pub struct FlyHash {
    expansion: Expansion,
    tag: TagRule,
    preprocess: Preprocess,
}

/// Builder-style parameters for the standard FlyHash.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlyHashParams {
    /// Input dimension `d`.
    pub n_inputs: usize,
    /// Number of Kenyon cells `m`.
    pub n_kc: usize,
    /// Inputs sampled per KC.
    pub fan_in: usize,
    /// Winners kept (`k`, the hash length).
    pub k: usize,
    /// Pre-processing.
    pub preprocess: Preprocess,
    /// Seed for the projection.
    pub seed: u64,
}

impl FlyHashParams {
    /// The paper's defaults for input dimension `d` and hash length `k`:
    /// `m = 10 d` KCs (Fig. 3, "a 10d expansion"), each sampling 10% of the
    /// inputs (p.8), mean-centred inputs.
    pub fn paper(n_inputs: usize, k: usize, seed: u64) -> Self {
        Self {
            n_inputs,
            n_kc: 10 * n_inputs,
            fan_in: (n_inputs / 10).max(1),
            k,
            preprocess: Preprocess::CenterMean,
            seed,
        }
    }
}

impl FlyHash {
    /// Standard FlyHash: sparse binary fixed-fan-in expansion + WTA.
    pub fn new(p: FlyHashParams) -> Result<Self> {
        let proj = SparseProjection::fixed_fan_in(p.n_inputs, p.n_kc, p.fan_in, p.seed)?;
        Self::with_expansion(proj.into(), TagRule::WinnerTakeAll(p.k), p.preprocess)
    }

    /// Any expansion with any tag rule (used for controls and for measured
    /// connectomes).
    pub fn with_expansion(
        expansion: Expansion,
        tag: TagRule,
        preprocess: Preprocess,
    ) -> Result<Self> {
        let m = expansion.n_outputs();
        match &tag {
            TagRule::WinnerTakeAll(k) if *k == 0 || *k > m => {
                return Err(invalid("k", format!("must be in 1..={m}, got {k}")));
            }
            TagRule::Fixed(units) => {
                if units.is_empty() {
                    return Err(invalid("tag", "fixed tag must name at least one unit"));
                }
                if units.iter().any(|&u| u as usize >= m) {
                    return Err(invalid(
                        "tag",
                        format!("fixed tag unit out of range (m = {m})"),
                    ));
                }
            }
            _ => {}
        }
        let tag = match tag {
            TagRule::Fixed(mut u) => {
                u.sort_unstable();
                u.dedup();
                TagRule::Fixed(u)
            }
            t => t,
        };
        Ok(Self {
            expansion,
            tag,
            preprocess,
        })
    }

    /// The "random tag" control: same expansion, `k` units fixed at random.
    pub fn random_tag(
        expansion: Expansion,
        k: usize,
        preprocess: Preprocess,
        seed: u64,
    ) -> Result<Self> {
        let m = expansion.n_outputs();
        if k == 0 || k > m {
            return Err(invalid("k", format!("must be in 1..={m}, got {k}")));
        }
        let mut rng = Rng::derive(seed, 0x5441_4721); // "TAG!"
        let units = rng
            .sample_distinct(m, k)
            .into_iter()
            .map(|u| u as u32)
            .collect();
        Self::with_expansion(expansion, TagRule::Fixed(units), preprocess)
    }

    /// Classical LSH as in Dasgupta 2017's comparison: `k` dense Gaussian
    /// projections, all kept (no expansion, no WTA).
    pub fn lsh(n_inputs: usize, k: usize, preprocess: Preprocess, seed: u64) -> Result<Self> {
        let proj = DenseProjection::gaussian(n_inputs, k, seed)?;
        Self::with_expansion(proj.into(), TagRule::All, preprocess)
    }

    /// Input dimension.
    pub fn n_inputs(&self) -> usize {
        self.expansion.n_inputs()
    }

    /// Code dimension (`m`).
    pub fn n_kc(&self) -> usize {
        self.expansion.n_outputs()
    }

    /// The expansion matrix.
    pub fn expansion(&self) -> &Expansion {
        &self.expansion
    }

    /// The tag rule.
    pub fn tag_rule(&self) -> &TagRule {
        &self.tag
    }

    /// Raw KC activations `M x` after pre-processing.
    pub fn activations(&self, x: &[f32], s: &mut Scratch) -> Result<Vec<f32>> {
        self.activate(x, s)?;
        Ok(s.act.clone())
    }

    fn activate(&self, x: &[f32], s: &mut Scratch) -> Result<()> {
        check_dim(self.n_inputs(), x.len())?;
        self.preprocess.apply_into(x, &mut s.pre);
        s.act.resize(self.n_kc(), 0.0);
        self.expansion.project_into(&s.pre, &mut s.act)
    }

    /// Hash `x`, reusing `s`.
    pub fn hash_with(&self, x: &[f32], s: &mut Scratch) -> Result<KcCode> {
        self.activate(x, s)?;
        let idx = match &self.tag {
            TagRule::WinnerTakeAll(k) => wta::top_k(&s.act, *k, &mut s.idx),
            TagRule::Fixed(u) => u.clone(),
            TagRule::All => (0..self.n_kc() as u32).collect(),
        };
        let vals = idx.iter().map(|&i| s.act[i as usize]).collect();
        Ok(KcCode::from_sorted(self.n_kc() as u32, idx, vals))
    }

    /// Hash `x`.
    pub fn hash(&self, x: &[f32]) -> Result<KcCode> {
        self.hash_with(x, &mut Scratch::default())
    }

    /// Hash each `n_inputs`-long row of `rows` (row-major).
    pub fn hash_batch(&self, rows: &[f32]) -> Result<Vec<KcCode>> {
        let d = self.n_inputs();
        if rows.len() % d != 0 {
            return Err(crate::KenyonError::DimensionMismatch {
                expected: d,
                got: rows.len() % d,
            });
        }
        let mut s = Scratch::default();
        rows.chunks_exact(d)
            .map(|r| self.hash_with(r, &mut s))
            .collect()
    }

    /// Heap bytes held (the projection matrix and tag).
    pub fn memory_bytes(&self) -> usize {
        self.expansion.memory_bytes()
            + match &self.tag {
                TagRule::Fixed(u) => u.capacity() * 4,
                _ => 0,
            }
    }
}

/// DenseFly (Sharma & Navlakha 2018, Alg. 1).
#[derive(Debug, Clone, PartialEq)]
pub struct DenseFly {
    expansion: Expansion,
    preprocess: Preprocess,
    block: usize,
}

impl DenseFly {
    /// `n_kc = m*k` sparse projections sampling `fan_in` inputs each; the
    /// pseudo-hash has `n_kc / block` bits (`block` is the paper's WTA factor
    /// `k`, 20 for the fly).
    pub fn new(
        n_inputs: usize,
        n_kc: usize,
        fan_in: usize,
        block: usize,
        preprocess: Preprocess,
        seed: u64,
    ) -> Result<Self> {
        if block == 0 || n_kc % block != 0 {
            return Err(invalid(
                "block",
                format!("must divide n_kc = {n_kc}, got {block}"),
            ));
        }
        let proj = SparseProjection::fixed_fan_in(n_inputs, n_kc, fan_in, seed)?;
        Ok(Self {
            expansion: proj.into(),
            preprocess,
            block,
        })
    }

    /// From any expansion.
    pub fn with_expansion(
        expansion: Expansion,
        block: usize,
        preprocess: Preprocess,
    ) -> Result<Self> {
        let m = expansion.n_outputs();
        if block == 0 || m % block != 0 {
            return Err(invalid(
                "block",
                format!("must divide n_kc = {m}, got {block}"),
            ));
        }
        Ok(Self {
            expansion,
            preprocess,
            block,
        })
    }

    /// Input dimension.
    pub fn n_inputs(&self) -> usize {
        self.expansion.n_inputs()
    }

    /// High-dimensional hash length.
    pub fn n_kc(&self) -> usize {
        self.expansion.n_outputs()
    }

    /// Pseudo-hash length (`n_kc / block`).
    pub fn pseudo_len(&self) -> usize {
        self.n_kc() / self.block
    }

    /// Both hashes: `(h1, h2)` = (high-dimensional sign code, pseudo-hash).
    pub fn hash_with(&self, x: &[f32], s: &mut Scratch) -> Result<(BitCode, BitCode)> {
        check_dim(self.n_inputs(), x.len())?;
        self.preprocess.apply_into(x, &mut s.pre);
        s.act.resize(self.n_kc(), 0.0);
        self.expansion.project_into(&s.pre, &mut s.act)?;
        let mut h1 = BitCode::zeros(self.n_kc());
        for (i, &a) in s.act.iter().enumerate() {
            if a >= 0.0 {
                h1.set(i);
            }
        }
        let mut h2 = BitCode::zeros(self.pseudo_len());
        for (j, blk) in s.act.chunks_exact(self.block).enumerate() {
            if blk.iter().sum::<f32>() >= 0.0 {
                h2.set(j);
            }
        }
        Ok((h1, h2))
    }

    /// Both hashes of `x`.
    pub fn hash(&self, x: &[f32]) -> Result<(BitCode, BitCode)> {
        self.hash_with(x, &mut Scratch::default())
    }

    /// Heap bytes held.
    pub fn memory_bytes(&self) -> usize {
        self.expansion.memory_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vecs(seed: u64, n: usize, d: usize) -> Vec<Vec<f32>> {
        let mut r = Rng::new(seed);
        (0..n)
            .map(|_| (0..d).map(|_| r.next_f64() as f32).collect())
            .collect()
    }

    #[test]
    fn deterministic_and_k_sparse() {
        let h = FlyHash::new(FlyHashParams::paper(50, 8, 1)).unwrap();
        let x = &vecs(2, 1, 50)[0];
        let a = h.hash(x).unwrap();
        assert_eq!(a.len(), 8);
        assert_eq!(
            a,
            FlyHash::new(FlyHashParams::paper(50, 8, 1))
                .unwrap()
                .hash(x)
                .unwrap()
        );
        assert!(h.hash(&x[..49]).is_err());
    }

    #[test]
    fn similar_inputs_share_more_units_than_dissimilar() {
        let h = FlyHash::new(FlyHashParams {
            n_inputs: 50,
            n_kc: 2000,
            fan_in: 6,
            k: 100,
            preprocess: Preprocess::CenterMean,
            seed: 9,
        })
        .unwrap();
        let xs = vecs(3, 2, 50);
        let mut near = xs[0].clone();
        near[0] += 0.05;
        near[1] -= 0.05;
        let base = h.hash(&xs[0]).unwrap();
        let o_near = base.overlap(&h.hash(&near).unwrap()).unwrap();
        let o_far = base.overlap(&h.hash(&xs[1]).unwrap()).unwrap();
        assert!(o_near > o_far, "{o_near} vs {o_far}");
    }

    #[test]
    fn controls_have_the_expected_shape() {
        let lsh = FlyHash::lsh(20, 4, Preprocess::CenterMean, 1).unwrap();
        assert_eq!(lsh.hash(&[1.0; 20]).unwrap().len(), 4);
        let exp: Expansion = SparseProjection::fixed_fan_in(20, 200, 2, 1)
            .unwrap()
            .into();
        let rt = FlyHash::random_tag(exp, 4, Preprocess::CenterMean, 5).unwrap();
        let xs = vecs(1, 2, 20);
        assert_eq!(
            rt.hash(&xs[0]).unwrap().indices(),
            rt.hash(&xs[1]).unwrap().indices()
        );
        let batch = rt.hash_batch(&xs.concat()).unwrap();
        assert_eq!(batch.len(), 2);
    }

    #[test]
    fn densefly_shapes() {
        let df = DenseFly::new(30, 160, 3, 20, Preprocess::CenterMean, 4).unwrap();
        let (h1, h2) = df.hash(&vecs(5, 1, 30)[0]).unwrap();
        assert_eq!((h1.len(), h2.len()), (160, 8));
        assert!(DenseFly::new(30, 161, 3, 20, Preprocess::CenterMean, 4).is_err());
    }
}
