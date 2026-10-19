//! Reproduction harness for `kenyon`: dataset loaders, retrieval metrics and
//! small statistics helpers shared by the `src/bin/*` experiments.
//! Not published; see `RESULTS.md` and `research/wiring/`.

pub mod data;
pub mod flyrag_bench;
pub mod metrics;
pub mod stats;

use std::path::PathBuf;

/// Dataset root (`$KENYON_DATA`, default `~/datasets`).
pub fn data_dir() -> PathBuf {
    if let Ok(p) = std::env::var("KENYON_DATA") {
        return PathBuf::from(p);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join("datasets")
}

/// Output directory for result files (`$KENYON_OUT`, default `results/`).
pub fn out_dir() -> PathBuf {
    let p = std::env::var("KENYON_OUT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("results"));
    let _ = std::fs::create_dir_all(&p);
    p
}

/// A dense row-major matrix of `f32`.
#[derive(Debug, Clone)]
pub struct Matrix {
    /// Rows.
    pub n: usize,
    /// Columns.
    pub d: usize,
    /// `n * d` values.
    pub data: Vec<f32>,
}

impl Matrix {
    /// Row `i`.
    pub fn row(&self, i: usize) -> &[f32] {
        &self.data[i * self.d..(i + 1) * self.d]
    }

    /// The rows listed in `idx`, in that order.
    pub fn select(&self, idx: &[usize]) -> Matrix {
        let mut data = Vec::with_capacity(idx.len() * self.d);
        for &i in idx {
            data.extend_from_slice(self.row(i));
        }
        Matrix {
            n: idx.len(),
            d: self.d,
            data,
        }
    }
}
