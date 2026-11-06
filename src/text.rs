//! A seeded hashing text encoder — turn `&str` into the `f32[d]`
//! feature vector [`crate::FlyHash`], [`crate::FlyBloom`] and
//! [`crate::FlyModel`] expect as input.
//!
//! The paper surfaces take `&[f32]` only: Shen/Dasgupta/Navlakha 2021
//! ("A neural circuit for continual learning") assumes a frozen
//! encoder you supply, and Dasgupta et al. 2018 ("A neural data
//! structure for novelty detection") benchmarks over SIFT and odor
//! vectors — both already numeric. In practice downstream code starts
//! from text and needs a cheap, deterministic hash into a fixed
//! dimension. [`HashedText`] is that encoder, factored out of flyrag so
//! [`crate::FlyModel`] and [`crate::FlyBloom`] are usable from text
//! without a second crate.
//!
//! The recipe, pinned so persisted codes stay comparable across
//! versions:
//! - Lower-case alphanumeric tokens (no stemming, no stop-words).
//! - Unigrams and adjacent bigrams.
//! - Each token / bigram hashed with a seeded FNV-1a into one of
//!   `dim` non-negative buckets.
//! - Term weight `1 + ln(tf)` — never negative, so the vector is a
//!   firing-rate like the projection neurons'.
//!
//! The FNV constants, separator byte (`0x1f`), and
//! `1 + ln(tf)` recipe are a bit-level stability surface: a change is
//! a breaking change (see README's "Bit-level stability guarantee").

/// A seeded, hashing text encoder. `Copy` + `Serialize` so a downstream
/// learner can carry the encoder in its serialised state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HashedText {
    dim: usize,
    seed: u64,
}

impl HashedText {
    /// An encoder into `dim` buckets (`dim >= 1`).
    #[must_use]
    pub fn new(dim: usize, seed: u64) -> Self {
        Self {
            dim: dim.max(1),
            seed,
        }
    }

    /// Output dimension (`d`).
    #[must_use]
    pub fn dim(&self) -> usize {
        self.dim
    }

    /// The lower-cased alphanumeric tokens of `text`.
    #[must_use]
    pub fn tokens(text: &str) -> Vec<String> {
        text.split(|c: char| !c.is_alphanumeric())
            .filter(|t| !t.is_empty())
            .map(str::to_lowercase)
            .collect()
    }

    fn bucket(&self, parts: &[&str]) -> usize {
        // FNV-1a seeded with (offset_basis XOR seed); each part separated
        // by a `0x1f` byte so "foo" + "bar" doesn't hash as "foobar".
        let mut h: u64 = 0xcbf2_9ce4_8422_2325 ^ self.seed;
        for (i, p) in parts.iter().enumerate() {
            if i > 0 {
                h ^= 0x1f;
                h = h.wrapping_mul(0x0100_0000_01b3);
            }
            for b in p.bytes() {
                h ^= u64::from(b);
                h = h.wrapping_mul(0x0100_0000_01b3);
            }
        }
        (h % self.dim as u64) as usize
    }

    /// The distinct buckets `text`'s features fall in, sorted. Useful
    /// for fitting per-bucket weights (TF-IDF) before a full
    /// [`HashedText::encode`] call.
    #[must_use]
    pub fn buckets(&self, text: &str) -> Vec<usize> {
        let toks = Self::tokens(text);
        let mut b: Vec<usize> = toks.iter().map(|t| self.bucket(&[t])).collect();
        b.extend(toks.windows(2).map(|w| self.bucket(&[&w[0], &w[1]])));
        b.sort_unstable();
        b.dedup();
        b
    }

    /// The feature vector of `text`: `1 + ln(tf)` per bucket, zero
    /// where no token hashed in.
    #[must_use]
    pub fn encode(&self, text: &str) -> Vec<f32> {
        let toks = Self::tokens(text);
        let mut tf = vec![0u32; self.dim];
        for t in &toks {
            tf[self.bucket(&[t])] += 1;
        }
        for w in toks.windows(2) {
            tf[self.bucket(&[&w[0], &w[1]])] += 1;
        }
        tf.into_iter()
            .map(|c| if c == 0 { 0.0 } else { 1.0 + (c as f32).ln() })
            .collect()
    }

    /// [`HashedText::encode`] with each bucket multiplied by
    /// `weights[b]` (missing or short weights count as 1). The usual
    /// shape is a bucket-wise IDF fitted over a corpus before any
    /// doc is stored; the fit is downstream's concern.
    #[must_use]
    pub fn encode_weighted(&self, text: &str, weights: &[f32]) -> Vec<f32> {
        let mut x = self.encode(text);
        for (v, w) in x.iter_mut().zip(weights) {
            *v *= w;
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_are_lowercase_alphanumeric_runs() {
        // `_` is non-alphanumeric per `char::is_alphanumeric`, so
        // `install_dependencies` splits into two tokens — this is
        // deliberately coarse (no stemming, no punctuation-sensitive
        // segmentation): the downstream FNV hash makes bucket
        // collisions the expensive thing, not token boundaries.
        assert_eq!(
            HashedText::tokens("Use bun, not NPM! Also: install_dependencies"),
            ["use", "bun", "not", "npm", "also", "install", "dependencies"]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        );
        assert!(HashedText::tokens("").is_empty());
    }

    #[test]
    fn encoding_is_deterministic_and_seeded() {
        let a = HashedText::new(64, 1);
        let b = HashedText::new(64, 1);
        let c = HashedText::new(64, 2);
        let text = "install bun not npm";
        assert_eq!(a.encode(text), b.encode(text));
        assert_ne!(a.encode(text), c.encode(text));
    }

    #[test]
    fn tf_grows_as_one_plus_ln() {
        let e = HashedText::new(128, 7);
        let x = e.encode("bun bun bun");
        let max = x.iter().copied().fold(0.0f32, f32::max);
        // tf for "bun" = 3 (three unigrams) + 2 (two bigrams of "bun bun"),
        // so the "bun" bucket holds at least 3; ln(3) + 1 ~ 2.0986.
        assert!(max >= 1.0 + 3.0f32.ln() - 1e-5, "{x:?}");
        assert!(x.iter().all(|&v| v >= 0.0));
    }

    #[test]
    fn encode_weighted_multiplies_per_bucket() {
        let e = HashedText::new(64, 3);
        let x = e.encode("bun npm install");
        let weights = vec![0.5; 64];
        let w = e.encode_weighted("bun npm install", &weights);
        for (a, b) in x.iter().zip(&w) {
            assert!((a * 0.5 - b).abs() < 1e-6);
        }
    }

    #[test]
    fn empty_text_is_zero_vector() {
        let e = HashedText::new(32, 11);
        assert_eq!(e.encode(""), vec![0.0; 32]);
        assert!(e.buckets("").is_empty());
    }

    #[test]
    fn dim_clamps_to_one() {
        let e = HashedText::new(0, 5);
        assert_eq!(e.dim(), 1);
        assert_eq!(e.encode("anything").len(), 1);
    }
}
