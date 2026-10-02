//! Similarity search over stored codes.
//!
//! [`FlyIndex`] keeps an inverted list per KC (which items activate it), so a
//! query only touches the items that share at least one active unit with it;
//! items are ranked by overlap (equivalently Hamming distance for fixed-`k`
//! codes). [`DenseFlyIndex`] implements DenseFly's multi-probe search: bins
//! keyed by the pseudo-hash are probed at increasing Hamming radius, and the
//! pooled candidates are ranked by the high-dimensional hash (Sharma &
//! Navlakha 2018, Sec. 3.4).

use std::collections::HashMap;

use crate::code::{BitCode, KcCode};
use crate::error::{KenyonError, Result};

/// A search hit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
    /// Item id (insertion order).
    pub id: u32,
    /// Similarity score (overlap for [`FlyIndex`], negative Hamming for
    /// [`DenseFlyIndex`]); higher is closer.
    pub score: f64,
}

fn rank(mut hits: Vec<Hit>, top_n: usize) -> Vec<Hit> {
    hits.sort_by(|a, b| b.score.total_cmp(&a.score).then(a.id.cmp(&b.id)));
    hits.truncate(top_n);
    hits
}

/// Inverted index over sparse KC codes.
#[derive(Debug, Clone, Default)]
pub struct FlyIndex {
    dim: usize,
    postings: Vec<Vec<u32>>,
    codes: Vec<KcCode>,
}

impl FlyIndex {
    /// Empty index for codes of dimension `dim`.
    pub fn new(dim: usize) -> Self {
        Self {
            dim,
            postings: vec![Vec::new(); dim],
            codes: Vec::new(),
        }
    }

    /// Number of stored items.
    pub fn len(&self) -> usize {
        self.codes.len()
    }

    /// `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.codes.is_empty()
    }

    /// Store a code; returns its id.
    pub fn insert(&mut self, code: KcCode) -> Result<u32> {
        if code.dim() != self.dim {
            return Err(KenyonError::CodeMismatch {
                left: self.dim,
                right: code.dim(),
            });
        }
        let id = u32::try_from(self.codes.len())
            .map_err(|_| crate::error::invalid("len", "index full"))?;
        for &u in code.indices() {
            self.postings[u as usize].push(id);
        }
        self.codes.push(code);
        Ok(id)
    }

    /// The stored code for `id`.
    pub fn get(&self, id: u32) -> Option<&KcCode> {
        self.codes.get(id as usize)
    }

    /// The `top_n` items with the largest overlap with `query` (ties by id).
    /// Items sharing no unit are never returned.
    pub fn query_overlap(&self, query: &KcCode, top_n: usize) -> Result<Vec<Hit>> {
        if query.dim() != self.dim {
            return Err(KenyonError::CodeMismatch {
                left: self.dim,
                right: query.dim(),
            });
        }
        let mut counts: HashMap<u32, u32> = HashMap::new();
        for &u in query.indices() {
            for &id in &self.postings[u as usize] {
                *counts.entry(id).or_insert(0) += 1;
            }
        }
        let hits = counts
            .into_iter()
            .map(|(id, c)| Hit {
                id,
                score: f64::from(c),
            })
            .collect();
        Ok(rank(hits, top_n))
    }

    /// Linear scan ranking by Euclidean distance between valued tags
    /// (Dasgupta 2017's evaluation); score is the negative squared distance.
    pub fn query_euclidean(&self, query: &KcCode, top_n: usize) -> Result<Vec<Hit>> {
        let mut hits = Vec::with_capacity(self.codes.len());
        for (id, c) in self.codes.iter().enumerate() {
            hits.push(Hit {
                id: id as u32,
                score: -query.euclidean_sq(c)?,
            });
        }
        Ok(rank(hits, top_n))
    }

    /// Heap bytes held (codes + postings).
    pub fn memory_bytes(&self) -> usize {
        self.postings
            .iter()
            .map(|p| p.capacity() * 4 + 24)
            .sum::<usize>()
            + self
                .codes
                .iter()
                .map(|c| c.memory_bytes() + 40)
                .sum::<usize>()
    }
}

