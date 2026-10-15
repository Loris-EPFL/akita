use super::*;
use crate::{FoldChallengeDrawDomain, ShellGroupDescriptor, SparseChallengeConfig};
use akita_transcript::{new_prover_channel, new_verifier_channel, FoldPreview, IndexedLimits,
    InstalledContext, InstalledEvent, InstalledRegistry, RequestSpec};

fn descriptor() -> ShellLevelDescriptor {
    ShellLevelDescriptor::new(vec![
        ShellGroupDescriptor::new(FoldChallengeDrawDomain::EvaluationTrace, 3, 0, 2, 2,
            SparseChallengeConfig::pm1_only(1), None).unwrap(),
        ShellGroupDescriptor::new(FoldChallengeDrawDomain::SubringCoefficientPacking {
            challenge_subring_dimension: 4 }, 4, 1, 1, 2,
            SparseChallengeConfig { count_pm1: 1, count_pm2: 1 }, None).unwrap()
            .with_native_embedding(8, 2).unwrap(),
    ]).unwrap()
}

fn context(descriptor: &ShellLevelDescriptor, levels: &[u32]) -> InstalledContext {
    let events = levels.iter().enumerate().map(|(ordinal, &level)| {
        InstalledEvent::Request(RequestSpec {
            request: descriptor.indexed_request(ordinal as u64, level).unwrap(),
            result_kind: CompletedResultKind::ShellLevel,
            result_len: descriptor.result_len(),
        })
    }).collect();
    let registry = InstalledRegistry::new(123, events, IndexedLimits {
        max_context_bytes: 4096, max_prefix_bytes: 1 << 20, max_key_bytes: 1 << 20,
        max_descriptor_bytes: 1 << 16, max_answer_bytes: 1 << 20,
        max_result_bytes: 1 << 16, max_proof_bytes: 4096,
    }).unwrap();
    InstalledContext::new(&registry, b"atomic-shell-test", b"full-augmented-test-instance").unwrap()
}

#[test]
fn preview_live_and_verifier_share_atomic_family_and_descendant_history() {
    let descriptor = descriptor();
    let context = context(&descriptor, &[2, 3]);
    let mut prover = new_prover_channel(&context).unwrap();
    let mut preview = FoldPreview::snapshot(&prover).unwrap();
    let preview_first = PreviewFoldDraw::new(&mut preview, 2).draw_shell_level(&descriptor).unwrap();
    let preview_second = PreviewFoldDraw::new(&mut preview, 3).draw_shell_level(&descriptor).unwrap();
    let live_first = ProverFoldDraw::new(&mut prover, 2).draw_shell_level(&descriptor).unwrap();
    let live_second = ProverFoldDraw::new(&mut prover, 3).draw_shell_level(&descriptor).unwrap();
    assert_eq!(preview_first, live_first);
    assert_eq!(preview_second, live_second);
    assert_eq!(live_first.groups().len(), 2);
    assert_eq!(live_first.group(0).unwrap().challenges().len(), 4);
    assert_eq!(live_first.group(1).unwrap().challenges().len(), 2);
    let proof = prover.finish().unwrap();
    assert!(proof.is_empty()); // Shell completions own history, not proof transport.
    let mut verifier = new_verifier_channel(&context, &proof).unwrap();
    let verified_first = VerifierFoldDraw::new(&mut verifier, 2).draw_shell_level(&descriptor).unwrap();
    let verified_second = VerifierFoldDraw::new(&mut verifier, 3).draw_shell_level(&descriptor).unwrap();
    assert_eq!(verified_first, live_first);
    assert_eq!(verified_second, live_second);
    verifier.finish().unwrap();
}

#[test]
fn group_prefix_or_changed_embedding_poison_every_channel_role() {
    let whole = descriptor();
    let context = context(&whole, &[2]);
    let prefix = ShellLevelDescriptor::new(vec![whole.groups()[0].clone()]).unwrap();
    let changed = ShellLevelDescriptor::new(vec![whole.groups()[0].clone(),
        whole.groups()[1].clone().with_native_embedding(16, 4).unwrap()]).unwrap();
    let mut prover = new_prover_channel(&context).unwrap();
    let mut preview = FoldPreview::snapshot(&prover).unwrap();
    assert!(PreviewFoldDraw::new(&mut preview, 2).draw_shell_level(&prefix).is_err());
    assert!(IndexedChannel::status(&preview).is_err());
    assert!(PreviewFoldDraw::new(&mut preview, 2).draw_shell_level(&whole).is_err());
    assert!(ProverFoldDraw::new(&mut prover, 2).draw_shell_level(&changed).is_err());
    assert!(prover.status().is_err());
    assert!(ProverFoldDraw::new(&mut prover, 2).draw_shell_level(&whole).is_err());
    let mut verifier = new_verifier_channel(&context, &[]).unwrap();
    assert!(VerifierFoldDraw::new(&mut verifier, 2).draw_shell_level(&prefix).is_err());
    assert!(verifier.status().is_err());
    assert!(verifier.finish().is_err());
}

#[test]
fn wrong_level_or_completion_length_cannot_advance_installed_grammar() {
    let descriptor = descriptor();
    let context = context(&descriptor, &[2]);
    let mut prover = new_prover_channel(&context).unwrap();
    assert!(ProverFoldDraw::new(&mut prover, 3).draw_shell_level(&descriptor).is_err());
    assert!(prover.finish().is_err());

    let request = descriptor.indexed_request(0, 2).unwrap();
    let registry = InstalledRegistry::new(124, vec![InstalledEvent::Request(RequestSpec {
        request, result_kind: CompletedResultKind::ShellLevel,
        result_len: descriptor.result_len() - 1,
    })], IndexedLimits {
        max_context_bytes: 4096, max_prefix_bytes: 1 << 20, max_key_bytes: 1 << 20,
        max_descriptor_bytes: 1 << 16, max_answer_bytes: 1 << 20,
        max_result_bytes: 1 << 16, max_proof_bytes: 4096,
    }).unwrap();
    let context = InstalledContext::new(&registry, b"test", b"full-context").unwrap();
    let mut prover = new_prover_channel(&context).unwrap();
    assert!(ProverFoldDraw::new(&mut prover, 2).draw_shell_level(&descriptor).is_err());
    assert!(prover.status().is_err());
    assert!(prover.finish().is_err());
}
