//! Selected strong-check event transport over the SAME live D1 compiler.
//!
//! The anchor is the complete ordered message/challenge history. Keys are
//! explicit installation inputs: this module does not emit or certify a registry.
//! Inner result records use `LE64(site) || u8(operation) || LE64(count) ||
//! E[count]`, where E is two canonical little-endian K atoms. The installed
//! registry must admit that exact codec and each structural occurrence. One
//! snapshot supplies the entire sequential inner vector; the outer compiler
//! retains its distinct atomic tape, reserved spans and complete result grammar.
use crate::native::{self, decode_e, encode_e, inner, outer, Error, E};
use akita_error::AkitaError;
mod borrow;
pub use borrow::NativeTranscriptBorrow;
#[cfg(test)]
#[path = "native_strong/outer_tests.rs"]
mod outer_tests;

pub use outer::Operation;

#[derive(Clone, Copy)]
pub struct OuterRecord<'a> {
    pub site: outer::Site<'a>,
    pub codec: &'a str,
}
#[derive(Clone, Copy)]
pub struct InnerDraw {
    pub site: u64,
    pub result_kind: u32,
}

pub struct InnerShellDraw {
    pub streams: Vec<u64>,
    pub result_kind: u32,
}

/// One logical native request, including zero-length vectors and shell bundles.
/// Metadata is not another wire record: the existing result codecs stay intact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InnerNativeRequest<'a> {
    Public {
        kind: u32,
        count: usize,
    },
    Proof {
        kind: u32,
        count: usize,
    },
    Draw {
        site: u64,
        result_kind: u32,
        operation: Operation,
        count: usize,
    },
    ShellBundle {
        streams: &'a [u64],
        result_kind: u32,
        count: usize,
    },
}

