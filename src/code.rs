//! Hash codes: the sparse KC "tag" and a packed bit vector.

use crate::error::{KenyonError, Result};

/// A sparse KC code: the active units of an `m`-dimensional tag, sorted
/// ascending, with the activation each one kept after winner-take-all.
///
/// Dasgupta 2017 ranks neighbours by Euclidean distance between tags that keep
/// the winners' values (preprint p.7-8); [`KcCode::euclidean_sq`] does that.
/// The binary variant (indices only, preprint p.9) compares by
/// [`KcCode::overlap`] / [`KcCode::hamming`].
#[derive(Debug, Clone, PartialEq)]
pub struct KcCode {
    dim: u32,
    indices: Vec<u32>,
    values: Vec<f32>,
}

impl KcCode {
    /// Build from parallel `indices` / `values`; indices are sorted here.
    pub fn new(dim: usize, indices: Vec<u32>, values: Vec<f32>) -> Result<Self> {
        if indices.len() != values.len() {
            return Err(KenyonError::DimensionMismatch {
                expected: indices.len(),
                got: values.len(),
            });
        }
        let dim32 =
            u32::try_from(dim).map_err(|_| crate::error::invalid("dim", "must fit in u32"))?;
        if let Some(&bad) = indices.iter().find(|&&i| i >= dim32) {
            return Err(crate::error::invalid(
                "indices",
                format!("index {bad} >= dim {dim}"),
            ));
        }
        let mut pairs: Vec<(u32, f32)> = indices.into_iter().zip(values).collect();
        pairs.sort_unstable_by_key(|p| p.0);
        pairs.dedup_by_key(|p| p.0);
        let (indices, values) = pairs.into_iter().unzip();
        Ok(Self {
            dim: dim32,
            indices,
            values,
        })
    }

    pub(crate) fn from_sorted(dim: u32, indices: Vec<u32>, values: Vec<f32>) -> Self {
        Self {
            dim,
            indices,
            values,
        }
    }

    /// Code dimension `m`.
    pub fn dim(&self) -> usize {
        self.dim as usize
    }

    /// Active unit indices, ascending.
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// Activation values, aligned with [`KcCode::indices`].
    pub fn values(&self) -> &[f32] {
        &self.values
    }

    /// Number of active units.
    pub fn len(&self) -> usize {
        self.indices.len()
    }

    /// `true` if no unit is active.
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    fn same_dim(&self, other: &Self) -> Result<()> {
        if self.dim == other.dim {
            Ok(())
        } else {
            Err(KenyonError::CodeMismatch {
                left: self.dim as usize,
                right: other.dim as usize,
            })
        }
    }

    /// Number of shared active units.
    pub fn overlap(&self, other: &Self) -> Result<usize> {
        self.same_dim(other)?;
        let (mut i, mut j, mut n) = (0, 0, 0);
        while i < self.indices.len() && j < other.indices.len() {
            match self.indices[i].cmp(&other.indices[j]) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Greater => j += 1,
                std::cmp::Ordering::Equal => {
                    n += 1;
                    i += 1;
                    j += 1;
                }
            }
        }
        Ok(n)
    }

    /// Hamming distance between the binary versions of the two codes.
    pub fn hamming(&self, other: &Self) -> Result<usize> {
        let o = self.overlap(other)?;
        Ok(self.len() + other.len() - 2 * o)
    }

    /// Jaccard similarity of the active sets (1 for two empty codes).
    pub fn jaccard(&self, other: &Self) -> Result<f64> {
        let o = self.overlap(other)?;
        let u = self.len() + other.len() - o;
        Ok(if u == 0 { 1.0 } else { o as f64 / u as f64 })
    }

    /// Squared Euclidean distance between the two tags as sparse vectors.
    pub fn euclidean_sq(&self, other: &Self) -> Result<f64> {
        self.same_dim(other)?;
        let (mut i, mut j, mut s) = (0, 0, 0f64);
        let (a, b) = (&self.indices, &other.indices);
        while i < a.len() || j < b.len() {
            let ai = a.get(i).copied().unwrap_or(u32::MAX);
            let bj = b.get(j).copied().unwrap_or(u32::MAX);
            if ai == bj {
                let d = f64::from(self.values[i]) - f64::from(other.values[j]);
                s += d * d;
                i += 1;
                j += 1;
            } else if ai < bj {
                s += f64::from(self.values[i]).powi(2);
                i += 1;
            } else {
                s += f64::from(other.values[j]).powi(2);
                j += 1;
            }
        }
        Ok(s)
    }

    /// The binary version as a packed bit vector.
    pub fn to_bits(&self) -> BitCode {
        let mut b = BitCode::zeros(self.dim as usize);
        for &i in &self.indices {
            b.set(i as usize);
        }
        b
    }

    /// Heap bytes held.
    pub fn memory_bytes(&self) -> usize {
        self.indices.capacity() * 4 + self.values.capacity() * 4
    }
}

