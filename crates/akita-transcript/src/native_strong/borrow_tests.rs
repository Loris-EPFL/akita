use super::*;
use crate::native::{inner, Error};
use crate::native_strong::{InnerDraw, InnerExecutionRegistry, InnerNativeRequest, InnerShellDraw};
use core::cell::Cell;
use std::rc::Rc;

const PUBLIC_REQUEST: InnerNativeRequest<'static> =
    InnerNativeRequest::Public { kind: 2, count: 1 };
const PROOF_REQUEST: InnerNativeRequest<'static> = InnerNativeRequest::Proof { kind: 3, count: 1 };
const SCALAR_REQUEST: InnerNativeRequest<'static> = InnerNativeRequest::Draw {
    site: 7,
    result_kind: 4,
    operation: Operation::RawScalar,
    count: 1,
};

/// Only a finite component-test ledger, not an accumulation-run certificate.
struct Registry {
    expected: Vec<InnerNativeRequest<'static>>,
    completed: Rc<Cell<usize>>,
    reject_publication: Cell<bool>,
}
impl inner::Registry for Registry {
    fn context(&self, kind: u32, bytes: &[u8]) -> Result<(), Error> {
        if kind == 1 && bytes == b"borrow-test" {
            Ok(())
        } else {
            Err(Error::Registry)
        }
    }
    fn record(&self, kind: u32, bytes: &[u8]) -> Result<(), Error> {
        if kind == 1 {
            return self.context(kind, bytes);
        }
        if (kind == 2 && bytes == b"public") || (kind == 3 && bytes == b"proof") {
            return Ok(());
        }
        if kind == 5 && bytes == b"full-native-instance" {
            return Ok(());
        }
        if kind == 4 && bytes.len() >= 17 {
            let count = u64::from_le_bytes(bytes[9..17].try_into().unwrap()) as usize;
            if bytes[..8] != 7u64.to_le_bytes()
                || ![
                    Operation::RawScalar as u8,
                    Operation::ScalarCoefficient as u8,
                    Operation::RawVector as u8,
                    Operation::CoefficientVector as u8,
                ]
                .contains(&bytes[8])
                || count.checked_mul(32).and_then(|n| n.checked_add(17)) != Some(bytes.len())
            {
                return Err(Error::Registry);
            }
            for atom in bytes[17..].chunks_exact(32) {
                crate::native::decode_e(atom)?;
            }
            return Ok(());
        }
        if kind == 6 && bytes.len() >= 8 {
            let count = u64::from_le_bytes(bytes[..8].try_into().unwrap()) as usize;
            if count.checked_mul(136).and_then(|n| n.checked_add(8)) != Some(bytes.len()) {
                return Err(Error::Registry);
            }
            for shell in bytes[8..].chunks_exact(136) {
                let stream = u64::from_le_bytes(shell[..8].try_into().unwrap());
                self.site(stream)?;
                let coefficients: [u8; 128] = shell[8..].try_into().unwrap();
                crate::native::shell::Shell::new(coefficients.map(|x| x as i8))?;
            }
            return Ok(());
        }
        Err(Error::Registry)
    }
    fn site(&self, site: u64) -> Result<(), Error> {
        if site == 7 || site == 8 {
            Ok(())
        } else {
            Err(Error::Registry)
        }
    }
}
impl InnerExecutionRegistry for Registry {
    fn preflight(&self, request: InnerNativeRequest<'_>) -> Result<(), Error> {
        if self.expected.get(self.completed.get()) == Some(&request) {
            Ok(())
        } else {
            Err(Error::Registry)
        }
    }
    fn published(&self, request: InnerNativeRequest<'_>, bytes: &[u8]) -> Result<(), Error> {
        self.preflight(request)?;
        if self.reject_publication.get() {
            return Err(Error::Registry);
        }
        match request {
            InnerNativeRequest::Public { kind, .. } | InnerNativeRequest::Proof { kind, .. } => {
                inner::Registry::record(self, kind, bytes)?;
            }
            InnerNativeRequest::Draw {
                site,
                result_kind,
                operation,
                count,
            } => {
                inner::Registry::record(self, result_kind, bytes)?;
                if bytes.get(..8) != Some(site.to_le_bytes().as_slice())
                    || bytes.get(8) != Some(&(operation as u8))
                    || bytes.get(9..17) != Some((count as u64).to_le_bytes().as_slice())
                {
                    return Err(Error::Registry);
                }
            }
            InnerNativeRequest::ShellBundle {
                streams,
                result_kind,
                count,
            } => {
                inner::Registry::record(self, result_kind, bytes)?;
                if streams.len() != count
                    || bytes.get(..8) != Some((count as u64).to_le_bytes().as_slice())
                {
                    return Err(Error::Registry);
                }
                for (record, stream) in bytes[8..].chunks_exact(136).zip(streams) {
                    if record[..8] != stream.to_le_bytes() {
                        return Err(Error::Registry);
                    }
                }
            }
        }
        self.completed.set(self.completed.get() + 1);
        Ok(())
    }
    fn accepting(&self) -> Result<(), Error> {
        if self.completed.get() == self.expected.len() {
            Ok(())
        } else {
            Err(Error::Registry)
        }
    }
}
fn compiler() -> inner::Transcript<Registry> {
    compiler_with(&[
        PUBLIC_REQUEST,
        PROOF_REQUEST,
        SCALAR_REQUEST,
        SCALAR_REQUEST,
    ])
}
fn compiler_with(expected: &[InnerNativeRequest<'static>]) -> inner::Transcript<Registry> {
    inner::Transcript::new(
        Registry {
            expected: expected.to_vec(),
            completed: Rc::new(Cell::new(0)),
            reject_publication: Cell::new(false),
        },
        inner::Limits {
            transcript_bytes: 4096,
            record_bytes: 1024,
            records: 8,
            sites: 2,
            blocks_per_site: 100,
            component_calls: 100,
            read_bits: 100_000,
        },
        1,
        b"borrow-test",
    )
    .unwrap()
}
#[test]
fn borrowed_frontend_and_owner_share_the_real_compiler_history() {
    let mut retained = compiler();
    let mut direct = compiler();
    let draw = InnerDraw {
        site: 7,
        result_kind: 4,
    };
    let borrowed_value = {
        let mut frontend = NativeTranscriptBorrow::new(&mut retained);
        frontend.public(&2, 1, b"public").unwrap();
        frontend.reborrow().proof(&3, 1, b"proof").unwrap();
        frontend.scalar(&draw, Operation::RawScalar).unwrap()
    };
    NativeStrongTranscript::public(&mut direct, &2, 1, b"public").unwrap();
    NativeStrongTranscript::proof(&mut direct, &3, 1, b"proof").unwrap();
    assert_eq!(
        borrowed_value,
        NativeStrongTranscript::scalar(&mut direct, &draw, Operation::RawScalar).unwrap()
    );
    assert_eq!(
        NativeStrongTranscript::scalar(&mut retained, &draw, Operation::RawScalar).unwrap(),
        NativeStrongTranscript::scalar(&mut direct, &draw, Operation::RawScalar).unwrap()
    );
    NativeStrongTranscript::finish(&mut retained).unwrap();
    NativeStrongTranscript::finish(&mut direct).unwrap();
}
#[test]
fn borrowing_failure_poison_is_retained_by_the_owner() {
    let mut retained = compiler();
    {
        let mut frontend = NativeTranscriptBorrow::new(&mut retained);
        assert!(frontend.public(&2, 1, b"wrong").is_err());
    }
    assert!(NativeStrongTranscript::public(&mut retained, &2, 1, b"public").is_err());
    assert!(NativeStrongTranscript::finish(&mut retained).is_err());
}

#[test]
fn local_frontend_rejection_poison_is_retained_without_emission() {
    let mut retained = compiler();
    NativeTranscriptBorrow::new(&mut retained).reject();
    assert!(NativeStrongTranscript::public(&mut retained, &2, 1, b"public").is_err());
    assert!(NativeStrongTranscript::finish(&mut retained).is_err());
}

use jolt_transcript::d1::frontend::{self, Boundary, FrontendBinding, InnerBinding};
use jolt_transcript::structure::{Component, Site, Stage, TreeMode};
use jolt_transcript::{
    CodecKind, DrawSite, EmissionBinding, InstalledCodec, MessageDefinition, MessageKind,
    MessageSite, ScopeEntry, Transcript as FrontendTranscript, TranscriptError,
};

const PUBLIC: Site<'static> = Site(&[Component::Role("public")]);
const DRAW: Stage<'static> = Stage {
    site: Site(&[Component::Role("draw")]),
    mode: TreeMode::FlatBundle,
    arities: &[],
    family: b"borrow-test-E",
    sampler: b"native-sequential",
    carries: &[],
    result_type: "E-result/v1",
};
struct Binding {
    messages: usize,
    draws: usize,
}
impl EmissionBinding for Binding {
    fn message(&self, d: &MessageDefinition<'_>) -> Result<(), TranscriptError> {
        if !matches!(d.occurrence.0, [Component::Role("public")])
            || d.kind != MessageKind::Public
            || d.codec.name != "public/v1"
            || d.codec.kind != CodecKind::Bytes
            || d.count != 6
        {
            return Err(TranscriptError::Descriptor);
        }
        Ok(())
    }
    fn draw(
        &self,
        stage: &Stage<'_>,
        operation: Operation,
        count: u64,
        powers: u64,
    ) -> Result<(), TranscriptError> {
        if !matches!(stage.site.0, [Component::Role("draw")])
            || stage.mode != DRAW.mode
            || !stage.arities.is_empty()
            || !stage.carries.is_empty()
            || stage.family != DRAW.family
            || stage.sampler != DRAW.sampler
            || stage.result_type != DRAW.result_type
            || operation != Operation::RawScalar
            || count != 1
            || powers != 0
        {
            return Err(TranscriptError::Descriptor);
        }
        Ok(())
    }
    fn compound(&self, _: Site<'_>, _: &[ScopeEntry<'_>]) -> Result<(), TranscriptError> {
        Err(TranscriptError::Compound)
    }
}
impl FrontendBinding for Binding {
    fn message_complete(&mut self, _: &MessageSite<'_>) -> Result<(), TranscriptError> {
        self.messages += 1;
        Ok(())
    }
    fn draw_complete(&mut self, _: &DrawSite<'_>) -> Result<(), TranscriptError> {
        self.draws += 1;
        Ok(())
    }
}
impl InnerBinding<Registry> for Binding {
    fn installation(
        &self,
        registry: &Registry,
        limits: &inner::Limits,
        kind: u32,
        context: &[u8],
        frontend: &frontend::Limits,
    ) -> Result<(), TranscriptError> {
        inner::Registry::context(registry, kind, context)?;
        if limits.record_bytes != 1024
            || frontend.payload_bytes != 1024
            || frontend.coordinates != 1
            || frontend.powers != 1
        {
            return Err(TranscriptError::Limit);
        }
        Ok(())
    }
    fn message_kind(&self, d: &MessageDefinition<'_>) -> Result<u32, TranscriptError> {
        self.message(d)?;
        Ok(2)
    }
    fn native_draw(&self, site: &DrawSite<'_>) -> Result<frontend::InnerDraw, TranscriptError> {
        self.draw(
            site.stage(),
            site.operation(),
            site.count(),
            site.powers_limit(),
        )?;
        Ok(frontend::InnerDraw {
            stream: 7,
            result_kind: 4,
        })
    }
    fn accepting(
        &self,
        _: &inner::Transcript<Registry>,
        _: Boundary,
    ) -> Result<(), TranscriptError> {
        if self.messages != 1 || self.draws != 1 {
            return Err(TranscriptError::Boundary);
        }
        Ok(())
    }
    fn handoff_kind(&self, instance: &[u8]) -> Result<u32, TranscriptError> {
        if instance != b"full-native-instance" {
            return Err(TranscriptError::Boundary);
        }
        Ok(5)
    }
}
fn frontend_limits() -> frontend::Limits {
    frontend::Limits {
        payload_bytes: 1024,
        coordinates: 1,
        powers: 1,
    }
}
fn frontend_sites(binding: &Binding) -> (MessageSite<'static>, DrawSite<'static>) {
    (
        MessageSite::bind(
            binding,
            MessageDefinition {
                occurrence: PUBLIC,
                kind: MessageKind::Public,
                codec: InstalledCodec {
                    name: "public/v1",
                    kind: CodecKind::Bytes,
                },
                count: 6,
            },
        )
        .unwrap(),
        DrawSite::bind(binding, &DRAW, Operation::RawScalar, 1, 0).unwrap(),
    )
}

