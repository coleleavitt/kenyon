//! A small, portable, seeded PRNG (xoshiro256** seeded through SplitMix64).
//!
//! `kenyon` ships its own generator instead of depending on `rand` so that a
//! given seed produces the same projection matrix on every platform and in
//! every future version of the crate: published hashes stay comparable.
//! Reference: Blackman & Vigna, "Scrambled linear pseudorandom number
//! generators", ACM TOMS 47(4), 2021.

/// Deterministic pseudo-random generator. Not cryptographic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rng {
    s: [u64; 4],
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

impl Rng {
    /// A generator seeded from one `u64`.
    pub fn new(seed: u64) -> Self {
        let mut sm = seed;
        let s = [
            splitmix64(&mut sm),
            splitmix64(&mut sm),
            splitmix64(&mut sm),
            splitmix64(&mut sm),
        ];
        Self { s }
    }

    /// An independent stream derived from `(seed, stream)`. Used to give each
    /// component (projection, tag, ...) its own reproducible stream.
    pub fn derive(seed: u64, stream: u64) -> Self {
        let mut sm = seed ^ stream.wrapping_mul(0xD1B5_4A32_D192_ED03);
        let _ = splitmix64(&mut sm);
        Self::new(splitmix64(&mut sm))
    }

    /// Next raw 64-bit output.
    pub fn next_u64(&mut self) -> u64 {
        let result = self.s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        result
    }

    /// Uniform `f64` in `[0, 1)` (53 random bits).
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// Uniform integer in `0..n` (Lemire's unbiased method). `n` must be > 0;
    /// `n == 0` returns 0.
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        let n = n as u64;
        loop {
            let x = self.next_u64();
            let m = (x as u128) * (n as u128);
            let lo = m as u64;
            if lo >= n || lo >= (n.wrapping_neg() % n) {
                return (m >> 64) as usize;
            }
        }
    }

    /// Standard normal sample (Box-Muller).
    pub fn normal(&mut self) -> f64 {
        let u1 = 1.0 - self.next_f64(); // (0, 1]
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
    }

    /// In-place Fisher-Yates shuffle.
    pub fn shuffle<T>(&mut self, xs: &mut [T]) {
        for i in (1..xs.len()).rev() {
            let j = self.below(i + 1);
            xs.swap(i, j);
        }
    }

    /// `k` distinct values from `0..n` in sampling order (Floyd's algorithm
    /// for small `k`, partial Fisher-Yates otherwise). `k` is clamped to `n`.
    pub fn sample_distinct(&mut self, n: usize, k: usize) -> Vec<usize> {
        let k = k.min(n);
        if k * 4 <= n {
            let mut chosen = std::collections::HashSet::with_capacity(k);
            let mut out = Vec::with_capacity(k);
            for j in (n - k)..n {
                let t = self.below(j + 1);
                let v = if chosen.contains(&t) { j } else { t };
                chosen.insert(v);
                out.push(v);
            }
            out
        } else {
            let mut all: Vec<usize> = (0..n).collect();
            for i in 0..k {
                let j = i + self.below(n - i);
                all.swap(i, j);
            }
            all.truncate(k);
            all
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_stream() {
        let mut a = Rng::new(7);
        let mut b = Rng::new(7);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
        assert_ne!(Rng::new(7).next_u64(), Rng::new(8).next_u64());
        assert_ne!(Rng::derive(7, 1).next_u64(), Rng::derive(7, 2).next_u64());
    }

    #[test]
    fn below_is_in_range_and_roughly_uniform() {
        let mut r = Rng::new(1);
        let mut counts = [0usize; 10];
        for _ in 0..100_000 {
            counts[r.below(10)] += 1;
        }
        for c in counts {
            assert!((9_000..11_000).contains(&c), "{counts:?}");
        }
    }

    #[test]
    fn sample_distinct_is_distinct() {
        let mut r = Rng::new(3);
        for (n, k) in [(10, 3), (10, 10), (1000, 6), (5, 9)] {
            let s = r.sample_distinct(n, k);
            let set: std::collections::HashSet<_> = s.iter().collect();
            assert_eq!(set.len(), k.min(n));
            assert!(s.iter().all(|&v| v < n));
        }
    }

    #[test]
    fn normal_moments() {
        let mut r = Rng::new(11);
        let xs: Vec<f64> = (0..200_000).map(|_| r.normal()).collect();
        let mean = xs.iter().sum::<f64>() / xs.len() as f64;
        let var = xs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / xs.len() as f64;
        assert!(mean.abs() < 0.01, "{mean}");
        assert!((var - 1.0).abs() < 0.02, "{var}");
    }
}
