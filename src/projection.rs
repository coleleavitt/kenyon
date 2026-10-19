//! The projection-neuron (PN) -> Kenyon-cell (KC) expansion.
//!
//! The fly's PN->KC map is modelled as a sparse, binary random matrix: each KC
//! sums a handful of PN inputs (about 6 of 50 glomeruli; Caron et al. 2013,
//! cited as ref. 14 in Dasgupta, Stevens & Navlakha 2017, preprint p.6).
//! [`SparseProjection`] stores it as compressed rows (one row per KC), with
//! optional per-edge weights so a measured connectome (synapse counts) can be
//! used directly ([`SparseProjection::from_edges`]).
//! [`DenseProjection`] is the dense Gaussian matrix of classical LSH, kept as
//! a control (Dasgupta 2017 Fig. 2A).

use crate::error::{KenyonError, Result, check_dim, invalid};
use crate::rng::Rng;

/// Input indices stored at the narrowest width that holds `n_inputs`
/// (1, 2 or 4 bytes per edge). Chosen deterministically from the shape, so
/// the same matrix always has the same representation.
#[derive(Debug, Clone, PartialEq)]
enum Indices {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
}

/// An index type the matrix can be stored in.
trait Ix: Copy {
    fn ix(self) -> usize;
}
impl Ix for u8 {
    fn ix(self) -> usize {
        usize::from(self)
    }
}
impl Ix for u16 {
    fn ix(self) -> usize {
        usize::from(self)
    }
}
impl Ix for u32 {
    fn ix(self) -> usize {
        self as usize
    }
}

impl Indices {
    /// Narrow `v` (every entry `< n_inputs`).
    fn compact(v: Vec<u32>, n_inputs: usize) -> Self {
        if n_inputs <= 1 << 8 {
            Self::U8(v.into_iter().map(|i| i as u8).collect())
        } else if n_inputs <= 1 << 16 {
            Self::U16(v.into_iter().map(|i| i as u16).collect())
        } else {
            Self::U32(v)
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::U8(v) => v.len(),
            Self::U16(v) => v.len(),
            Self::U32(v) => v.len(),
        }
    }

    fn get(&self, e: usize) -> usize {
        match self {
            Self::U8(v) => v[e].ix(),
            Self::U16(v) => v[e].ix(),
            Self::U32(v) => v[e].ix(),
        }
    }

    fn bytes(&self) -> usize {
        match self {
            Self::U8(v) => v.capacity(),
            Self::U16(v) => v.capacity() * 2,
            Self::U32(v) => v.capacity() * 4,
        }
    }
}

/// Row boundaries: a fixed stride when every output has the same fan-in (the
/// standard FlyHash), else explicit offsets (Bernoulli rows, measured wiring).
#[derive(Debug, Clone, PartialEq)]
enum Rows {
    Stride(usize),
    Offsets(Vec<u32>),
}

impl Rows {
    /// The fixed stride if `offsets` is uniform, else `offsets` itself.
    fn compact(offsets: Vec<u32>) -> Self {
        let mut widths = offsets.windows(2).map(|w| w[1] - w[0]);
        match widths.next() {
            Some(first) if widths.all(|w| w == first) => Self::Stride(first as usize),
            None => Self::Stride(0),
            _ => Self::Offsets(offsets),
        }
    }

    fn range(&self, j: usize) -> (usize, usize) {
        match self {
            Self::Stride(f) => (j * f, (j + 1) * f),
            Self::Offsets(o) => (o[j] as usize, o[j + 1] as usize),
        }
    }

    fn bytes(&self) -> usize {
        match self {
            Self::Stride(_) => 0,
            Self::Offsets(o) => o.capacity() * 4,
        }
    }
}

fn row_sum<T: Ix>(idx: &[T], x: &[f32]) -> f32 {
    idx.iter().map(|&i| x[i.ix()]).sum()
}

fn row_dot<T: Ix>(idx: &[T], w: &[f32], x: &[f32]) -> f32 {
    idx.iter().zip(w).map(|(&i, &wt)| x[i.ix()] * wt).sum()
}

