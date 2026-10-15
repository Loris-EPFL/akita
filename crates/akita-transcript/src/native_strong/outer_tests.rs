//! Finite component protocol only: public, scalar, handoff, proof, vector, shell.
//! Anchor: complete raw/native prefix bytes, outputs and charged sampler work.
use super::*;
use crate::native::{shell, wire};
use core::cell::Cell;
use jolt_transcript::d1::frontend::{self, Boundary, FrontendBinding, OuterBinding};
use jolt_transcript::{
    CodecKind, DrawSite, EmissionBinding, InstalledCodec, MessageDefinition, MessageKind,
    MessageSite, ScopeEntry, Transcript as FrontendTranscript, TranscriptError,
};

const CONTEXT: &[u8] = b"finite-outer-native-acceptance-test";
const INSTANCE: &[u8] = b"complete-native-instance";
const PUBLIC: OuterRecord<'static> = OuterRecord {
    site: outer::Site(&[outer::Component::Role("public")]),
    codec: "public/v1",
};
const PROOF: OuterRecord<'static> = OuterRecord {
    site: outer::Site(&[outer::Component::Role("proof")]),
    codec: "proof/v1",
};
const SCALAR: outer::Stage<'static> = outer::Stage {
    site: outer::Site(&[outer::Component::Role("scalar")]),
    mode: outer::TreeMode::FlatBundle,
    arities: &[],
    family: b"finite-E2-shell",
    sampler: b"AE=2;LS=64;fixed-slots",
    carries: &[],
    result_type: "scalar-result/v1",
};
const VECTOR: outer::Stage<'static> = outer::Stage {
    site: outer::Site(&[outer::Component::Role("vector")]),
    result_type: "vector-result/v1",
    ..SCALAR
};
const SHELL: outer::Stage<'static> = outer::Stage {
    site: outer::Site(&[outer::Component::Role("shell")]),
    arities: &[2, 2],
    result_type: "shell-bundle-result/v1",
    ..SCALAR
};
const EMPTY_SHELL: outer::Stage<'static> = outer::Stage {
    arities: &[],
    ..SHELL
};
const HANDOFF: outer::Site<'static> =
    outer::Site(&[outer::Component::Role("frontend/to-native-pcs")]);
const EVENTS: u64 = 6;

fn limits() -> outer::Limits {
    outer::Limits {
        wire: wire::Limits {
            bytes: 16_384,
            items: 128,
            depth: 16,
        },
        context_bytes: 1024,
        prefix_bytes: 32_768,
        event_bytes: 16_384,
        proof_bytes: 1024,
        events: EVENTS,
        sites: 3,
        coordinates: 2,
        limb_attempts: 2,
        shell_attempts: 64,
        oracle_queries: 4,
        raw_bits: 32_768,
    }
}
fn require(condition: bool) -> Result<(), Error> {
    if condition {
        Ok(())
    } else {
        Err(Error::Registry)
    }
}
fn take<'a>(rest: &mut &'a [u8], n: usize) -> Result<&'a [u8], Error> {
    let value = rest.get(..n).ok_or(Error::NonCanonical)?;
    *rest = rest.get(n..).ok_or(Error::NonCanonical)?;
    Ok(value)
}
fn integer(rest: &mut &[u8]) -> Result<u64, Error> {
    Ok(u64::from_le_bytes(
        take(rest, 8)?.try_into().map_err(|_| Error::NonCanonical)?,
    ))
}
fn bytes<'a>(rest: &mut &'a [u8]) -> Result<&'a [u8], Error> {
    let n = usize::try_from(integer(rest)?).map_err(|_| Error::Overflow)?;
    take(rest, n)
}
fn list(encoded: &[u8], count: u64) -> Result<Vec<&[u8]>, Error> {
    let mut rest = encoded;
    require(integer(&mut rest)? == count)?;
    let mut values = Vec::new();
    for _ in 0..count {
        values.push(bytes(&mut rest)?);
    }
    require(rest.is_empty())?;
    Ok(values)
}
fn descriptor(
    stage: &outer::Stage<'_>,
    operation: Operation,
    count: u64,
    coordinate: Option<u64>,
) -> Vec<u8> {
    outer::Request {
        stage,
        operation,
        count,
        coordinate,
    }
    .encode(limits().wire)
    .unwrap()
}