/// A packed bit vector (DenseFly hashes, binary FlyHash tags).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitCode {
    len: usize,
    words: Vec<u64>,
}

impl BitCode {
    /// All-zero code of `len` bits.
    pub fn zeros(len: usize) -> Self {
        Self {
            len,
            words: vec![0; len.div_ceil(64)],
        }
    }

    /// Number of bits.
    pub fn len(&self) -> usize {
        self.len
    }

    /// `true` for a zero-length code.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Set bit `i` (ignored if out of range).
    pub fn set(&mut self, i: usize) {
        if i < self.len {
            self.words[i / 64] |= 1u64 << (i % 64);
        }
    }

    /// Read bit `i` (`false` if out of range).
    pub fn get(&self, i: usize) -> bool {
        i < self.len && (self.words[i / 64] >> (i % 64)) & 1 == 1
    }

    /// Number of set bits.
    pub fn count_ones(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }

    /// Hamming distance (popcount of XOR).
    pub fn hamming(&self, other: &Self) -> Result<usize> {
        if self.len != other.len {
            return Err(KenyonError::CodeMismatch {
                left: self.len,
                right: other.len,
            });
        }
        Ok(self
            .words
            .iter()
            .zip(&other.words)
            .map(|(a, b)| (a ^ b).count_ones() as usize)
            .sum())
    }

    /// Number of bits set in both.
    pub fn overlap(&self, other: &Self) -> Result<usize> {
        if self.len != other.len {
            return Err(KenyonError::CodeMismatch {
                left: self.len,
                right: other.len,
            });
        }
        Ok(self
            .words
            .iter()
            .zip(&other.words)
            .map(|(a, b)| (a & b).count_ones() as usize)
            .sum())
    }

    /// The packed words (little-endian bit order within each word).
    pub fn words(&self) -> &[u64] {
        &self.words
    }

    /// Heap bytes held.
    pub fn memory_bytes(&self) -> usize {
        self.words.capacity() * 8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_hamming_euclid() {
        let a = KcCode::new(10, vec![1, 3, 5], vec![1.0, 2.0, 3.0]).unwrap();
        let b = KcCode::new(10, vec![5, 3, 7], vec![3.0, 1.0, 4.0]).unwrap();
        assert_eq!(a.overlap(&b).unwrap(), 2);
        assert_eq!(a.hamming(&b).unwrap(), 2);
        // (1-0)^2 + (2-1)^2 + (3-3)^2 + (0-4)^2
        assert_eq!(a.euclidean_sq(&b).unwrap(), 18.0);
        assert_eq!(a.to_bits().hamming(&b.to_bits()).unwrap(), 2);
        assert_eq!(a.to_bits().overlap(&b.to_bits()).unwrap(), 2);
        assert!((a.jaccard(&b).unwrap() - 0.5).abs() < 1e-12);
    }

    #[test]
    fn mismatched_dims_error() {
        let a = KcCode::new(10, vec![1], vec![1.0]).unwrap();
        let b = KcCode::new(11, vec![1], vec![1.0]).unwrap();
        assert!(a.overlap(&b).is_err());
        assert!(KcCode::new(4, vec![4], vec![1.0]).is_err());
    }
}