/// A sparse `n_outputs x n_inputs` matrix in compressed-row form.
///
/// Storage is compact: input indices use 1, 2 or 4 bytes each depending on
/// `n_inputs`, and a uniform fan-in needs no offsets table. A standard
/// FlyHash over 128 inputs costs one byte per edge.
#[derive(Debug, Clone, PartialEq)]
pub struct SparseProjection {
    n_inputs: usize,
    n_outputs: usize,
    rows: Rows,
    indices: Indices,
    /// `None` = every edge has weight 1 (the binary fly model).
    weights: Option<Vec<f32>>,
}

impl SparseProjection {
    fn build(
        n_inputs: usize,
        n_outputs: usize,
        offsets: Vec<u32>,
        indices: Vec<u32>,
        weights: Option<Vec<f32>>,
    ) -> Self {
        Self {
            n_inputs,
            n_outputs,
            rows: Rows::compact(offsets),
            indices: Indices::compact(indices, n_inputs),
            weights,
        }
    }

    /// Every output samples exactly `fan_in` distinct inputs, uniformly at
    /// random. With `fan_in = 0.1 * d` this is the FlyHash setting used by
    /// Dasgupta 2017 (preprint p.8, "sampling 10%") and DenseFly (Sharma &
    /// Navlakha 2018, Alg. 1, `alpha = 0.1`).
    pub fn fixed_fan_in(
        n_inputs: usize,
        n_outputs: usize,
        fan_in: usize,
        seed: u64,
    ) -> Result<Self> {
        check_shape(n_inputs, n_outputs)?;
        if fan_in == 0 || fan_in > n_inputs {
            return Err(invalid(
                "fan_in",
                format!("must be in 1..={n_inputs}, got {fan_in}"),
            ));
        }
        let mut rng = Rng::derive(seed, 0x5052_4f4a); // "PROJ"
        let mut offsets = Vec::with_capacity(n_outputs + 1);
        let mut indices = Vec::with_capacity(n_outputs * fan_in);
        offsets.push(0u32);
        for _ in 0..n_outputs {
            let mut row: Vec<u32> = rng
                .sample_distinct(n_inputs, fan_in)
                .into_iter()
                .map(|v| v as u32)
                .collect();
            row.sort_unstable();
            indices.extend_from_slice(&row);
            offsets.push(to_u32(indices.len())?);
        }
        Ok(Self::build(n_inputs, n_outputs, offsets, indices, None))
    }

    /// Each entry is 1 independently with probability `p` (the model stated in
    /// Dasgupta 2017, preprint p.6: "each entry M_ij is set independently with
    /// probability p"). Rows may be empty.
    pub fn bernoulli(n_inputs: usize, n_outputs: usize, p: f64, seed: u64) -> Result<Self> {
        check_shape(n_inputs, n_outputs)?;
        if !(p > 0.0 && p <= 1.0) {
            return Err(invalid("p", format!("must be in (0, 1], got {p}")));
        }
        let mut rng = Rng::derive(seed, 0x4245_524e); // "BERN"
        let mut offsets = Vec::with_capacity(n_outputs + 1);
        let mut indices = Vec::new();
        offsets.push(0u32);
        for _ in 0..n_outputs {
            for i in 0..n_inputs {
                if rng.next_f64() < p {
                    indices.push(i as u32);
                }
            }
            offsets.push(to_u32(indices.len())?);
        }
        Ok(Self::build(n_inputs, n_outputs, offsets, indices, None))
    }

    /// Build from explicit `(input, output, weight)` edges, e.g. a connectome's
    /// PN->KC synapse counts. Duplicate edges are merged by summing weights.
    /// If every weight is exactly 1 the matrix is stored as binary.
    pub fn from_edges(
        n_inputs: usize,
        n_outputs: usize,
        edges: &[(usize, usize, f32)],
    ) -> Result<Self> {
        check_shape(n_inputs, n_outputs)?;
        let mut sorted: Vec<(usize, usize, f32)> = Vec::with_capacity(edges.len());
        for &(i, o, w) in edges {
            if i >= n_inputs || o >= n_outputs {
                return Err(KenyonError::EdgeOutOfRange {
                    input: i,
                    output: o,
                    n_inputs,
                    n_outputs,
                });
            }
            if !w.is_finite() {
                return Err(invalid(
                    "weight",
                    format!("edge ({i} -> {o}) has non-finite weight"),
                ));
            }
            sorted.push((o, i, w));
        }
        sorted.sort_by_key(|e| (e.0, e.1));
        let mut offsets = Vec::with_capacity(n_outputs + 1);
        let mut indices: Vec<u32> = Vec::with_capacity(sorted.len());
        let mut weights: Vec<f32> = Vec::with_capacity(sorted.len());
        offsets.push(0u32);
        let mut e = 0;
        for o in 0..n_outputs {
            while e < sorted.len() && sorted[e].0 == o {
                let (_, i, w) = sorted[e];
                if indices.len() > offsets[o] as usize && indices.last() == Some(&(i as u32)) {
                    if let Some(last) = weights.last_mut() {
                        *last += w;
                    }
                } else {
                    indices.push(i as u32);
                    weights.push(w);
                }
                e += 1;
            }
            offsets.push(to_u32(indices.len())?);
        }
        let binary = weights.iter().all(|&w| w == 1.0);
        Ok(Self::build(
            n_inputs,
            n_outputs,
            offsets,
            indices,
            if binary { None } else { Some(weights) },
        ))
    }