#[test]
fn universal_frontend_loan_then_native_matches_uninterrupted_execution() {
    // The frontend binding accounts for its own public/draw/handoff interval;
    // native ledgers account only for the native requests each path exercises.
    let mut retained = compiler_with(&[PROOF_REQUEST, SCALAR_REQUEST]);
    let mut direct = compiler_with(&[
        PROOF_REQUEST,
        PUBLIC_REQUEST,
        SCALAR_REQUEST,
        SCALAR_REQUEST,
    ]);
    let binding = Binding {
        messages: 0,
        draws: 0,
    };
    let (public, draw) = frontend_sites(&binding);
    let native_draw = InnerDraw {
        site: 7,
        result_kind: 4,
    };
    NativeStrongTranscript::proof(&mut retained, &3, 1, b"proof").unwrap();
    NativeStrongTranscript::proof(&mut direct, &3, 1, b"proof").unwrap();
    let value = NativeTranscriptBorrow::new(&mut retained)
        .with_frontend::<E, _, _, _, _>(
            binding,
            frontend_limits(),
            |frontend| -> Result<_, TranscriptError> {
                frontend.append_bytes(&public, b"public")?;
                Ok((frontend.challenge(&draw)?, b"full-native-instance"))
            },
        )
        .unwrap();
    NativeStrongTranscript::public(&mut direct, &2, 1, b"public").unwrap();
    assert_eq!(
        value,
        NativeStrongTranscript::scalar(&mut direct, &native_draw, Operation::RawScalar,).unwrap()
    );
    direct.append(5, b"full-native-instance").unwrap();
    assert_eq!(
        NativeStrongTranscript::scalar(&mut retained, &native_draw, Operation::RawScalar).unwrap(),
        NativeStrongTranscript::scalar(&mut direct, &native_draw, Operation::RawScalar).unwrap(),
    );
    NativeStrongTranscript::finish(&mut retained).unwrap();
    NativeStrongTranscript::finish(&mut direct).unwrap();
}