/// No progress counter: admission is repeatable and only compiler-owned history
/// proves completion. This registry certifies exactly this six-event fixture.
struct Registry {
    reject_accepting: Cell<bool>,
    accepting_calls: Cell<usize>,
    empty_shell: bool,
}
impl outer::Registry for Registry {
    fn context(&self, context: &[u8], caps: &outer::Limits) -> Result<(), Error> {
        require(context == CONTEXT && caps.limb_attempts == 2 && caps.shell_attempts == 64)
    }
    fn record(&self, record: &outer::Record<'_>, preceding_events: u64) -> Result<(), Error> {
        let shell = self.shell_stage();
        let (kind, site, codec, count) = match preceding_events {
            0 => (outer::Kind::Public, PUBLIC.site, PUBLIC.codec, 6),
            1 => (outer::Kind::Challenge, SCALAR.site, SCALAR.result_type, 1),
            2 => (outer::Kind::Transition, HANDOFF, "handoff/v1", 1),
            3 => (outer::Kind::Proof, PROOF.site, PROOF.codec, 5),
            4 => (outer::Kind::Challenge, VECTOR.site, VECTOR.result_type, 2),
            5 => (
                outer::Kind::Challenge,
                shell.site,
                shell.result_type,
                shell.arities.len() as u64,
            ),
            _ => return Err(Error::Registry),
        };
        require(
            record.kind == kind
                && record.site == site.encode(limits().wire)?
                && record.codec == codec
                && record.count == count,
        )?;
        match preceding_events {
            0 => require(record.payload == b"public"),
            2 => require(
                record.payload
                    == wire::list(
                        &[b"jolt-frontend", b"ordinary-native-pcs", INSTANCE],
                        limits().wire,
                    )?,
            ),
            3 => require(record.payload == b"proof"),
            1 | 4 | 5 => {
                let payload = list(record.payload, 2)?;
                match preceding_events {
                    1 => {
                        require(payload[0] == descriptor(&SCALAR, Operation::RawScalar, 1, None))?;
                        decode_e(payload[1]).map(|_| ())
                    }
                    4 => {
                        require(
                            payload[0]
                                == descriptor(&VECTOR, Operation::CoefficientVector, 2, None),
                        )?;
                        for value in list(payload[1], 2)? {
                            decode_e(value)?;
                        }
                        Ok(())
                    }
                    _ => {
                        if self.empty_shell {
                            let descriptors = list(payload[0], 1)?;
                            require(
                                descriptors[0]
                                    == descriptor(shell, Operation::ShellBundle, 0, None),
                            )?;
                            list(payload[1], 0)?;
                            return Ok(());
                        }
                        let descriptors = list(payload[0], 2)?;
                        for (coordinate, encoded) in descriptors.into_iter().enumerate() {
                            require(
                                encoded
                                    == descriptor(
                                        &SHELL,
                                        Operation::ShellCoordinate,
                                        1,
                                        Some(coordinate as u64),
                                    ),
                            )?;
                        }
                        for value in list(payload[1], 2)? {
                            shell::Shell::decode(value)?;
                        }
                        Ok(())
                    }
                }
            }
            _ => Err(Error::Registry),
        }
    }
    fn request(&self, request: &outer::Request<'_, '_>, caps: &outer::Limits) -> Result<(), Error> {
        require(caps.limb_attempts == 2 && caps.shell_attempts == 64)?;
        let expected = match (request.operation, request.count, request.coordinate) {
            (Operation::RawScalar, 1, None) => &SCALAR,
            (Operation::CoefficientVector, 2, None) => &VECTOR,
            (Operation::ShellBundle, n, None) if n == self.shell_stage().arities.len() as u64 => {
                self.shell_stage()
            }
            (Operation::ShellCoordinate, 1, Some(0 | 1)) if !self.empty_shell => &SHELL,
            _ => return Err(Error::Registry),
        };
        require(
            request.stage.result_type == expected.result_type
                && request.encode(caps.wire)?
                    == descriptor(
                        expected,
                        request.operation,
                        request.count,
                        request.coordinate,
                    ),
        )
    }
    fn handoff(&self, instance: &[u8]) -> Result<(), Error> {
        require(instance == INSTANCE)
    }
}
impl Registry {
    fn shell_stage(&self) -> &'static outer::Stage<'static> {
        if self.empty_shell {
            &EMPTY_SHELL
        } else {
            &SHELL
        }
    }
    fn prefix(&self, completed_events: u64, mut prefix: &[u8]) -> Result<(), Error> {
        for ordinal in 0..completed_events {
            let kind = match take(&mut prefix, 1)? {
                [1] => outer::Kind::Public,
                [2] => outer::Kind::Proof,
                [3] => outer::Kind::Challenge,
                [4] => outer::Kind::Transition,
                _ => return Err(Error::Registry),
            };
            let site = bytes(&mut prefix)?;
            let codec =
                core::str::from_utf8(bytes(&mut prefix)?).map_err(|_| Error::NonCanonical)?;
            let count = integer(&mut prefix)?;
            let payload = bytes(&mut prefix)?;
            outer::Registry::record(
                self,
                &outer::Record {
                    kind,
                    site,
                    codec,
                    count,
                    payload,
                },
                ordinal,
            )?;
        }
        require(prefix.is_empty())
    }
}
impl OuterExecutionRegistry for Registry {
    fn accepting(&self, completed_events: u64, prefix: &[u8]) -> Result<(), Error> {
        self.accepting_calls.set(self.accepting_calls.get() + 1);
        require(!self.reject_accepting.get() && completed_events == EVENTS)?;
        self.prefix(completed_events, prefix)
    }
}
fn compiler_with(caps: outer::Limits) -> outer::Transcript<Registry> {
    compiler_with_shell(caps, false)
}
fn compiler_with_shell(caps: outer::Limits, empty_shell: bool) -> outer::Transcript<Registry> {
    outer::Transcript::new(
        Registry {
            reject_accepting: Cell::new(false),
            accepting_calls: Cell::new(0),
            empty_shell,
        },
        caps,
        CONTEXT.to_vec(),
    )
    .unwrap()
}
fn compiler() -> outer::Transcript<Registry> {
    compiler_with(limits())
}
fn raw_step(compiler: &mut outer::Transcript<Registry>, event: u64) {
    match event {
        0 => compiler
            .public(PUBLIC.site, PUBLIC.codec, 6, b"public")
            .unwrap(),
        1 => {
            compiler.scalar(&SCALAR, Operation::RawScalar).unwrap();
        }
        2 => compiler.handoff(INSTANCE, "handoff/v1").unwrap(),
        3 => compiler
            .proof(PROOF.site, PROOF.codec, 5, b"proof")
            .unwrap(),
        4 => {
            compiler
                .vector(&VECTOR, Operation::CoefficientVector, 2)
                .unwrap();
        }
        5 => {
            compiler.shell_bundle(&SHELL, 2).unwrap();
        }
        _ => panic!("invalid fixture event"),
    }
}