/// DenseFly multi-probe index (one table).
#[derive(Debug, Clone, Default)]
pub struct DenseFlyIndex {
    bins: HashMap<BitCode, Vec<u32>>,
    codes: Vec<BitCode>,
}

impl DenseFlyIndex {
    /// Empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of stored items.
    pub fn len(&self) -> usize {
        self.codes.len()
    }

    /// `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.codes.is_empty()
    }

    /// Number of non-empty bins.
    pub fn n_bins(&self) -> usize {
        self.bins.len()
    }

    /// Store `(h1, h2)` from [`crate::DenseFly::hash`]; returns the id.
    pub fn insert(&mut self, h1: BitCode, h2: BitCode) -> Result<u32> {
        if let Some(first) = self.codes.first() {
            if first.len() != h1.len() {
                return Err(KenyonError::CodeMismatch {
                    left: first.len(),
                    right: h1.len(),
                });
            }
        }
        let id = u32::try_from(self.codes.len())
            .map_err(|_| crate::error::invalid("len", "index full"))?;
        self.bins.entry(h2).or_default().push(id);
        self.codes.push(h1);
        Ok(id)
    }

    /// Probe bins within increasing pseudo-hash radius until at least
    /// `min_candidates` items are pooled (or every bin is used), then rank them
    /// by Hamming distance on `h1`. Returns hits and the radius used.
    pub fn query(
        &self,
        h1: &BitCode,
        h2: &BitCode,
        min_candidates: usize,
        top_n: usize,
    ) -> Result<(Vec<Hit>, usize)> {
        let mut by_dist: Vec<(usize, &Vec<u32>)> = Vec::with_capacity(self.bins.len());
        for (key, ids) in &self.bins {
            by_dist.push((key.hamming(h2)?, ids));
        }
        by_dist.sort_by_key(|(d, ids)| (*d, ids.first().copied()));
        let mut cands: Vec<u32> = Vec::new();
        let mut radius = 0;
        let mut i = 0;
        while i < by_dist.len() {
            radius = by_dist[i].0;
            while i < by_dist.len() && by_dist[i].0 == radius {
                cands.extend_from_slice(by_dist[i].1);
                i += 1;
            }
            if cands.len() >= min_candidates {
                break;
            }
        }
        let mut hits = Vec::with_capacity(cands.len());
        for id in cands {
            let d = h1.hamming(&self.codes[id as usize])?;
            hits.push(Hit {
                id,
                score: -(d as f64),
            });
        }
        Ok((rank(hits, top_n), radius))
    }

    /// Heap bytes held (approximate: codes, bin keys and id lists).
    pub fn memory_bytes(&self) -> usize {
        self.codes
            .iter()
            .map(|c| c.memory_bytes() + 32)
            .sum::<usize>()
            + self
                .bins
                .iter()
                .map(|(k, v)| k.memory_bytes() + 32 + v.capacity() * 4 + 24)
                .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DenseFly, FlyHash, FlyHashParams, Preprocess, Rng};

    #[test]
    fn exact_item_is_top_hit() {
        let h = FlyHash::new(FlyHashParams::paper(40, 16, 3)).unwrap();
        let mut r = Rng::new(1);
        let xs: Vec<Vec<f32>> = (0..200)
            .map(|_| (0..40).map(|_| r.next_f64() as f32).collect())
            .collect();
        let mut idx = FlyIndex::new(h.n_kc());
        for x in &xs {
            idx.insert(h.hash(x).unwrap()).unwrap();
        }
        let q = h.hash(&xs[17]).unwrap();
        assert_eq!(idx.query_overlap(&q, 1).unwrap()[0].id, 17);
        assert_eq!(idx.query_euclidean(&q, 1).unwrap()[0].id, 17);

        let df = DenseFly::new(40, 400, 4, 20, Preprocess::CenterMean, 3).unwrap();
        let mut di = DenseFlyIndex::new();
        for x in &xs {
            let (a, b) = df.hash(x).unwrap();
            di.insert(a, b).unwrap();
        }
        let (a, b) = df.hash(&xs[17]).unwrap();
        let (hits, radius) = di.query(&a, &b, 10, 5).unwrap();
        assert_eq!(hits[0].id, 17);
        assert!(radius <= df.pseudo_len());
    }
}