#[derive(Debug, PartialEq)]
enum CallerError {
    Transcript(TranscriptError),
    Application,
}
impl From<TranscriptError> for CallerError {
    fn from(error: TranscriptError) -> Self {
        Self::Transcript(error)
    }
}

#[test]
fn universal_loan_preserves_typed_errors_and_rejects_ignored_failures() {
    for ignored in [false, true] {
        let mut retained = compiler();
        let binding = Binding {
            messages: 0,
            draws: 0,
        };
        let (site, _) = frontend_sites(&binding);
        let result = NativeTranscriptBorrow::new(&mut retained).with_frontend::<E, _, _, _, _>(
            binding,
            frontend_limits(),
            |frontend| -> Result<_, CallerError> {
                if ignored {
                    assert_eq!(
                        frontend.append_message(&site, jolt_transcript::MessageValue::U64(0)),
                        Err(TranscriptError::Codec),
                    );
                    Ok(((), b"full-native-instance"))
                } else {
                    Err(CallerError::Application)
                }
            },
        );
        assert_eq!(
            result,
            Err(if ignored {
                CallerError::Transcript(TranscriptError::D1(Error::Failed))
            } else {
                CallerError::Application
            })
        );
        assert_eq!(retained.finish(), Err(Error::Failed));
    }
}