#[test]
fn every_proper_prefix_refuses_native_closure_and_poison_is_sticky() {
    for completed in 0..EVENTS {
        let mut transcript = compiler();
        for event in 0..completed {
            raw_step(&mut transcript, event);
        }
        assert_eq!(transcript.completed_events(), completed);
        assert!(NativeStrongTranscript::finish(&mut transcript).is_err());
        assert!(matches!(transcript.admission(), Err(Error::Failed)));
        assert_eq!(transcript.finish(), Err(Error::Failed));
        assert!(NativeStrongTranscript::public(&mut transcript, &PUBLIC, 6, b"public").is_err());
    }
}

#[test]
fn raw_component_closure_does_not_claim_native_acceptance() {
    let mut transcript = compiler();
    raw_step(&mut transcript, 0);
    assert_eq!(transcript.admission().unwrap().0.accepting_calls.get(), 0);
    transcript.finish().unwrap();
    assert!(NativeStrongTranscript::finish(&mut transcript).is_err());
}

#[test]
fn prebinding_and_repeated_admission_do_not_execute_the_protocol() {
    let mut transcript = compiler();
    let registry = transcript.admission().unwrap().0;
    let site = PUBLIC.site.encode(limits().wire).unwrap();
    for _ in 0..EVENTS {
        outer::Registry::record(
            registry,
            &outer::Record {
                kind: outer::Kind::Public,
                site: &site,
                codec: PUBLIC.codec,
                count: 6,
                payload: b"public",
            },
            0,
        )
        .unwrap();
        outer::Registry::request(
            registry,
            &outer::Request {
                stage: &SCALAR,
                operation: Operation::RawScalar,
                count: 1,
                coordinate: None,
            },
            &limits(),
        )
        .unwrap();
    }
    assert_eq!(transcript.completed_events(), 0);
    assert!(transcript.prefix_bytes().is_empty());
    assert_eq!(transcript.work(), (0, 0));
    assert!(NativeStrongTranscript::finish(&mut transcript).is_err());
}

