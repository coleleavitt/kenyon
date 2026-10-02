//! Error type for every fallible `kenyon` operation.

use thiserror::Error;

/// Errors returned by `kenyon`.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum KenyonError {
    /// An input vector (or a batch row) had the wrong length.
    #[error("dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch {
        /// Length the structure was built for.
        expected: usize,
        /// Length that was passed.
        got: usize,
    },
    /// A constructor parameter was out of range.
    #[error("invalid parameter `{name}`: {reason}")]
    InvalidParameter {
        /// Parameter name.
        name: &'static str,
        /// Why it was rejected.
        reason: String,
    },
    /// An edge referenced an input or output unit that does not exist.
    #[error("edge ({input} -> {output}) out of range for {n_inputs} inputs x {n_outputs} outputs")]
    EdgeOutOfRange {
        /// Input (pre-synaptic) index.
        input: usize,
        /// Output (post-synaptic) index.
        output: usize,
        /// Number of inputs.
        n_inputs: usize,
        /// Number of outputs.
        n_outputs: usize,
    },
    /// A class label was outside `0..n_classes`.
    #[error("class {class} out of range (n_classes = {n_classes})")]
    ClassOutOfRange {
        /// Offending label.
        class: usize,
        /// Number of classes the model was built with.
        n_classes: usize,
    },
    /// Two codes of different dimension were compared.
    #[error("code dimension mismatch: {left} vs {right}")]
    CodeMismatch {
        /// Dimension of the left code.
        left: usize,
        /// Dimension of the right code.
        right: usize,
    },
}

/// Convenience alias.
pub type Result<T> = std::result::Result<T, KenyonError>;

pub(crate) fn invalid(name: &'static str, reason: impl Into<String>) -> KenyonError {
    KenyonError::InvalidParameter {
        name,
        reason: reason.into(),
    }
}

pub(crate) fn check_dim(expected: usize, got: usize) -> Result<()> {
    if expected == got {
        Ok(())
    } else {
        Err(KenyonError::DimensionMismatch { expected, got })
    }
}