#[test]
fn admission_rejects_actual_compiler_caps_before_callback() {
    let mut limits = *compiler().admission().unwrap().1;
    limits.record_bytes = 512;
    let registry = Registry {
        expected: vec![PUBLIC_REQUEST],
        completed: Rc::new(Cell::new(0)),
        reject_publication: Cell::new(false),
    };
    let mut retained = inner::Transcript::new(registry, limits, 1, b"borrow-test").unwrap();
    let binding = Binding {
        messages: 0,
        draws: 0,
    };
    let mut called = false;
    let result = NativeTranscriptBorrow::new(&mut retained).with_frontend::<E, _, _, _, _>(
        binding,
        frontend_limits(),
        |_| -> Result<_, TranscriptError> {
            called = true;
            Ok(((), b"full-native-instance"))
        },
    );
    assert_eq!(result, Err(TranscriptError::Limit));
    assert!(!called);
    assert_eq!(retained.finish(), Err(Error::Failed));
}

#[test]
fn native_request_preflight_rejects_wrong_kind_count_order_and_draw_provenance() {
    for (proof, count) in [(true, 1), (false, 0), (false, 2)] {
        let mut transcript = compiler_with(&[PUBLIC_REQUEST]);
        let before = transcript.len();
        let result = if proof {
            NativeStrongTranscript::proof(&mut transcript, &2, count, b"public")
        } else {
            NativeStrongTranscript::public(&mut transcript, &2, count, b"public")
        };
        assert!(result.is_err());
        assert_eq!(transcript.len(), before);
        assert!(transcript.finish().is_err());
    }
    let mut skipped = compiler();
    assert!(NativeStrongTranscript::scalar(
        &mut skipped,
        &InnerDraw {
            site: 7,
            result_kind: 4
        },
        Operation::RawScalar,
    )
    .is_err());
    for (site, result_kind, operation) in [
        (8, 4, Operation::RawScalar),
        (7, 6, Operation::RawScalar),
        (7, 4, Operation::ScalarCoefficient),
    ] {
        let mut transcript = compiler_with(&[SCALAR_REQUEST]);
        let before = transcript.len();
        assert!(NativeStrongTranscript::scalar(
            &mut transcript,
            &InnerDraw { site, result_kind },
            operation,
        )
        .is_err());
        assert_eq!(transcript.len(), before);
        assert!(transcript.finish().is_err());
    }
}

