//! Frontend emission capability over the exact retained native compiler.
use super::{NativeStrongTranscript, Operation};
use crate::native::E;
use akita_error::AkitaError;
use core::marker::PhantomData;
#[cfg(test)]
#[path = "borrow_tests.rs"]
mod tests;

/// A borrowing capability, passed by value to a consuming frontend callback.
/// It cannot replace, extract or close the owner's actual compiler. Application
/// adapters may borrow this capability again, but never receive `&mut T`.
///
/// This intentionally does not implement `NativeStrongTranscript`: that trait
/// includes final closure, which belongs only to the native run owner.
/// Loans require [`super::InnerExecutionRegistry`] or
/// [`super::OuterExecutionRegistry`]; lending the same compiler does not bypass
/// its native admission/publication/acceptance checks.
/// ```compile_fail
/// use akita_transcript::native_strong::{NativeStrongTranscript, NativeTranscriptBorrow};
/// fn close<T, R, D>(mut frontend: NativeTranscriptBorrow<'_, T, R, D>)
/// where T: NativeStrongTranscript<R, D> {
///     frontend.finish().unwrap();
/// }
/// ```
pub struct NativeTranscriptBorrow<'a, T, R, D>
where
    T: NativeStrongTranscript<R, D>,
{
    transcript: &'a mut T,
    marker: PhantomData<fn() -> (R, D)>,
}
impl<'a, T, R, D> NativeTranscriptBorrow<'a, T, R, D>
where
    T: NativeStrongTranscript<R, D>,
{
    pub fn new(transcript: &'a mut T) -> Self {
        Self {
            transcript,
            marker: PhantomData,
        }
    }
    pub fn reborrow(&mut self) -> NativeTranscriptBorrow<'_, T, R, D> {
        NativeTranscriptBorrow::new(self.transcript)
    }
    /// Poison the same retained compiler on a frontend-local validation error.
    /// This grants no reset, replacement or successful closure authority.
    pub fn reject(&mut self) {
        self.transcript.reject();
    }
    pub fn public(&mut self, site: &R, count: usize, bytes: &[u8]) -> Result<(), AkitaError> {
        self.transcript.public(site, count, bytes)
    }
    pub fn proof(&mut self, site: &R, count: usize, bytes: &[u8]) -> Result<(), AkitaError> {
        self.transcript.proof(site, count, bytes)
    }
    pub fn scalar(&mut self, site: &D, mode: Operation) -> Result<E, AkitaError> {
        self.transcript.scalar(site, mode)
    }
    pub fn vector(
        &mut self,
        site: &D,
        mode: Operation,
        count: usize,
    ) -> Result<Vec<E>, AkitaError> {
        self.transcript.vector(site, mode, count)
    }
}

impl<'a, G: super::InnerExecutionRegistry>
    NativeTranscriptBorrow<'a, crate::native::inner::Transcript<G>, u32, super::InnerDraw>
{
    /// Read the registry of this same compiler without granting compiler
    /// replacement, extraction or closure authority.
    pub fn inner_registry(&self) -> Result<&G, jolt_transcript::TranscriptError> {
        let (registry, _, _, _) = self.transcript.admission()?;
        Ok(registry)
    }

    /// Scope a checked Jolt frontend loan over this exact private compiler.
    /// The callback returns `(value, complete_canonical_native_instance)`. The
    /// helper itself checks acceptance and emits the handoff before native resumes.
    /// Errors never pass through the native trait's `InvalidProof` conversion.
    ///
    /// ```compile_fail
    /// use akita_transcript::{native::{inner, E}, native_strong::{NativeTranscriptBorrow, InnerDraw, InnerExecutionRegistry}};
    /// use jolt_transcript::d1::frontend::{InnerBinding, Limits};
    /// use jolt_transcript::TranscriptError;
    /// fn replace<G: InnerExecutionRegistry, B: InnerBinding<G>>(
    ///     loan: NativeTranscriptBorrow<'_, inner::Transcript<G>, u32, InnerDraw>,
    ///     binding: B, limits: Limits, replacement: inner::Transcript<G>,
    /// ) -> Result<(), TranscriptError> {
    ///     loan.with_frontend::<E, _, _, _, _>(binding, limits, |frontend| {
    ///         core::mem::replace(frontend, replacement);
    ///         Ok(((), b"instance"))
    ///     })
    /// }
    /// ```
    /// ```compile_fail
    /// use jolt_transcript::d1::{frontend::{BorrowedInner, InnerBinding}, inner};
    /// fn close<G: inner::Registry, B: InnerBinding<G>>(t: &mut BorrowedInner<'_, G, B>) {
    ///     t.finish().unwrap();
    /// }
    /// ```
    /// ```compile_fail
    /// use jolt_transcript::d1::{frontend::{BorrowedInner, InnerBinding}, inner};
    /// fn extract<G: inner::Registry, B: InnerBinding<G>>(t: BorrowedInner<'_, G, B>) {
    ///     t.into_native(b"instance").unwrap();
    /// }
    /// ```
    pub fn with_frontend<C, B, V, H, X>(
        self,
        binding: B,
        limits: jolt_transcript::d1::frontend::Limits,
        run: impl FnOnce(
            &mut jolt_transcript::d1::frontend::BorrowedInner<'a, G, B, C>,
        ) -> Result<(V, H), X>,
    ) -> Result<V, X>
    where
        C: jolt_transcript::d1::ChallengeField,
        B: jolt_transcript::d1::frontend::InnerBinding<G>,
        H: AsRef<[u8]>,
        X: From<jolt_transcript::TranscriptError>,
    {
        jolt_transcript::d1::frontend::BorrowedInner::<G, B, C>::with_frontend(
            self.transcript,
            binding,
            limits,
            run,
        )
    }
}

impl<'a, 's, G: super::OuterExecutionRegistry>
    NativeTranscriptBorrow<
        'a,
        crate::native::outer::Transcript<G>,
        super::OuterRecord<'s>,
        crate::native::outer::Stage<'s>,
    >
{
    /// Full-C outer counterpart of the checked inner frontend loan. The callback
    /// receives no raw replacement/extraction/closure authority; complete handoff
    /// and typed-error poisoning are enforced by the scoped helper.
    pub fn with_frontend<C, B, V, H, X>(
        self,
        binding: B,
        limits: jolt_transcript::d1::frontend::Limits,
        run: impl FnOnce(
            &mut jolt_transcript::d1::frontend::BorrowedOuter<'a, G, B, C>,
        ) -> Result<(V, H), X>,
    ) -> Result<V, X>
    where
        C: jolt_transcript::d1::ChallengeField,
        B: jolt_transcript::d1::frontend::OuterBinding<G>,
        H: AsRef<[u8]>,
        X: From<jolt_transcript::TranscriptError>,
    {
        jolt_transcript::d1::frontend::BorrowedOuter::<G, B, C>::with_frontend(
            self.transcript,
            binding,
            limits,
            run,
        )
    }
}