#[test]
fn admitted_but_failed_append_never_publishes_or_accepts() {
    let mut caps = limits();
    caps.events = 1;
    let mut transcript = compiler_with(caps);
    raw_step(&mut transcript, 0);
    let before = transcript.prefix_bytes().to_vec();
    assert!(
        NativeStrongTranscript::scalar(&mut transcript, &SCALAR, Operation::RawScalar).is_err()
    );
    assert_eq!(transcript.completed_events(), 1);
    assert_eq!(transcript.prefix_bytes(), before);
    assert!(NativeStrongTranscript::finish(&mut transcript).is_err());
    assert_eq!(transcript.finish(), Err(Error::Failed));
}

#[test]
fn explicit_accepting_failure_poisoning_preserves_complete_history() {
    let mut transcript = compiler();
    for event in 0..EVENTS {
        raw_step(&mut transcript, event);
    }
    transcript.admission().unwrap().0.reject_accepting.set(true);
    let before = transcript.prefix_bytes().to_vec();
    assert!(NativeStrongTranscript::finish(&mut transcript).is_err());
    assert_eq!(transcript.completed_events(), EVENTS);
    assert_eq!(transcript.prefix_bytes(), before);
    assert_eq!(transcript.finish(), Err(Error::Failed));
}

struct Binding {
    completed: u64,
}
impl EmissionBinding for Binding {
    fn message(&self, definition: &MessageDefinition<'_>) -> Result<(), TranscriptError> {
        require(
            definition.occurrence.encode(limits().wire)? == PUBLIC.site.encode(limits().wire)?
                && definition.kind == MessageKind::Public
                && definition.codec.name == PUBLIC.codec
                && definition.codec.kind == CodecKind::Bytes
                && definition.count == 6,
        )?;
        Ok(())
    }
    fn draw(
        &self,
        stage: &outer::Stage<'_>,
        operation: Operation,
        count: u64,
        powers: u64,
    ) -> Result<(), TranscriptError> {
        require(
            operation == Operation::RawScalar
                && count == 1
                && powers == 0
                && stage.result_type == SCALAR.result_type
                && descriptor(stage, operation, count, None)
                    == descriptor(&SCALAR, operation, count, None),
        )?;
        Ok(())
    }
    fn compound(&self, _: outer::Site<'_>, _: &[ScopeEntry<'_>]) -> Result<(), TranscriptError> {
        Err(TranscriptError::Compound)
    }
}
impl FrontendBinding for Binding {
    fn message_complete(&mut self, site: &MessageSite<'_>) -> Result<(), TranscriptError> {
        self.message(site.definition())?;
        require(self.completed == 0)?;
        self.completed = 1;
        Ok(())
    }
    fn draw_complete(&mut self, site: &DrawSite<'_>) -> Result<(), TranscriptError> {
        self.draw(
            site.stage(),
            site.operation(),
            site.count(),
            site.powers_limit(),
        )?;
        require(self.completed == 1)?;
        self.completed = 2;
        Ok(())
    }
}
impl OuterBinding<Registry> for Binding {
    fn installation(
        &self,
        registry: &Registry,
        caps: &outer::Limits,
        context: &[u8],
        frontend: &frontend::Limits,
    ) -> Result<(), TranscriptError> {
        outer::Registry::context(registry, context, caps)?;
        require(
            frontend.payload_bytes == 16_384 && frontend.coordinates == 2 && frontend.powers == 2,
        )?;
        Ok(())
    }
    fn accepting(
        &self,
        compiler: &outer::Transcript<Registry>,
        boundary: Boundary,
    ) -> Result<(), TranscriptError> {
        require(
            boundary == Boundary::Handoff
                && self.completed == 2
                && compiler.completed_events() == 2,
        )?;
        compiler.admission()?.0.prefix(2, compiler.prefix_bytes())?;
        Ok(())
    }
    fn handoff_codec(&self, instance: &[u8]) -> Result<&str, TranscriptError> {
        require(instance == INSTANCE)?;
        Ok("handoff/v1")
    }
}
fn frontend_limits() -> frontend::Limits {
    frontend::Limits {
        payload_bytes: 16_384,
        coordinates: 2,
        powers: 2,
    }
}
fn frontend_sites(binding: &Binding) -> (MessageSite<'static>, DrawSite<'static>) {
    (
        MessageSite::bind(
            binding,
            MessageDefinition {
                occurrence: PUBLIC.site,
                kind: MessageKind::Public,
                codec: InstalledCodec {
                    name: PUBLIC.codec,
                    kind: CodecKind::Bytes,
                },
                count: 6,
            },
        )
        .unwrap(),
        DrawSite::bind(binding, &SCALAR, Operation::RawScalar, 1, 0).unwrap(),
    )
}