#[test]
fn native_zero_requests_are_published_in_order_and_cannot_be_skipped() {
    let zero_vector = InnerNativeRequest::Draw {
        site: 7,
        result_kind: 4,
        operation: Operation::RawVector,
        count: 0,
    };
    let zero_shell = InnerNativeRequest::ShellBundle {
        streams: &[],
        result_kind: 6,
        count: 0,
    };
    let draw = InnerDraw {
        site: 7,
        result_kind: 4,
    };
    let shells = InnerShellDraw {
        streams: vec![],
        result_kind: 6,
    };
    let mut transcript = compiler_with(&[zero_vector, zero_shell]);
    let completed = transcript.admission().unwrap().0.completed.clone();
    let before = transcript.len();
    assert!(
        NativeStrongTranscript::vector(&mut transcript, &draw, Operation::RawVector, 0,)
            .unwrap()
            .is_empty()
    );
    assert_eq!(transcript.len(), before + 12 + 17);
    assert_eq!(completed.get(), 1);
    assert!(
        NativeStrongTranscript::shell_bundle(&mut transcript, &shells, 0)
            .unwrap()
            .is_empty()
    );
    assert_eq!(transcript.len(), before + 12 + 17 + 12 + 8);
    assert_eq!(completed.get(), 2);
    NativeStrongTranscript::finish(&mut transcript).unwrap();

    let mut skipped = compiler_with(&[zero_vector, zero_shell]);
    assert!(NativeStrongTranscript::shell_bundle(&mut skipped, &shells, 0).is_err());
    assert!(skipped.finish().is_err());
    let mut wrong_count = compiler_with(&[zero_vector]);
    assert!(
        NativeStrongTranscript::vector(&mut wrong_count, &draw, Operation::RawVector, 1,).is_err()
    );
    assert!(wrong_count.finish().is_err());
}

