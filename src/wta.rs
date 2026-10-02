//! Winner-take-all: keep the `k` most active units.
//!
//! In the fly, the APL neuron's feedback inhibition leaves only the top ~5% of
//! KCs firing (Dasgupta 2017 preprint p.6 and Fig. 1). Ties are broken by the
//! lower unit index so the result is a deterministic function of the input.

use std::cmp::Ordering;

/// `true` if unit `a` beats unit `b` (higher value; lower index on ties;
/// NaN loses to everything).
fn beats(values: &[f32], a: u32, b: u32) -> Ordering {
    let (va, vb) = (values[a as usize], values[b as usize]);
    let ka = if va.is_nan() { f32::NEG_INFINITY } else { va };
    let kb = if vb.is_nan() { f32::NEG_INFINITY } else { vb };
    kb.total_cmp(&ka).then(a.cmp(&b))
}

/// Indices of the `k` largest entries of `values`, sorted ascending by index.
/// `scratch` is reused between calls to avoid allocation. `k` is clamped to
/// `values.len()`.
pub fn top_k(values: &[f32], k: usize, scratch: &mut Vec<u32>) -> Vec<u32> {
    let k = k.min(values.len());
    if k == 0 {
        return Vec::new();
    }
    scratch.clear();
    scratch.extend(0..values.len() as u32);
    if k < scratch.len() {
        scratch.select_nth_unstable_by(k - 1, |&a, &b| beats(values, a, b));
    }
    let mut out = scratch[..k].to_vec();
    out.sort_unstable();
    out
}

/// Like [`top_k`] but only among strictly positive entries (FlyModel Eq. 2:
/// "one of the l largest positive entries", Shen, Dasgupta & Navlakha 2021
/// p.4). May return fewer than `k` units.
pub fn top_k_positive(values: &[f32], k: usize, scratch: &mut Vec<u32>) -> Vec<u32> {
    let mut out = top_k(values, k, scratch);
    out.retain(|&i| values[i as usize] > 0.0);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_largest_with_index_tiebreak() {
        let v = [0.5, 3.0, 3.0, -1.0, 2.0, f32::NAN];
        let mut s = Vec::new();
        assert_eq!(top_k(&v, 2, &mut s), vec![1, 2]);
        assert_eq!(top_k(&v, 3, &mut s), vec![1, 2, 4]);
        assert_eq!(top_k(&[1.0, 1.0, 1.0], 2, &mut s), vec![0, 1]);
        assert_eq!(top_k(&v, 0, &mut s), Vec::<u32>::new());
        assert_eq!(top_k(&v, 99, &mut s).len(), 6);
        assert_eq!(top_k_positive(&[-1.0, 0.0, 2.0], 2, &mut s), vec![2]);
    }
}
