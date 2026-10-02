//! Input pre-processing applied before the expansion.
//!
//! The fly's projection neurons remove odour concentration so every odour has
//! nearly the same mean firing rate ("centers the mean", divisive
//! normalisation; Dasgupta 2017 preprint p.2). DenseFly states its only
//! pre-processing step is "to center each input vector about the mean"
//! (Sharma & Navlakha 2018, p.4). Kleyko & Rachkovskij 2024 (p.3) find
//! centring plus normalising matters.

/// How an input vector is transformed before projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Preprocess {
    /// Use the input as is.
    #[default]
    None,
    /// Subtract the vector's own mean.
    CenterMean,
    /// Subtract the mean, then scale to unit L2 norm (zero vectors stay zero).
    CenterUnit,
}

impl Preprocess {
    /// Apply into `out` (same length as `x`).
    pub fn apply_into(self, x: &[f32], out: &mut Vec<f32>) {
        out.clear();
        out.extend_from_slice(x);
        match self {
            Self::None => {}
            Self::CenterMean | Self::CenterUnit => {
                if x.is_empty() {
                    return;
                }
                let mean = x.iter().map(|&v| f64::from(v)).sum::<f64>() / x.len() as f64;
                for v in out.iter_mut() {
                    *v = (f64::from(*v) - mean) as f32;
                }
                if self == Self::CenterUnit {
                    let norm = out
                        .iter()
                        .map(|&v| f64::from(v).powi(2))
                        .sum::<f64>()
                        .sqrt();
                    if norm > 0.0 {
                        for v in out.iter_mut() {
                            *v = (f64::from(*v) / norm) as f32;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centering() {
        let mut o = Vec::new();
        Preprocess::CenterMean.apply_into(&[1.0, 2.0, 3.0], &mut o);
        assert_eq!(o, vec![-1.0, 0.0, 1.0]);
        Preprocess::CenterUnit.apply_into(&[1.0, 2.0, 3.0], &mut o);
        assert!((o[2] - 1.0 / 2f32.sqrt()).abs() < 1e-6);
        Preprocess::CenterUnit.apply_into(&[2.0, 2.0], &mut o);
        assert_eq!(o, vec![0.0, 0.0]);
    }
}
