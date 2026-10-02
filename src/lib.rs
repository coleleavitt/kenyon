//! # kenyon
//!
//! Algorithms from the fruit fly's mushroom body, implemented as small,
//! deterministic, dependency-light building blocks:
//!
//! - [`FlyHash`]: sparse binary random expansion + winner-take-all, a
//!   locality-sensitive hash (Dasgupta, Stevens & Navlakha, *Science* 2017),
//!   and [`DenseFly`] (Sharma & Navlakha 2018), with a similarity index
//!   ([`FlyIndex`], [`DenseFlyIndex`]).
//! - [`FlyBloom`]: the fly Bloom filter, a distance- and time-sensitive
//!   novelty detector (Dasgupta et al., *PNAS* 2018).
//! - [`FlyModel`]: continual learning with sparse coding and partial freezing
//!   on top of a frozen encoder (Shen, Dasgupta & Navlakha 2021).
//!
//! Every random structure is built from a `u64` seed by the crate's own
//! portable generator ([`Rng`]), so the same seed gives the same hash on every
//! platform. Projections can also be built from measured wiring
//! ([`SparseProjection::from_edges`]), e.g. a connectome's PN->KC synapses.
//!
//! What this crate is for: novelty detection, continual learning and
//! tiny-memory similarity codes. It is **not** a replacement for HNSW or
//! product quantisation; see `RESULTS.md` for where fly hashing loses.
//!
//! ```
//! use kenyon::{BloomParams, FlyBloom, FlyHash, FlyHashParams};
//! # fn main() -> kenyon::Result<()> {
//! let hasher = FlyHash::new(FlyHashParams::paper(64, 16, 42))?;
//! let x: Vec<f32> = (0..64).map(|i| (i % 7) as f32).collect();
//! let code = hasher.hash(&x)?;
//! assert_eq!(code.len(), 16);
//!
//! let mut filter = FlyBloom::new(hasher, BloomParams::STATIC)?;
//! assert_eq!(filter.novelty(&x)?, 1.0);
//! filter.insert(&x)?;
//! assert_eq!(filter.novelty(&x)?, 0.0);
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod bloom;
mod code;
mod error;
mod hash;
mod index;
mod model;
mod preprocess;
mod projection;
mod rng;
pub mod wta;

pub use bloom::{BloomParams, FlyBloom};
pub use code::{BitCode, KcCode};
pub use error::{KenyonError, Result};
pub use hash::{DenseFly, FlyHash, FlyHashParams, Scratch, TagRule};
pub use index::{DenseFlyIndex, FlyIndex, Hit};
pub use model::{Coding, FlyModel, FlyModelParams, LearningRule};
pub use preprocess::Preprocess;
pub use projection::{DenseProjection, Expansion, SparseProjection};
pub use rng::Rng;