#[test]
fn full_scoped_frontend_and_native_closure_preserve_raw_bytes_and_work() {
    let mut native = compiler();
    let mut raw = compiler();
    let binding = Binding { completed: 0 };
    let (public, draw) = frontend_sites(&binding);
    // Prebinding is not execution, and the frontend returns the full instance.
    assert_eq!(native.completed_events(), 0);
    let scalar = NativeTranscriptBorrow::new(&mut native)
        .with_frontend::<E, _, _, _, _>(
            binding,
            frontend_limits(),
            |frontend| -> Result<_, TranscriptError> {
                frontend.append_bytes(&public, b"public")?;
                Ok((frontend.challenge(&draw)?, INSTANCE))
            },
        )
        .unwrap();
    raw_step(&mut raw, 0);
    assert_eq!(scalar, raw.scalar(&SCALAR, Operation::RawScalar).unwrap());
    raw_step(&mut raw, 2);
    NativeStrongTranscript::proof(&mut native, &PROOF, 5, b"proof").unwrap();
    raw_step(&mut raw, 3);
    assert_eq!(
        NativeStrongTranscript::vector(&mut native, &VECTOR, Operation::CoefficientVector, 2)
            .unwrap(),
        raw.vector(&VECTOR, Operation::CoefficientVector, 2)
            .unwrap()
    );
    let native_shells = NativeStrongTranscript::shell_bundle(&mut native, &SHELL, 2).unwrap();
    let raw_shells = raw.shell_bundle(&SHELL, 2).unwrap();
    for (native, raw) in native_shells.iter().zip(&raw_shells) {
        assert_eq!(native.coefficients(), raw.coefficients());
    }
    assert_eq!(native.context_bytes(), raw.context_bytes());
    assert_eq!(native.prefix_bytes(), raw.prefix_bytes());
    assert_eq!(native.work(), raw.work());
    assert_eq!(native.work(), (4, 3 * 2 * 256 + 2 * 64 * 187));
    assert_eq!(native.completed_events(), EVENTS);
    NativeStrongTranscript::finish(&mut native).unwrap();
    raw.finish().unwrap();
    assert!(matches!(native.admission(), Err(Error::Closed)));
    assert_eq!(native.finish(), Err(Error::Closed));
}

