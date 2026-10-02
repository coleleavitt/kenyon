//! Retrieval and correlation metrics.

/// Average precision of a predicted ranking against a true neighbour set.
/// Returns `(ap_over_r, ap_over_hits)`:
/// - `ap_over_r` = sum over hits of precision@i, divided by `r` (ml_metrics
///   `apk`, the predeclared primary for R1);
/// - `ap_over_hits` = the same sum divided by the number of hits (0 if none).
pub fn average_precision(predicted: &[u32], truth: &[u32], r: usize) -> (f64, f64) {
    let set: std::collections::HashSet<u32> = truth.iter().copied().collect();
    let mut hits = 0usize;
    let mut sum = 0f64;
    for (i, p) in predicted.iter().take(r).enumerate() {
        if set.contains(p) {
            hits += 1;
            sum += hits as f64 / (i + 1) as f64;
        }
    }
    let denom = r.min(truth.len()).max(1);
    (sum / denom as f64, if hits == 0 { 0.0 } else { sum / hits as f64 })
}

/// Indices of the `r` smallest `dist` values, excluding `exclude`, ordered by
/// distance and then by `order[i]` (a random rank used to break ties).
pub fn top_r_by(dist: &[f64], order: &[u32], exclude: usize, r: usize) -> Vec<u32> {
    let mut idx: Vec<u32> = (0..dist.len() as u32).filter(|&i| i as usize != exclude).collect();
    let key = |i: &u32| (dist[*i as usize], order[*i as usize]);
    let cmp = |a: &u32, b: &u32| {
        let (da, oa) = key(a);
        let (db, ob) = key(b);
        da.total_cmp(&db).then(oa.cmp(&ob))
    };
    let r = r.min(idx.len());
    if r == 0 {
        return Vec::new();
    }
    if r < idx.len() {
        idx.select_nth_unstable_by(r - 1, cmp);
        idx.truncate(r);
    }
    idx.sort_unstable_by(cmp);
    idx
}

/// Pearson correlation (0 if either side is constant).
pub fn pearson(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len()) as f64;
    if n < 2.0 {
        return 0.0;
    }
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let (mut sxy, mut sxx, mut syy) = (0.0, 0.0, 0.0);
    for (a, b) in x.iter().zip(y) {
        sxy += (a - mx) * (b - my);
        sxx += (a - mx).powi(2);
        syy += (b - my).powi(2);
    }
    if sxx == 0.0 || syy == 0.0 { 0.0 } else { sxy / (sxx * syy).sqrt() }
}

/// Squared Euclidean distance.
pub fn sq_dist(a: &[f32], b: &[f32]) -> f64 {
    a.iter().zip(b).map(|(x, y)| f64::from(x - y).powi(2)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ap_examples() {
        let (a, b) = average_precision(&[1, 2, 3, 4], &[1, 3, 9, 8], 4);
        // hits at 1 (1/1) and 3 (2/3)
        assert!((a - (1.0 + 2.0 / 3.0) / 4.0).abs() < 1e-12);
        assert!((b - (1.0 + 2.0 / 3.0) / 2.0).abs() < 1e-12);
        assert_eq!(average_precision(&[5], &[1], 1), (0.0, 0.0));
    }

    #[test]
    fn top_r_excludes_and_breaks_ties() {
        let d = [0.0, 1.0, 1.0, 0.5];
        assert_eq!(top_r_by(&d, &[0, 2, 1, 3], 0, 2), vec![3, 2]);
    }

    #[test]
    fn pearson_basic() {
        assert!((pearson(&[1.0, 2.0, 3.0], &[2.0, 4.0, 6.0]) - 1.0).abs() < 1e-12);
        assert_eq!(pearson(&[1.0, 1.0], &[1.0, 2.0]), 0.0);
    }
}
