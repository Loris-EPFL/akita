//! Consuming protocol completion capabilities. Public contexts alone never
//! establish that a strong check, normalization or terminal driver completed.
use super::ProofContext;

/// Issued only by the native driver after its complete arithmetic check.
/// It is neither cloneable nor publicly constructible.
///
/// A caller-held operation context cannot manufacture completion authority:
/// ```compile_fail
/// use akita_prover::backend::{FullRingCompletion, FullRingCompletionKind, ProofContext};
/// fn forge(context: ProofContext) -> FullRingCompletion {
///     FullRingCompletion { context, kind: FullRingCompletionKind::Terminal }
/// }
/// ```
pub struct FullRingCompletion {
    context: ProofContext,
    kind: FullRingCompletionKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FullRingCompletionKind {
    Reduction { commitment_domain_len: usize },
    FinalNormalization,
    Terminal,
}

impl FullRingCompletion {
    pub(crate) fn new(context: ProofContext, kind: FullRingCompletionKind) -> Self {
        Self { context, kind }
    }
    pub fn context(&self) -> &ProofContext {
        &self.context
    }
    pub fn kind(&self) -> FullRingCompletionKind {
        self.kind
    }
}