#[test]
fn incomplete_scoped_frontend_poisoning_cannot_be_closed_natively() {
    let mut native = compiler();
    let binding = Binding { completed: 0 };
    let (public, _) = frontend_sites(&binding);
    let result = NativeTranscriptBorrow::new(&mut native).with_frontend::<E, _, _, _, _>(
        binding,
        frontend_limits(),
        |frontend| -> Result<_, TranscriptError> {
            frontend.append_bytes(&public, b"public")?;
            Ok(((), INSTANCE))
        },
    );
    assert!(result.is_err());
    assert_eq!(native.completed_events(), 1);
    assert!(NativeStrongTranscript::finish(&mut native).is_err());
    assert_eq!(native.finish(), Err(Error::Failed));
}

fn empty_shell_native_prefix() -> outer::Transcript<Registry> {
    let mut native = compiler_with_shell(limits(), true);
    let binding = Binding { completed: 0 };
    let (public, draw) = frontend_sites(&binding);
    NativeTranscriptBorrow::new(&mut native)
        .with_frontend::<E, _, _, _, _>(
            binding,
            frontend_limits(),
            |frontend| -> Result<_, TranscriptError> {
                frontend.append_bytes(&public, b"public")?;
                frontend.challenge(&draw)?;
                Ok(((), INSTANCE))
            },
        )
        .unwrap();
    NativeStrongTranscript::proof(&mut native, &PROOF, 5, b"proof").unwrap();
    NativeStrongTranscript::vector(&mut native, &VECTOR, Operation::CoefficientVector, 2).unwrap();
    native
}

#[test]
fn empty_shell_native_completion_is_bound_once_and_requires_the_full_schedule() {
    for completed in 0..EVENTS {
        let mut native = compiler_with_shell(limits(), true);
        for event in 0..completed {
            raw_step(&mut native, event);
        }
        assert!(NativeStrongTranscript::finish(&mut native).is_err());
        assert_eq!(native.finish(), Err(Error::Failed));
    }

    let mut native = empty_shell_native_prefix();
    let before_work = native.work();
    assert!(
        NativeStrongTranscript::shell_bundle(&mut native, &EMPTY_SHELL, 0)
            .unwrap()
            .is_empty()
    );
    assert_eq!(native.work(), before_work);
    assert_eq!(native.completed_events(), EVENTS);
    NativeStrongTranscript::finish(&mut native).unwrap();
    assert!(matches!(native.admission(), Err(Error::Closed)));

    let mut repeated = empty_shell_native_prefix();
    NativeStrongTranscript::shell_bundle(&mut repeated, &EMPTY_SHELL, 0).unwrap();
    let prefix = repeated.prefix_bytes().to_vec();
    assert!(NativeStrongTranscript::shell_bundle(&mut repeated, &EMPTY_SHELL, 0).is_err());
    assert_eq!(repeated.prefix_bytes(), prefix);
    assert_eq!(repeated.completed_events(), EVENTS);
    assert!(NativeStrongTranscript::finish(&mut repeated).is_err());
    assert_eq!(repeated.finish(), Err(Error::Failed));
}

#[test]
fn empty_shell_metadata_rejection_poisons_the_retained_native_compiler() {
    for (stage, count) in [
        (
            outer::Stage {
                family: b"unregistered-family",
                ..EMPTY_SHELL
            },
            0,
        ),
        (
            outer::Stage {
                sampler: b"unregistered-sampler",
                ..EMPTY_SHELL
            },
            0,
        ),
        (
            outer::Stage {
                mode: outer::TreeMode::Nested,
                ..EMPTY_SHELL
            },
            0,
        ),
        (
            outer::Stage {
                arities: &[2],
                ..EMPTY_SHELL
            },
            0,
        ),
        (EMPTY_SHELL, 1),
    ] {
        let mut native = empty_shell_native_prefix();
        let prefix = native.prefix_bytes().to_vec();
        let work = native.work();
        assert!(NativeStrongTranscript::shell_bundle(&mut native, &stage, count).is_err());
        assert_eq!(native.completed_events(), EVENTS - 1);
        assert_eq!(native.prefix_bytes(), prefix);
        assert_eq!(native.work(), work);
        assert!(matches!(native.admission(), Err(Error::Failed)));
        assert!(NativeStrongTranscript::shell_bundle(&mut native, &EMPTY_SHELL, 0).is_err());
        assert!(NativeStrongTranscript::finish(&mut native).is_err());
        assert_eq!(native.finish(), Err(Error::Failed));
    }
}