    /// Number of inputs (PNs, `d`).
    pub fn n_inputs(&self) -> usize {
        self.n_inputs
    }

    /// Number of outputs (KCs, `m`).
    pub fn n_outputs(&self) -> usize {
        self.n_outputs
    }

    /// Number of non-zero entries (claws).
    pub fn n_edges(&self) -> usize {
        self.indices.len()
    }

    /// `true` if every edge has weight 1.
    pub fn is_binary(&self) -> bool {
        self.weights.is_none()
    }

    fn range(&self, j: usize) -> Option<(usize, usize)> {
        (j < self.n_outputs).then(|| self.rows.range(j))
    }

    /// The inputs of output `j`, sorted ascending (empty if `j` is out of
    /// range).
    pub fn row(&self, j: usize) -> Vec<u32> {
        match self.range(j) {
            Some((a, b)) => (a..b).map(|e| self.indices.get(e) as u32).collect(),
            None => Vec::new(),
        }
    }

    /// The edge weights of output `j` (all 1 for a binary matrix).
    pub fn row_weights(&self, j: usize) -> Vec<f32> {
        match (self.range(j), &self.weights) {
            (Some((a, b)), Some(w)) => w[a..b].to_vec(),
            (Some((a, b)), None) => vec![1.0; b - a],
            (None, _) => Vec::new(),
        }
    }

    /// All edges as `(input, output, weight)`.
    pub fn edges(&self) -> Vec<(usize, usize, f32)> {
        let mut out = Vec::with_capacity(self.indices.len());
        for j in 0..self.n_outputs {
            let (a, b) = self.rows.range(j);
            for e in a..b {
                let w = self.weights.as_ref().map_or(1.0, |w| w[e]);
                out.push((self.indices.get(e), j, w));
            }
        }
        out
    }

    /// In-degree (fan-in) of every output.
    pub fn fan_in(&self) -> Vec<usize> {
        (0..self.n_outputs)
            .map(|j| {
                let (a, b) = self.rows.range(j);
                b - a
            })
            .collect()
    }

    /// Out-degree of every input (how many outputs sample it).
    pub fn out_degree(&self) -> Vec<usize> {
        let mut deg = vec![0usize; self.n_inputs];
        for e in 0..self.indices.len() {
            deg[self.indices.get(e)] += 1;
        }
        deg
    }

    /// The same topology with every weight set to 1.
    pub fn binarized(&self) -> Self {
        Self {
            weights: None,
            ..self.clone()
        }
    }

    /// `out = M x`. `out.len()` must equal `n_outputs`.
    pub fn project_into(&self, x: &[f32], out: &mut [f32]) -> Result<()> {
        check_dim(self.n_inputs, x.len())?;
        check_dim(self.n_outputs, out.len())?;
        for (j, o) in out.iter_mut().enumerate() {
            let (a, b) = self.rows.range(j);
            *o = match (&self.indices, &self.weights) {
                (Indices::U8(v), None) => row_sum(&v[a..b], x),
                (Indices::U16(v), None) => row_sum(&v[a..b], x),
                (Indices::U32(v), None) => row_sum(&v[a..b], x),
                (Indices::U8(v), Some(w)) => row_dot(&v[a..b], &w[a..b], x),
                (Indices::U16(v), Some(w)) => row_dot(&v[a..b], &w[a..b], x),
                (Indices::U32(v), Some(w)) => row_dot(&v[a..b], &w[a..b], x),
            };
        }
        Ok(())
    }