/// Required execution capability for the sealed inner native boundary.
///
/// The primitive [`inner::Registry`] remains pure schema/site admission:
/// context and record checks may precede a compiler limit failure, and site
/// lookups may repeat for every bit cursor read. None advances execution.
/// Implementations retain their finite installed protocol state using interior
/// mutability; only successful `published` callbacks advance the native ledger.
/// These hooks do not themselves supply a production accumulation registry.
pub trait InnerExecutionRegistry: inner::Registry {
    /// Check the complete next logical request without advancing execution.
    /// Called before any absorption or sampling, including for count zero.
    fn preflight(&self, request: InnerNativeRequest<'_>) -> Result<(), Error>;
    /// Validate and commit the request after its entire canonical payload was
    /// appended successfully to the SAME compiler. `payload` is the original
    /// public/proof payload or complete encoded draw/shell result, not a copy.
    /// Check its codec and provenance against `request` and the installed state.
    /// An error permanently poisons that compiler; publication is not rolled back.
    fn published(&self, request: InnerNativeRequest<'_>, payload: &[u8]) -> Result<(), Error>;
    /// Require the finite installed protocol's accepting state: every required
    /// request/publication and terminal obligation is complete. Raw compiler
    /// liveness or exhaustion of a convenient prefix is not acceptance.
    fn accepting(&self) -> Result<(), Error>;
}

/// Required completion capability for the sealed outer native boundary.
///
/// [`outer::Registry::record`] already receives the complete canonical record
/// and its compiler-owned ordinal; [`outer::Registry::request`] admits the
/// complete request and sampler limits. Both remain admission, not execution:
/// schema prebinding and repeated request checks do not publish events.
///
/// The compiler's actual completed count and borrowed canonical prefix supply
/// publication evidence at closure, including frontend records and handoffs.
/// No second request transport, result copy, sampler or publication ledger is
/// needed. These hooks do not supply a production accumulation registry.
///
/// Raw admission alone grants neither native closure nor a scoped native loan.
/// ```compile_fail
/// use akita_transcript::{native::outer, native_strong::NativeStrongTranscript};
/// fn close<G: outer::Registry>(compiler: &mut outer::Transcript<G>) {
///     NativeStrongTranscript::finish(compiler).unwrap();
/// }
/// ```
/// ```compile_fail
/// use akita_transcript::{native::outer, native_strong::NativeTranscriptBorrow};
/// fn lend<G: outer::Registry>(compiler: &mut outer::Transcript<G>) {
///     let _ = NativeTranscriptBorrow::new(compiler);
/// }
/// ```
pub trait OuterExecutionRegistry: outer::Registry {
    /// Require the entire installed finite protocol and terminal obligations,
    /// not just liveness or exhaustion of a convenient component prefix.
    /// `completed_events` and `prefix` come from the SAME active compiler.
    /// Admission callbacks must not be counted as successful publication:
    /// an admitted record can still fail the compiler's append limits.
    fn accepting(&self, completed_events: u64, prefix: &[u8]) -> Result<(), Error>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeCompilerFamily {
    InnerH1610,
    OuterShake256,
}

mod sealed {
    pub trait Sealed {}
    impl<G: super::OuterExecutionRegistry> Sealed for super::outer::Transcript<G> {}
    impl<G: super::InnerExecutionRegistry> Sealed for super::inner::Transcript<G> {}
}

/// The selected concrete compiler, not a caller-supplied transcript adapter.
/// External failures poison the same live compiler.
pub trait NativeStrongTranscript<R, D>: sealed::Sealed {
    const FAMILY: NativeCompilerFamily;
    fn public(&mut self, site: &R, count: usize, bytes: &[u8]) -> Result<(), AkitaError>;
    fn proof(&mut self, site: &R, count: usize, bytes: &[u8]) -> Result<(), AkitaError>;
    type ShellSite;
    fn shell_bundle(
        &mut self,
        site: &Self::ShellSite,
        count: usize,
    ) -> Result<Vec<native::shell::Shell>, AkitaError>;
    fn scalar(&mut self, site: &D, mode: Operation) -> Result<E, AkitaError>;
    fn vector(&mut self, site: &D, mode: Operation, count: usize) -> Result<Vec<E>, AkitaError>;
    fn reject(&mut self);
    /// Require registered native acceptance before closing the raw compiler.
    /// The run owner must also discharge terminal obligations and exact
    /// proof-reader exhaustion; raw compiler `finish` alone does not
    /// certify that either native protocol reached its accepting state.
    fn finish(&mut self) -> Result<(), AkitaError>;
}
fn failure<T>(result: Result<T, Error>) -> Result<T, AkitaError> {
    result.map_err(|_| AkitaError::InvalidProof)
}
impl<'a, G: OuterExecutionRegistry> NativeStrongTranscript<OuterRecord<'a>, outer::Stage<'a>>
    for outer::Transcript<G>
{
    const FAMILY: NativeCompilerFamily = NativeCompilerFamily::OuterShake256;
    fn public(
        &mut self,
        site: &OuterRecord<'a>,
        count: usize,
        bytes: &[u8],
    ) -> Result<(), AkitaError> {
        failure(outer::Transcript::public(
            self,
            site.site,
            site.codec,
            count as u64,
            bytes,
        ))
    }
    type ShellSite = outer::Stage<'a>;
    fn shell_bundle(
        &mut self,
        site: &Self::ShellSite,
        count: usize,
    ) -> Result<Vec<native::shell::Shell>, AkitaError> {
        failure(outer::Transcript::shell_bundle(self, site, count as u64))
    }
    fn proof(
        &mut self,
        site: &OuterRecord<'a>,
        count: usize,
        bytes: &[u8],
    ) -> Result<(), AkitaError> {
        failure(outer::Transcript::proof(
            self,
            site.site,
            site.codec,
            count as u64,
            bytes,
        ))
    }
    fn scalar(&mut self, site: &outer::Stage<'a>, mode: Operation) -> Result<E, AkitaError> {
        failure(outer::Transcript::scalar(self, site, mode))
    }
    fn vector(
        &mut self,
        site: &outer::Stage<'a>,
        mode: Operation,
        count: usize,
    ) -> Result<Vec<E>, AkitaError> {
        failure(outer::Transcript::vector(self, site, mode, count as u64))
    }
    fn reject(&mut self) {
        outer::Transcript::reject(self, Error::NonCanonical);
    }
    fn finish(&mut self) -> Result<(), AkitaError> {
        let result = (|| {
            self.admission()?
                .0
                .accepting(self.completed_events(), self.prefix_bytes())?;
            outer::Transcript::finish(self)
        })();
        failure(result.map_err(|error| self.reject(error)))
    }
}
fn inner_result<G: InnerExecutionRegistry>(
    transcript: &mut inner::Transcript<G>,
    site: &InnerDraw,
    mode: Operation,
    values: &[E],
) -> Result<(), Error> {
    let len = values
        .len()
        .checked_mul(32)
        .and_then(|n| n.checked_add(17))
        .ok_or(Error::Overflow)?;
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(len).map_err(|_| Error::Limit)?;
    bytes.extend_from_slice(&site.site.to_le_bytes());
    bytes.push(mode as u8);
    bytes.extend_from_slice(&(values.len() as u64).to_le_bytes());
    for value in values {
        bytes.extend_from_slice(&encode_e(value));
    }
    transcript.append(site.result_kind, &bytes)?;
    transcript.admission()?.0.published(
        InnerNativeRequest::Draw {
            site: site.site,
            result_kind: site.result_kind,
            operation: mode,
            count: values.len(),
        },
        &bytes,
    )
}
impl<G: InnerExecutionRegistry> NativeStrongTranscript<u32, InnerDraw> for inner::Transcript<G> {
    const FAMILY: NativeCompilerFamily = NativeCompilerFamily::InnerH1610;
    type ShellSite = InnerShellDraw;
    fn shell_bundle(
        &mut self,
        site: &InnerShellDraw,
        count: usize,
    ) -> Result<Vec<native::shell::Shell>, AkitaError> {
        let result = (|| {
            let request = InnerNativeRequest::ShellBundle {
                streams: &site.streams,
                result_kind: site.result_kind,
                count,
            };
            self.admission()?.0.preflight(request)?;
            if count != site.streams.len() {
                return Err(Error::Registry);
            }
            for (index, stream) in site.streams.iter().enumerate() {
                if site.streams[..index].contains(stream) {
                    return Err(Error::Registry);
                }
            }
            let shells = self.sample(|snapshot| {
                let mut shells = Vec::new();
                shells.try_reserve_exact(count).map_err(|_| Error::Limit)?;
                for &stream in &site.streams {
                    shells.push(native::inner_shell(snapshot, stream)?);
                }
                Ok(shells)
            })?;
            // Complete coordinate keys and all signed coefficient bytes enter
            // history together; no partial bundle is published.
            let len = count
                .checked_mul(136)
                .and_then(|n| n.checked_add(8))
                .ok_or(Error::Overflow)?;
            let mut bytes = Vec::new();
            bytes.try_reserve_exact(len).map_err(|_| Error::Limit)?;
            bytes.extend_from_slice(&(count as u64).to_le_bytes());
            for (&stream, shell) in site.streams.iter().zip(&shells) {
                bytes.extend_from_slice(&stream.to_le_bytes());
                for &coefficient in shell.coefficients() {
                    bytes.push(coefficient as u8);
                }
            }
            self.append(site.result_kind, &bytes)?;
            self.admission()?.0.published(request, &bytes)?;
            Ok(shells)
        })();
        failure(result.map_err(|error| self.reject(error)))
    }
    fn public(&mut self, site: &u32, count: usize, bytes: &[u8]) -> Result<(), AkitaError> {
        let result = (|| {
            let request = InnerNativeRequest::Public { kind: *site, count };
            self.admission()?.0.preflight(request)?;
            self.append(*site, bytes)?;
            self.admission()?.0.published(request, bytes)
        })();
        failure(result.map_err(|error| self.reject(error)))
    }
    fn proof(&mut self, site: &u32, count: usize, bytes: &[u8]) -> Result<(), AkitaError> {
        let result = (|| {
            let request = InnerNativeRequest::Proof { kind: *site, count };
            self.admission()?.0.preflight(request)?;
            self.append(*site, bytes)?;
            self.admission()?.0.published(request, bytes)
        })();
        failure(result.map_err(|error| self.reject(error)))
    }
    fn scalar(&mut self, site: &InnerDraw, mode: Operation) -> Result<E, AkitaError> {
        let result = (|| {
            if !matches!(mode, Operation::RawScalar | Operation::ScalarCoefficient) {
                return Err(Error::Registry);
            }
            self.admission()?.0.preflight(InnerNativeRequest::Draw {
                site: site.site,
                result_kind: site.result_kind,
                operation: mode,
                count: 1,
            })?;
            let value = self.sample(|snapshot| native::inner_scalar(snapshot, site.site))?;
            inner_result(self, site, mode, &[value])?;
            Ok(value)
        })();
        failure(result.map_err(|error| self.reject(error)))
    }
    fn vector(
        &mut self,
        site: &InnerDraw,
        mode: Operation,
        count: usize,
    ) -> Result<Vec<E>, AkitaError> {
        let result = (|| {
            if !matches!(mode, Operation::RawVector | Operation::CoefficientVector) {
                return Err(Error::Registry);
            }
            self.admission()?.0.preflight(InnerNativeRequest::Draw {
                site: site.site,
                result_kind: site.result_kind,
                operation: mode,
                count,
            })?;
            let values = self.sample(|snapshot| {
                let mut values = Vec::new();
                values.try_reserve_exact(count).map_err(|_| Error::Limit)?;
                for _ in 0..count {
                    values.push(native::inner_scalar(snapshot, site.site)?);
                }
                Ok(values)
            })?;
            inner_result(self, site, mode, &values)?;
            Ok(values)
        })();
        failure(result.map_err(|error| self.reject(error)))
    }
    fn reject(&mut self) {
        inner::Transcript::reject(self, Error::NonCanonical);
    }
    fn finish(&mut self) -> Result<(), AkitaError> {
        let result = (|| {
            self.admission()?.0.accepting()?;
            inner::Transcript::finish(self)
        })();
        failure(result.map_err(|error| self.reject(error)))
    }
}

pub struct RoundSite<R, D> {
    pub coefficients: R,
    pub challenge: D,
}
/// Explicit installed terminal occurrences; the shell key uses the compiler's
/// own associated site type (an inner stream bundle or an outer stage).
pub struct TerminalSites<R, S> {
    pub descriptor: R,
    pub a_images: R,
    pub point: R,
    pub value: R,
    pub partials: R,
    pub response: R,
    pub shell: S,
}
/// Exact installed maps, including a separate occurrence for EVERY round.
pub struct StrongSites<R, D> {
    pub binding: R,
    pub tau0: D,
    pub rho: D,
    pub first: Vec<RoundSite<R, D>>,
    pub quarters: R,
    pub xi: D,
    pub second: Vec<RoundSite<R, D>>,
    pub virtual_value: R,
    pub mix: D,
    pub fused: Vec<RoundSite<R, D>>,
    pub value: R,
}
/// Inner final receiver's standalone range/binary/zero reduction. There is no
/// coefficient relation and deliberately no rho site or implicit linear term.
pub struct FinalReceiverDomainSites<R, D> {
    pub binding: R,
    pub tau0: D,
    pub first: Vec<RoundSite<R, D>>,
    pub quarters: R,
    pub xi: D,
    pub second: Vec<RoundSite<R, D>>,
    pub virtual_value: R,
    pub mix: D,
    pub fused: Vec<RoundSite<R, D>>,
    pub value: R,
}
/// Positive Boolean retains its direct predicate and unconditional zero draw.
pub struct BooleanSourceDomainSites<R, D> {
    pub binding: R,
    pub anchor: D,
    pub eta_zero: D,
    pub rounds: Vec<RoundSite<R, D>>,
    pub value: R,
}
/// Balanced fresh sources use two degree-five stages and one degree-three
/// raw realization. The optional site is determined by authenticated support.
pub struct BalancedSourceDomainSites<R, D> {
    pub binding: R,
    pub tau0: D,
    pub first: Vec<RoundSite<R, D>>,
    pub quarters: R,
    pub xi: D,
    pub second: Vec<RoundSite<R, D>>,
    pub virtual_value: R,
    pub eta_zero: Option<D>,
    pub realization: Vec<RoundSite<R, D>>,
    pub value: R,
}
pub enum SourceDomainSites<R, D> {
    Boolean(BooleanSourceDomainSites<R, D>),
    Balanced(BalancedSourceDomainSites<R, D>),
}
impl<R, D> SourceDomainSites<R, D> {
    /// Validate the entire installed grammar before any public event or draw.
    pub fn validate(
        &self,
        rounds: usize,
        balanced: bool,
        physical_zeros: bool,
    ) -> Result<(), AkitaError> {
        let valid = match self {
            Self::Boolean(sites) => !balanced && sites.rounds.len() == rounds,
            Self::Balanced(sites) => {
                balanced
                    && sites.first.len() == rounds
                    && sites.second.len() == rounds
                    && sites.realization.len() == rounds
                    && sites.eta_zero.is_some() == physical_zeros
            }
        };
        if !valid || rounds >= usize::BITS as usize {
            return Err(AkitaError::InvalidProof);
        }
        Ok(())
    }
}
pub struct NormalizationSites<R, D> {
    pub binding: R,
    pub claims: R,
    pub alpha: D,
    pub rounds: Vec<RoundSite<R, D>>,
    pub value: R,
}
/// All errors, including algebra and external source-reader errors, reject the
/// original transcript. A failed reduction cannot be retried on that session.
pub fn guarded<T, R, D, V>(
    transcript: &mut T,
    run: impl FnOnce(&mut T) -> Result<V, AkitaError>,
) -> Result<V, AkitaError>
where
    T: NativeStrongTranscript<R, D>,
{
    let result = run(transcript);
    if result.is_err() {
        transcript.reject();
    }
    result
}
/// Full coefficients in increasing degree order, with no omitted constant or
/// trailing zeros. Stack storage covers the largest selected degree (five).
pub fn send_values<T, R, D>(
    transcript: &mut T,
    site: &R,
    values: &[E],
    proof: &mut Vec<u8>,
) -> Result<(), AkitaError>
where
    T: NativeStrongTranscript<R, D>,
{
    guarded(transcript, |transcript| {
        if values.len() > 6 {
            return Err(AkitaError::InvalidProof);
        }
        let mut bytes = [0; 6 * 32];
        let bytes = &mut bytes[..values.len() * 32];
        for (value, atom) in values.iter().zip(bytes.chunks_exact_mut(32)) {
            atom.copy_from_slice(&encode_e(value));
        }
        transcript.proof(site, values.len(), bytes)?;
        proof
            .try_reserve(bytes.len())
            .map_err(|_| AkitaError::InvalidProof)?;
        proof.extend_from_slice(bytes);
        Ok(())
    })
}
pub fn receive_values<T, R, D>(
    transcript: &mut T,
    site: &R,
    values: &mut [E],
    proof: &mut &[u8],
) -> Result<(), AkitaError>
where
    T: NativeStrongTranscript<R, D>,
{
    guarded(transcript, |transcript| {
        let len = values
            .len()
            .checked_mul(32)
            .ok_or(AkitaError::InvalidProof)?;
        let bytes = proof.get(..len).ok_or(AkitaError::InvalidProof)?;
        for (value, atom) in values.iter_mut().zip(bytes.chunks_exact(32)) {
            *value = failure(decode_e(atom))?;
        }
        transcript.proof(site, values.len(), bytes)?;
        *proof = proof.get(len..).ok_or(AkitaError::InvalidProof)?;
        Ok(())
    })
}

/// Bind every ordered normalization claim, including ordinary/raw distinction,
/// complete points and evaluations. Decoder identities, permutations and affine
/// constants belong to the separately registered binding header, in this order.
/// Codec: LE64(count), then (u8(raw), LE64(point_len), E[point_len], E(value)).
pub fn bind_normalization_claims<'a, T, R, D, I>(
    transcript: &mut T,
    site: &R,
    claims: I,
) -> Result<(), AkitaError>
where
    T: NativeStrongTranscript<R, D>,
    I: ExactSizeIterator<Item = (bool, &'a [E], E)>,
{
    guarded(transcript, |transcript| {
        let count = claims.len();
        let mut bytes = Vec::new();
        bytes.try_reserve(8).map_err(|_| AkitaError::InvalidProof)?;
        bytes.extend_from_slice(&(count as u64).to_le_bytes());
        for (raw, point, value) in claims {
            let length = point
                .len()
                .checked_mul(32)
                .and_then(|n| n.checked_add(41))
                .ok_or(AkitaError::InvalidProof)?;
            bytes
                .try_reserve(length)
                .map_err(|_| AkitaError::InvalidProof)?;
            bytes.push(u8::from(raw));
            bytes.extend_from_slice(&(point.len() as u64).to_le_bytes());
            for coordinate in point {
                bytes.extend_from_slice(&encode_e(coordinate));
            }
            bytes.extend_from_slice(&encode_e(&value));
        }
        transcript.public(site, count, &bytes)
    })
}