#[test]
fn native_shell_preflight_checks_stream_order_result_kind_and_count() {
    let request = InnerNativeRequest::ShellBundle {
        streams: &[7, 8],
        result_kind: 6,
        count: 2,
    };
    for (streams, result_kind, count) in [
        (vec![8, 7], 6, 2),
        (vec![7, 8], 4, 2),
        (vec![7, 8], 6, 0),
        (vec![7, 7], 6, 2),
    ] {
        let mut transcript = compiler_with(&[request]);
        let before = transcript.len();
        assert!(NativeStrongTranscript::shell_bundle(
            &mut transcript,
            &InnerShellDraw {
                streams,
                result_kind
            },
            count,
        )
        .is_err());
        assert_eq!(transcript.len(), before);
        assert!(transcript.finish().is_err());
    }
}

#[test]
fn native_admission_and_bit_cursor_lookups_do_not_advance_execution() {
    let mut transcript = compiler_with(&[SCALAR_REQUEST]);
    let registry = transcript.admission().unwrap().0;
    let completed = registry.completed.clone();
    for _ in 0..3 {
        inner::Registry::context(registry, 1, b"borrow-test").unwrap();
        inner::Registry::record(registry, 2, b"public").unwrap();
        inner::Registry::site(registry, 7).unwrap();
        registry.preflight(SCALAR_REQUEST).unwrap();
    }
    assert_eq!(completed.get(), 0);
    NativeStrongTranscript::scalar(
        &mut transcript,
        &InnerDraw {
            site: 7,
            result_kind: 4,
        },
        Operation::RawScalar,
    )
    .unwrap();
    assert_eq!(completed.get(), 1);
    NativeStrongTranscript::finish(&mut transcript).unwrap();
}

#[test]
fn native_publication_rejection_and_append_failure_are_sticky() {
    let mut transcript = compiler_with(&[PUBLIC_REQUEST]);
    let before = transcript.len();
    let registry = transcript.admission().unwrap().0;
    let completed = registry.completed.clone();
    registry.reject_publication.set(true);
    assert!(NativeStrongTranscript::public(&mut transcript, &2, 1, b"public").is_err());
    assert_eq!(transcript.len(), before + 12 + 6);
    assert_eq!(completed.get(), 0);
    assert!(NativeStrongTranscript::public(&mut transcript, &2, 1, b"public").is_err());
    assert!(NativeStrongTranscript::finish(&mut transcript).is_err());
    assert!(transcript.finish().is_err());

    let mut transcript = compiler_with(&[InnerNativeRequest::Draw {
        site: 7,
        result_kind: 4,
        operation: Operation::RawVector,
        count: 32,
    }]);
    let completed = transcript.admission().unwrap().0.completed.clone();
    let before = transcript.len();
    // Sampling succeeds within the fixture budget, but the complete 1041-byte
    // result exceeds the installed 1024-byte record cap: no completion callback.
    assert!(NativeStrongTranscript::vector(
        &mut transcript,
        &InnerDraw {
            site: 7,
            result_kind: 4
        },
        Operation::RawVector,
        32,
    )
    .is_err());
    assert_eq!(transcript.len(), before);
    assert_eq!(completed.get(), 0);
    assert!(transcript.finish().is_err());
}