    /// Heap bytes held by the matrix.
    pub fn memory_bytes(&self) -> usize {
        self.rows.bytes()
            + self.indices.bytes()
            + self.weights.as_ref().map_or(0, |w| w.capacity() * 4)
    }
}

/// A dense `n_outputs x n_inputs` matrix with i.i.d. N(0, 1) entries
/// (classical LSH / SimHash projections; Charikar 2002).
#[derive(Debug, Clone, PartialEq)]
pub struct DenseProjection {
    n_inputs: usize,
    n_outputs: usize,
    w: Vec<f32>,
}

impl DenseProjection {
    /// Gaussian random matrix from `seed`.
    pub fn gaussian(n_inputs: usize, n_outputs: usize, seed: u64) -> Result<Self> {
        check_shape(n_inputs, n_outputs)?;
        let mut rng = Rng::derive(seed, 0x4741_5553); // "GAUS"
        let w = (0..n_inputs * n_outputs)
            .map(|_| rng.normal() as f32)
            .collect();
        Ok(Self {
            n_inputs,
            n_outputs,
            w,
        })
    }

    /// Number of inputs.
    pub fn n_inputs(&self) -> usize {
        self.n_inputs
    }

    /// Number of outputs.
    pub fn n_outputs(&self) -> usize {
        self.n_outputs
    }

    /// `out = W x`.
    pub fn project_into(&self, x: &[f32], out: &mut [f32]) -> Result<()> {
        check_dim(self.n_inputs, x.len())?;
        check_dim(self.n_outputs, out.len())?;
        for (row, o) in self.w.chunks_exact(self.n_inputs).zip(out.iter_mut()) {
            *o = row.iter().zip(x).map(|(a, b)| a * b).sum();
        }
        Ok(())
    }

    /// Heap bytes held by the matrix.
    pub fn memory_bytes(&self) -> usize {
        self.w.capacity() * 4
    }
}

/// Either kind of expansion matrix.
#[derive(Debug, Clone, PartialEq)]
pub enum Expansion {
    /// Sparse (binary or weighted) matrix: the fly.
    Sparse(SparseProjection),
    /// Dense Gaussian matrix: the LSH control.
    Dense(DenseProjection),
}

impl Expansion {
    /// Number of inputs.
    pub fn n_inputs(&self) -> usize {
        match self {
            Self::Sparse(p) => p.n_inputs(),
            Self::Dense(p) => p.n_inputs(),
        }
    }

    /// Number of outputs.
    pub fn n_outputs(&self) -> usize {
        match self {
            Self::Sparse(p) => p.n_outputs(),
            Self::Dense(p) => p.n_outputs(),
        }
    }

    /// `out = M x`.
    pub fn project_into(&self, x: &[f32], out: &mut [f32]) -> Result<()> {
        match self {
            Self::Sparse(p) => p.project_into(x, out),
            Self::Dense(p) => p.project_into(x, out),
        }
    }

    /// Heap bytes held by the matrix.
    pub fn memory_bytes(&self) -> usize {
        match self {
            Self::Sparse(p) => p.memory_bytes(),
            Self::Dense(p) => p.memory_bytes(),
        }
    }
}

impl From<SparseProjection> for Expansion {
    fn from(p: SparseProjection) -> Self {
        Self::Sparse(p)
    }
}

impl From<DenseProjection> for Expansion {
    fn from(p: DenseProjection) -> Self {
        Self::Dense(p)
    }
}

fn check_shape(n_inputs: usize, n_outputs: usize) -> Result<()> {
    if n_inputs == 0 || n_outputs == 0 {
        return Err(invalid(
            "shape",
            format!("{n_outputs} x {n_inputs} must be non-empty"),
        ));
    }
    if n_inputs > u32::MAX as usize || n_outputs > u32::MAX as usize {
        return Err(invalid("shape", "dimensions must fit in u32"));
    }
    Ok(())
}

fn to_u32(v: usize) -> Result<u32> {
    u32::try_from(v).map_err(|_| invalid("n_edges", "more than u32::MAX edges"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compact storage: index width follows `n_inputs`, a uniform fan-in
    /// needs no offsets, and the compact matrix projects exactly like the
    /// same edges built explicitly (variable rows, u32 path).
    #[test]
    fn storage_is_compact_and_projects_identically() {
        let sift = SparseProjection::fixed_fan_in(128, 150_000, 6, 1).unwrap();
        assert!(matches!(sift.indices, Indices::U8(_)));
        assert!(matches!(sift.rows, Rows::Stride(6)));
        assert_eq!(
            sift.memory_bytes(),
            150_000 * 6,
            "one byte per edge, no offsets"
        );
        let mnist = SparseProjection::fixed_fan_in(784, 7840, 78, 1).unwrap();
        assert!(matches!(mnist.indices, Indices::U16(_)));
        let wide = SparseProjection::fixed_fan_in(70_000, 10, 3, 1).unwrap();
        assert!(matches!(wide.indices, Indices::U32(_)));
        assert!(matches!(
            SparseProjection::bernoulli(32, 50, 0.2, 3).unwrap().rows,
            Rows::Offsets(_)
        ));
        for p in [&sift, &mnist] {
            // Same edges plus one extra edge on output 0: variable rows force
            // the offsets path; every other output must project identically.
            let mut edges = p.edges();
            let extra = (0..p.n_inputs())
                .find(|i| !p.row(0).contains(&(*i as u32)))
                .unwrap();
            edges.push((extra, 0, 1.0));
            let q = SparseProjection::from_edges(p.n_inputs(), p.n_outputs(), &edges).unwrap();
            assert!(matches!(q.rows, Rows::Offsets(_)));
            let mut r = Rng::new(9);
            let x: Vec<f32> = (0..p.n_inputs()).map(|_| r.next_f64() as f32).collect();
            let (mut a, mut b) = (vec![0.0; p.n_outputs()], vec![0.0; p.n_outputs()]);
            p.project_into(&x, &mut a).unwrap();
            q.project_into(&x, &mut b).unwrap();
            assert_eq!(a[1..], b[1..]);
            // Output 0 gained one edge mid-row, so its sum runs in another
            // order: equal up to float rounding.
            assert!(
                (b[0] - (a[0] + x[extra])).abs() < 1e-5,
                "{} vs {}",
                b[0],
                a[0] + x[extra]
            );
        }
    }

    #[test]
    fn fixed_fan_in_has_exact_fan_in_and_is_seeded() {
        let p = SparseProjection::fixed_fan_in(50, 2000, 6, 42).unwrap();
        assert!(p.fan_in().iter().all(|&f| f == 6));
        assert_eq!(p, SparseProjection::fixed_fan_in(50, 2000, 6, 42).unwrap());
        assert_ne!(p, SparseProjection::fixed_fan_in(50, 2000, 6, 43).unwrap());
        assert_eq!(p.out_degree().iter().sum::<usize>(), 12_000);
    }

    #[test]
    fn project_sums_sampled_inputs() {
        let p =
            SparseProjection::from_edges(3, 2, &[(0, 0, 1.0), (2, 0, 1.0), (1, 1, 1.0)]).unwrap();
        assert!(p.is_binary());
        let mut out = [0.0; 2];
        p.project_into(&[1.0, 2.0, 4.0], &mut out).unwrap();
        assert_eq!(out, [5.0, 2.0]);
    }

    #[test]
    fn from_edges_merges_duplicates_and_keeps_weights() {
        let p =
            SparseProjection::from_edges(3, 1, &[(1, 0, 2.0), (1, 0, 3.0), (0, 0, 1.0)]).unwrap();
        assert_eq!(p.row(0), vec![0, 1]);
        assert_eq!(p.row_weights(0), vec![1.0, 5.0]);
        let mut out = [0.0];
        p.project_into(&[1.0, 1.0, 1.0], &mut out).unwrap();
        assert_eq!(out, [6.0]);
        assert!(SparseProjection::from_edges(3, 1, &[(3, 0, 1.0)]).is_err());
    }

    #[test]
    fn dimension_mismatch_is_an_error() {
        let p = SparseProjection::fixed_fan_in(4, 4, 2, 1).unwrap();
        let mut out = [0.0; 4];
        assert!(matches!(
            p.project_into(&[1.0; 3], &mut out),
            Err(KenyonError::DimensionMismatch {
                expected: 4,
                got: 3
            })
        ));
    }

    #[test]
    fn bernoulli_density() {
        let p = SparseProjection::bernoulli(100, 1000, 0.1, 5).unwrap();
        let density = p.n_edges() as f64 / 100_000.0;
        assert!((density - 0.1).abs() < 0.01, "{density}");
    }
}