#[test]
fn native_premature_finish_poison_cannot_be_repaired_by_later_requests() {
    let mut transcript = compiler_with(&[PUBLIC_REQUEST, PROOF_REQUEST]);
    NativeStrongTranscript::public(&mut transcript, &2, 1, b"public").unwrap();
    assert!(NativeStrongTranscript::finish(&mut transcript).is_err());
    assert!(NativeStrongTranscript::proof(&mut transcript, &3, 1, b"proof").is_err());
    assert!(transcript.finish().is_err());
    let mut raw = compiler_with(&[PUBLIC_REQUEST]);
    // Raw compiler closure deliberately makes no native acceptance claim.
    raw.finish().unwrap();
}

#[test]
fn native_draw_hooks_preserve_original_sampler_and_result_bytes() {
    for (operation, count) in [
        (Operation::RawScalar, 1),
        (Operation::ScalarCoefficient, 1),
        (Operation::RawVector, 0),
        (Operation::RawVector, 3),
        (Operation::CoefficientVector, 0),
        (Operation::CoefficientVector, 3),
    ] {
        let mut registered = compiler_with(&[InnerNativeRequest::Draw {
            site: 7,
            result_kind: 4,
            operation,
            count,
        }]);
        let mut raw = compiler_with(&[]);
        let draw = InnerDraw {
            site: 7,
            result_kind: 4,
        };
        let actual = if matches!(
            operation,
            Operation::RawScalar | Operation::ScalarCoefficient
        ) {
            vec![NativeStrongTranscript::scalar(&mut registered, &draw, operation).unwrap()]
        } else {
            NativeStrongTranscript::vector(&mut registered, &draw, operation, count).unwrap()
        };
        let expected = raw
            .sample(|snapshot| {
                (0..count)
                    .map(|_| crate::native::inner_scalar(snapshot, 7))
                    .collect::<Result<Vec<_>, _>>()
            })
            .unwrap();
        assert_eq!(actual, expected);
        let mut bytes = 7u64.to_le_bytes().to_vec();
        bytes.push(operation as u8);
        bytes.extend_from_slice(&(count as u64).to_le_bytes());
        for value in &expected {
            bytes.extend_from_slice(&crate::native::encode_e(value));
        }
        raw.append(4, &bytes).unwrap();
        assert_eq!(registered.len(), raw.len());
        assert_eq!(
            registered
                .sample(|snapshot| crate::native::inner_scalar(snapshot, 7))
                .unwrap(),
            raw.sample(|snapshot| crate::native::inner_scalar(snapshot, 7))
                .unwrap(),
        );
        NativeStrongTranscript::finish(&mut registered).unwrap();
    }
    let mut registered = compiler_with(&[InnerNativeRequest::ShellBundle {
        streams: &[7, 8],
        result_kind: 6,
        count: 2,
    }]);
    let mut raw = compiler_with(&[]);
    let actual = NativeStrongTranscript::shell_bundle(
        &mut registered,
        &InnerShellDraw {
            streams: vec![7, 8],
            result_kind: 6,
        },
        2,
    )
    .unwrap();
    let expected = raw
        .sample(|snapshot| {
            [7, 8]
                .into_iter()
                .map(|stream| crate::native::inner_shell(snapshot, stream))
                .collect::<Result<Vec<_>, _>>()
        })
        .unwrap();
    assert_eq!(actual, expected);
    let mut bytes = 2u64.to_le_bytes().to_vec();
    for (stream, shell) in [7u64, 8].iter().zip(&expected) {
        bytes.extend_from_slice(&stream.to_le_bytes());
        bytes.extend(shell.coefficients().iter().map(|&c| c as u8));
    }
    raw.append(6, &bytes).unwrap();
    assert_eq!(registered.len(), raw.len());
    assert_eq!(
        registered
            .sample(|snapshot| crate::native::inner_scalar(snapshot, 7))
            .unwrap(),
        raw.sample(|snapshot| crate::native::inner_scalar(snapshot, 7))
            .unwrap(),
    );
    NativeStrongTranscript::finish(&mut registered).unwrap();
}
